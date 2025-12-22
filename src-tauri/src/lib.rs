use parking_lot::Mutex;
use serde::{Deserialize, Serialize};
use serialport::{DataBits, FlowControl, Parity, SerialPort, StopBits};
use std::collections::HashMap;
use std::io::{Read, Write};
use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};
use tauri::{AppHandle, Emitter, Manager, State};

// Struct để lưu thông tin cổng serial
#[derive(Debug, Serialize, Clone)]
pub struct PortInfo {
    pub name: String,
    pub port_type: String,
    pub manufacturer: Option<String>,
    pub product: Option<String>,
}

// Cấu hình serial port
#[derive(Debug, Deserialize, Clone)]
pub struct SerialConfig {
    pub port_name: String,
    pub baud_rate: u32,
    pub data_bits: u8,
    pub stop_bits: String,
    pub parity: String,
    pub dtr: bool,
    pub rts: bool,
}

// Dữ liệu nhận được từ serial
#[derive(Debug, Serialize, Clone)]
pub struct SerialData {
    pub port_name: String,
    pub data: Vec<u8>,
    pub timestamp: u64,
}

// Event khi port bị ngắt kết nối
#[derive(Debug, Serialize, Clone)]
pub struct PortDisconnected {
    pub port_name: String,
    pub reason: String,
    pub timestamp: u64,
}

// Quản lý trạng thái serial port
pub struct SerialState {
    ports: Mutex<HashMap<String, Arc<Mutex<Box<dyn SerialPort>>>>>,
    running: Mutex<HashMap<String, bool>>,
}

impl Default for SerialState {
    fn default() -> Self {
        Self {
            ports: Mutex::new(HashMap::new()),
            running: Mutex::new(HashMap::new()),
        }
    }
}

// Liệt kê các cổng serial có sẵn (hỗ trợ Windows, Linux, macOS)
#[tauri::command]
fn list_serial_ports() -> Result<Vec<PortInfo>, String> {
    let ports = serialport::available_ports().map_err(|e| e.to_string())?;

    let port_list: Vec<PortInfo> = ports
        .into_iter()
        .filter(|p| {
            // Chỉ lấy USB ports
            if !matches!(p.port_type, serialport::SerialPortType::UsbPort(_)) {
                return false;
            }

            let name = &p.port_name;

            // Windows: COMx (COM1, COM2, ...)
            #[cfg(target_os = "windows")]
            {
                name.starts_with("COM")
            }

            // Linux: /dev/ttyUSB*, /dev/ttyACM*, /dev/ttyS*
            #[cfg(target_os = "linux")]
            {
                name.starts_with("/dev/ttyUSB")
                    || name.starts_with("/dev/ttyACM")
                    || name.starts_with("/dev/ttyS")
            }

            // macOS: /dev/tty.* (loại trừ usbmodem)
            #[cfg(target_os = "macos")]
            {
                name.starts_with("/dev/tty.")
            }

            // Fallback cho các OS khác
            #[cfg(not(any(target_os = "windows", target_os = "linux", target_os = "macos")))]
            {
                true
            }
        })
        .map(|p| {
            let short_name = if cfg!(target_os = "windows") {
                // Windows: giữ nguyên tên (COM1, COM2, ...)
                p.port_name.clone()
            } else {
                // Unix-like: lấy phần cuối sau dấu /
                p.port_name
                    .split('/')
                    .last()
                    .unwrap_or(&p.port_name)
                    .to_string()
            };

            // Lấy thông tin USB nếu có
            let (manufacturer, product) = match &p.port_type {
                serialport::SerialPortType::UsbPort(usb_info) => {
                    let mut prod = usb_info.product.clone();
                    // Windows: loại bỏ hậu tố (COMx) từ product name
                    #[cfg(target_os = "windows")]
                    if let Some(ref p) = prod {
                        if let Some(idx) = p.rfind(" (COM") {
                            prod = Some(p[..idx].to_string());
                        }
                    }
                    (usb_info.manufacturer.clone(), prod)
                }
                _ => (None, None),
            };

            PortInfo {
                name: p.port_name,
                port_type: short_name,
                manufacturer,
                product,
            }
        })
        .collect();

    Ok(port_list)
}

