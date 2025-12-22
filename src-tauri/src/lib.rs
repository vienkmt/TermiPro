use parking_lot::Mutex;
use serde::{Deserialize, Serialize};
use serialport::{DataBits, FlowControl, Parity, SerialPort, StopBits};
use std::collections::HashMap;
use std::io::{Read, Write};
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use tauri::{AppHandle, Emitter, Manager, State};

// Struct để lưu thông tin cổng serial
#[derive(Debug, Serialize, Clone)]
pub struct PortInfo {
    pub name: String,
    pub port_type: String,
}

// Cấu hình serial port
#[derive(Debug, Deserialize, Clone)]
pub struct SerialConfig {
    pub port_name: String,
    pub baud_rate: u32,
    pub data_bits: u8,
    pub stop_bits: String,
    pub parity: String,
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
                name.starts_with("/dev/tty.") && !name.contains("usbmodem")
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

            PortInfo {
                name: p.port_name,
                port_type: short_name,
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

    // Mở serial port
    let port = serialport::new(&port_name, config.baud_rate)
        .data_bits(data_bits)
        .stop_bits(stop_bits)
        .parity(parity)
        .flow_control(FlowControl::None)
        .timeout(Duration::from_millis(100))
        .open()
        .map_err(|e| format!("Không thể mở port {}: {}", port_name, e))?;

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

    thread::spawn(move || {
        let mut buffer = [0u8; 1024];
        let mut disconnect_reason: Option<String> = None;

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

            if let Some(n) = bytes_read {
                if n > 0 {
                    let data = SerialData {
                        port_name: port_name_clone.clone(),
                        data: buffer[..n].to_vec(),
                        timestamp: std::time::SystemTime::now()
                            .duration_since(std::time::UNIX_EPOCH)
                            .unwrap()
                            .as_millis() as u64,
                    };
                    let _ = app_clone.emit("serial-data", data);
                }
            }

            thread::sleep(Duration::from_millis(10));
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
    port.write_all(&bytes)
        .map_err(|e| format!("Lỗi gửi dữ liệu: {}", e))?;

    Ok(format!("Đã gửi {} bytes", bytes.len()))
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