// Mở kết nối serial port
#[tauri::command]
fn open_port(
    app: AppHandle,
    state: State<SerialState>,
    config: SerialConfig,
) -> Result<String, String> {
    let port_name = config.port_name.clone();

    // Kiểm tra xem port đã mở chưa
    {
        let ports = state.ports.lock();
        if ports.contains_key(&port_name) {
            return Err(format!("Port {} đã được mở", port_name));
        }
    }

    // Parse data bits
    let data_bits = match config.data_bits {
        5 => DataBits::Five,
        6 => DataBits::Six,
        7 => DataBits::Seven,
        8 => DataBits::Eight,
        _ => return Err("Data bits không hợp lệ".to_string()),
    };

    // Parse stop bits
    let stop_bits = match config.stop_bits.as_str() {
        "1" => StopBits::One,
        "1.5" => StopBits::Two, // serialport crate không hỗ trợ 1.5, dùng 2
        "2" => StopBits::Two,
        _ => return Err("Stop bits không hợp lệ".to_string()),
    };

    // Parse parity
    let parity = match config.parity.to_lowercase().as_str() {
        "none" => Parity::None,
        "odd" => Parity::Odd,
        "even" => Parity::Even,
        _ => return Err("Parity không hợp lệ".to_string()),
    };

    // Mở serial port với timeout ngắn để poll nhanh
    let mut port = serialport::new(&port_name, config.baud_rate)
        .data_bits(data_bits)
        .stop_bits(stop_bits)
        .parity(parity)
        .flow_control(FlowControl::None)
        .timeout(Duration::from_millis(5)) // Timeout ngắn để responsive
        .open()
        .map_err(|e| format!("Không thể mở port {}: {}", port_name, e))?;

    // Set DTR và RTS
    port.write_data_terminal_ready(config.dtr)
        .map_err(|e| format!("Không thể set DTR: {}", e))?;
    port.write_request_to_send(config.rts)
        .map_err(|e| format!("Không thể set RTS: {}", e))?;

    let port = Arc::new(Mutex::new(port));

    // Lưu port vào state
    {
        let mut ports = state.ports.lock();
        ports.insert(port_name.clone(), port.clone());
    }

    // Đánh dấu port đang chạy
    {
        let mut running = state.running.lock();
        running.insert(port_name.clone(), true);
    }

    // Tạo thread đọc dữ liệu
    let port_name_clone = port_name.clone();
    let app_clone = app.clone();
    let state_ptr = app.state::<SerialState>().inner() as *const SerialState as usize;
    let baud = config.baud_rate;

    thread::spawn(move || {
        let mut buffer = [0u8; 4096]; // Tăng buffer để đọc nhiều data hơn mỗi lần
        let mut disconnect_reason: Option<String> = None;

        // Tính gap timeout động dựa vào baud rate
        // Công thức: thời gian truyền 256 bytes, min 5ms, max 50ms
        // Cho phép message interval ~50ms+ được tách riêng
        let gap_timeout_ms: u64 = {
            let time_for_256b = (256 * 10 * 1000) / baud as u64; // ms để truyền 256 bytes
            time_for_256b.clamp(5, 50)
        };

        // Batching: tích lũy data và emit khi có "gap"
        let mut accumulated_data: Vec<u8> = Vec::with_capacity(8192);
        let mut last_data_received = Instant::now();
        let mut has_pending_data = false;

        loop {
            // Kiểm tra xem có nên tiếp tục đọc không
            let state = unsafe { &*(state_ptr as *const SerialState) };
            {
                let running = state.running.lock();
                if !running.get(&port_name_clone).unwrap_or(&false) {
                    break;
                }
            }

            // Đọc dữ liệu
            let bytes_read = {
                let ports = state.ports.lock();
                if let Some(port) = ports.get(&port_name_clone) {
                    let mut port = port.lock();
                    match port.read(&mut buffer) {
                        Ok(n) => Some(n),
                        Err(ref e) if e.kind() == std::io::ErrorKind::TimedOut => None,
                        Err(e) => {
                            // Lưu lý do ngắt kết nối
                            disconnect_reason = Some(format!("{}", e));
                            break;
                        }
                    }
                } else {
                    disconnect_reason = Some("Port không tồn tại".to_string());
                    break;
                }
            };

            // Tích lũy data
            if let Some(n) = bytes_read {
                if n > 0 {
                    accumulated_data.extend_from_slice(&buffer[..n]);
                    last_data_received = Instant::now();
                    has_pending_data = true;
                }
            }

            // Emit khi có gap: có data pending và không nhận thêm data trong gap_timeout_ms
            let should_emit = has_pending_data
                && last_data_received.elapsed() > Duration::from_millis(gap_timeout_ms);

            if should_emit {
                let data = SerialData {
                    port_name: port_name_clone.clone(),
                    data: accumulated_data.clone(),
                    timestamp: std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap()
                        .as_millis() as u64,
                };
                let _ = app_clone.emit("serial-data", data);
                accumulated_data.clear();
                has_pending_data = false;
            }

            // Không cần sleep vì read timeout đã cung cấp delay
        }

        // Emit data còn lại trước khi exit
        if !accumulated_data.is_empty() {
            let data = SerialData {
                port_name: port_name_clone.clone(),
                data: accumulated_data,
                timestamp: std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_millis() as u64,
            };
            let _ = app_clone.emit("serial-data", data);
        }

        // Nếu có lỗi (thiết bị bị rút), emit event thông báo
        if let Some(reason) = disconnect_reason {
            // Cleanup state
            let state = unsafe { &*(state_ptr as *const SerialState) };
            {
                let mut ports = state.ports.lock();
                ports.remove(&port_name_clone);
            }
            {
                let mut running = state.running.lock();
                running.remove(&port_name_clone);
            }

            // Emit event về frontend
            let event = PortDisconnected {
                port_name: port_name_clone.clone(),
                reason,
                timestamp: std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_millis() as u64,
            };
            let _ = app_clone.emit("serial-disconnected", event);
        }
    });

    Ok(format!("Đã mở port {} thành công", port_name))
}

// Đóng kết nối serial port
#[tauri::command]
fn close_port(state: State<SerialState>, port_name: String) -> Result<String, String> {
    // Dừng thread đọc
    {
        let mut running = state.running.lock();
        running.insert(port_name.clone(), false);
    }

    // Đợi thread dừng
    thread::sleep(Duration::from_millis(200));

    // Xóa port khỏi state
    {
        let mut ports = state.ports.lock();
        ports.remove(&port_name);
    }

    {
        let mut running = state.running.lock();
        running.remove(&port_name);
    }

    Ok(format!("Đã đóng port {}", port_name))
}

// Gửi dữ liệu qua serial port
#[tauri::command]
fn send_data(
    state: State<SerialState>,
    port_name: String,
    data: String,
    is_hex: bool,
    byte_delay_us: Option<u64>, // Inter-byte delay (microseconds), 0 = disabled
) -> Result<String, String> {
    let ports = state.ports.lock();
    let port = ports
        .get(&port_name)
        .ok_or_else(|| format!("Port {} chưa được mở", port_name))?;

    let bytes: Vec<u8> = if is_hex {
        // Parse hex string - chỉ lấy các ký tự hex hợp lệ
        let hex_chars: String = data
            .chars()
            .filter(|c| c.is_ascii_hexdigit())
            .collect();
        
        hex_chars
            .as_bytes()
            .chunks(2)
            .map(|chunk| {
                let s = std::str::from_utf8(chunk).unwrap_or("00");
                u8::from_str_radix(s, 16).unwrap_or(0)
            })
            .collect()
    } else {
        data.as_bytes().to_vec()
    };

    let mut port = port.lock();
    let total_bytes = bytes.len();
    let delay = byte_delay_us.unwrap_or(0);

    if delay > 0 {
        // Inter-byte delay mode: gửi từng byte với delay
        for byte in bytes.iter() {
            port.write_all(&[*byte])
                .map_err(|e| format!("Lỗi gửi dữ liệu: {}", e))?;
            thread::sleep(Duration::from_micros(delay));
        }
        port.flush()
            .map_err(|e| format!("Lỗi flush: {}", e))?;
    } else {
        // Chunking mode: gửi theo chunk để tránh mất ký tự
        const CHUNK_SIZE: usize = 256;

        for chunk in bytes.chunks(CHUNK_SIZE) {
            port.write_all(chunk)
                .map_err(|e| format!("Lỗi gửi dữ liệu: {}", e))?;
            port.flush()
                .map_err(|e| format!("Lỗi flush: {}", e))?;

            // Delay nhỏ giữa các chunk
            if total_bytes > CHUNK_SIZE {
                thread::sleep(Duration::from_micros(500));
            }
        }
    }

    Ok(format!("Đã gửi {} bytes", total_bytes))
}

// Kiểm tra trạng thái kết nối
#[tauri::command]
fn is_port_open(state: State<SerialState>, port_name: String) -> bool {
    let ports = state.ports.lock();
    ports.contains_key(&port_name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(SerialState::default())
        .invoke_handler(tauri::generate_handler![
            list_serial_ports,
            open_port,
            close_port,
            send_data,
            is_port_open
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
