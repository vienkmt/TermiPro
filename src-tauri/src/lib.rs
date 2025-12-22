use parking_lot::Mutex;
use serde::{Deserialize, Serialize};
use serialport::{DataBits, FlowControl, Parity, SerialPort, StopBits};
use std::collections::HashMap;
use std::io::{Read, Write};
use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};
use tauri::{AppHandle, Emitter, Manager, State};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::mpsc;
use std::sync::atomic::{AtomicBool, Ordering};

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

// ===================== TCP STRUCTS =====================

// TCP Client configuration
#[derive(Debug, Deserialize, Clone)]
pub struct TcpClientConfig {
    pub host: String,
    pub port: u16,
    pub connection_id: String,
}

// TCP Server configuration
#[derive(Debug, Deserialize, Clone)]
pub struct TcpServerConfig {
    pub port: u16,
    pub bind_address: String,
    pub server_id: String,
    pub max_clients: u32,
}

// TCP Client info (for server's connected clients list)
#[derive(Debug, Serialize, Clone)]
pub struct TcpClientInfo {
    pub client_id: String,
    pub remote_addr: String,
    pub connected_at: u64,
}

// TCP Data event
#[derive(Debug, Serialize, Clone)]
pub struct TcpData {
    pub connection_id: String,
    pub client_id: Option<String>,
    pub data: Vec<u8>,
    pub timestamp: u64,
}

// TCP connection status event
#[derive(Debug, Serialize, Clone)]
pub struct TcpConnectionStatus {
    pub connection_id: String,
    pub status: String,
    pub message: Option<String>,
    pub timestamp: u64,
}

// TCP Server client event (connect/disconnect)
#[derive(Debug, Serialize, Clone)]
pub struct TcpServerClientEvent {
    pub server_id: String,
    pub client_id: String,
    pub remote_addr: String,
    pub event_type: String,
    pub timestamp: u64,
}

// TCP Client connection handle
pub struct TcpClientHandle {
    pub tx: mpsc::Sender<Vec<u8>>,
    pub running: Arc<AtomicBool>,
    pub config: TcpClientConfig,
}

// TCP Server client handle
pub struct TcpServerClientHandle {
    pub tx: mpsc::Sender<Vec<u8>>,
    pub info: TcpClientInfo,
}

// TCP Server instance
pub struct TcpServerHandle {
    pub config: TcpServerConfig,
    pub running: Arc<AtomicBool>,
    pub clients: Arc<Mutex<HashMap<String, TcpServerClientHandle>>>,
    pub echo_enabled: Arc<AtomicBool>,
}

// ===================== STATE MANAGEMENT =====================

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

// Quản lý trạng thái TCP
pub struct TcpState {
    clients: Arc<Mutex<HashMap<String, TcpClientHandle>>>,
    servers: Arc<Mutex<HashMap<String, TcpServerHandle>>>,
    runtime: tokio::runtime::Runtime,
}

impl Default for TcpState {
    fn default() -> Self {
        Self {
            clients: Arc::new(Mutex::new(HashMap::new())),
            servers: Arc::new(Mutex::new(HashMap::new())),
            runtime: tokio::runtime::Builder::new_multi_thread()
                .worker_threads(4)
                .enable_all()
                .build()
                .expect("Failed to create Tokio runtime"),
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
                    #[allow(unused_mut)]
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

// ===================== TCP CLIENT COMMANDS =====================

fn get_timestamp() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64
}

// TCP Client retry configuration
const TCP_CLIENT_MAX_RETRIES: u32 = 3;
const TCP_CLIENT_RETRY_DELAY_MS: u64 = 500;
const TCP_CLIENT_RECONNECT_DELAY_MS: u64 = 1000;
const TCP_CLIENT_MAX_RECONNECT_ATTEMPTS: u32 = 3;

// Kết nối đến TCP server với auto-reconnect
#[tauri::command]
fn tcp_client_connect(
    app: AppHandle,
    state: State<TcpState>,
    config: TcpClientConfig,
) -> Result<String, String> {
    let connection_id = config.connection_id.clone();

    // Kiểm tra xem connection đã tồn tại chưa
    {
        let clients = state.clients.lock();
        if clients.contains_key(&connection_id) {
            return Err(format!("Connection {} đã tồn tại", connection_id));
        }
    }

    let addr = format!("{}:{}", config.host, config.port);
    let addr_for_response = addr.clone();
    let app_clone = app.clone();
    let connection_id_clone = connection_id.clone();
    let running = Arc::new(AtomicBool::new(true));
    let running_clone = running.clone();

    // Clone clients Arc để có thể cleanup trong async task
    let clients_clone = state.clients.clone();
    let connection_id_for_cleanup = connection_id.clone();

    // Channel để gửi data từ frontend
    let (tx, rx) = mpsc::channel::<Vec<u8>>(100);

    // Spawn async task để handle connection với auto-reconnect
    state.runtime.spawn(async move {
        let mut reconnect_attempts = 0u32;
        // Wrap receiver trong Option để có thể move vào/ra async closures
        let mut rx_option: Option<mpsc::Receiver<Vec<u8>>> = Some(rx);

        'connection_loop: loop {
            if !running_clone.load(Ordering::Relaxed) {
                break;
            }

            // Kết nối đến server
            let stream = match TcpStream::connect(&addr).await {
                Ok(s) => {
                    reconnect_attempts = 0; // Reset counter on successful connect
                    s
                }
                Err(e) => {
                    reconnect_attempts += 1;

                    if reconnect_attempts >= TCP_CLIENT_MAX_RECONNECT_ATTEMPTS {
                        let _ = app_clone.emit("tcp-client-status", TcpConnectionStatus {
                            connection_id: connection_id_clone.clone(),
                            status: "error".to_string(),
                            message: Some(format!("Không thể kết nối sau {} lần thử: {}", reconnect_attempts, e)),
                            timestamp: get_timestamp(),
                        });
                        break 'connection_loop;
                    }

                    // Emit reconnecting status
                    let _ = app_clone.emit("tcp-client-status", TcpConnectionStatus {
                        connection_id: connection_id_clone.clone(),
                        status: "reconnecting".to_string(),
                        message: Some(format!("Đang thử kết nối lại ({}/{}): {}", reconnect_attempts, TCP_CLIENT_MAX_RECONNECT_ATTEMPTS, e)),
                        timestamp: get_timestamp(),
                    });

                    tokio::time::sleep(Duration::from_millis(TCP_CLIENT_RECONNECT_DELAY_MS)).await;
                    continue 'connection_loop;
                }
            };

            // Emit connected status
            let _ = app_clone.emit("tcp-client-status", TcpConnectionStatus {
                connection_id: connection_id_clone.clone(),
                status: "connected".to_string(),
                message: None,
                timestamp: get_timestamp(),
            });

            let (mut read_half, mut write_half) = stream.into_split();

            // Flag để báo hiệu connection bị đứt
            let connection_lost = Arc::new(AtomicBool::new(false));
            let connection_lost_read = connection_lost.clone();
            let connection_lost_write = connection_lost.clone();

            // Task đọc data
            let app_read = app_clone.clone();
            let conn_id_read = connection_id_clone.clone();
            let running_read = running_clone.clone();

            let read_task = tokio::spawn(async move {
                let mut buffer = [0u8; 4096];
                let mut accumulated_data: Vec<u8> = Vec::with_capacity(8192);

                loop {
                    if !running_read.load(Ordering::Relaxed) || connection_lost_read.load(Ordering::Relaxed) {
                        break;
                    }

                    tokio::select! {
                        result = read_half.read(&mut buffer) => {
                            match result {
                                Ok(0) => {
                                    // Connection closed by server
                                    connection_lost_read.store(true, Ordering::Relaxed);
                                    break;
                                }
                                Ok(n) => {
                                    accumulated_data.extend_from_slice(&buffer[..n]);

                                    if !accumulated_data.is_empty() {
                                        let _ = app_read.emit("tcp-data", TcpData {
                                            connection_id: conn_id_read.clone(),
                                            client_id: None,
                                            data: accumulated_data.clone(),
                                            timestamp: get_timestamp(),
                                        });
                                        accumulated_data.clear();
                                    }
                                }
                                Err(_) => {
                                    connection_lost_read.store(true, Ordering::Relaxed);
                                    break;
                                }
                            }
                        }
                        _ = tokio::time::sleep(Duration::from_millis(100)) => {
                            // Timeout để check flags
                        }
                    }
                }
            });

            // Task ghi data với retry logic
            let running_write = running_clone.clone();
            let app_write = app_clone.clone();
            let conn_id_write = connection_id_clone.clone();

            // Take receiver từ Option
            let mut rx = match rx_option.take() {
                Some(r) => r,
                None => {
                    // Không có receiver, không thể tiếp tục
                    break 'connection_loop;
                }
            };

            let write_task = tokio::spawn(async move {
                while running_write.load(Ordering::Relaxed) && !connection_lost_write.load(Ordering::Relaxed) {
                    tokio::select! {
                        Some(data) = rx.recv() => {
                            let mut write_success = false;
                            let mut last_error: Option<std::io::Error> = None;

                            // Retry logic cho việc gửi data
                            for retry in 0..TCP_CLIENT_MAX_RETRIES {
                                match tokio::time::timeout(
                                    Duration::from_secs(5),
                                    write_half.write_all(&data)
                                ).await {
                                    Ok(Ok(())) => {
                                        // Flush với timeout
                                        if tokio::time::timeout(
                                            Duration::from_secs(2),
                                            write_half.flush()
                                        ).await.is_ok() {
                                            write_success = true;
                                            break;
                                        }
                                    }
                                    Ok(Err(e)) => {
                                        last_error = Some(e);
                                    }
                                    Err(_) => {
                                        // Timeout
                                        last_error = Some(std::io::Error::new(
                                            std::io::ErrorKind::TimedOut,
                                            "Write timeout"
                                        ));
                                    }
                                }

                                // Nếu không phải lần retry cuối, đợi rồi thử lại
                                if retry < TCP_CLIENT_MAX_RETRIES - 1 {
                                    let _ = app_write.emit("tcp-client-status", TcpConnectionStatus {
                                        connection_id: conn_id_write.clone(),
                                        status: "retrying".to_string(),
                                        message: Some(format!("Gửi thất bại, đang thử lại ({}/{})", retry + 1, TCP_CLIENT_MAX_RETRIES)),
                                        timestamp: get_timestamp(),
                                    });
                                    tokio::time::sleep(Duration::from_millis(TCP_CLIENT_RETRY_DELAY_MS)).await;
                                }
                            }

                            if !write_success {
                                // Gửi thất bại sau tất cả các lần retry
                                let error_msg = last_error
                                    .map(|e| e.to_string())
                                    .unwrap_or_else(|| "Unknown error".to_string());

                                let _ = app_write.emit("tcp-client-status", TcpConnectionStatus {
                                    connection_id: conn_id_write.clone(),
                                    status: "write_failed".to_string(),
                                    message: Some(format!("Gửi thất bại sau {} lần thử: {}", TCP_CLIENT_MAX_RETRIES, error_msg)),
                                    timestamp: get_timestamp(),
                                });

                                connection_lost_write.store(true, Ordering::Relaxed);
                                break;
                            }
                        }
                        _ = tokio::time::sleep(Duration::from_millis(100)) => {}
                    }
                }
                rx // Return receiver để có thể reuse
            });

            // Wait cho tasks hoàn thành
            let (_, write_result) = tokio::join!(read_task, write_task);

            // Lấy lại receiver từ write task và put vào Option
            if let Ok(returned_rx) = write_result {
                rx_option = Some(returned_rx);
            }

            // Kiểm tra xem có nên reconnect không
            if !running_clone.load(Ordering::Relaxed) {
                // User muốn disconnect, thoát luôn
                break 'connection_loop;
            }

            if connection_lost.load(Ordering::Relaxed) {
                reconnect_attempts += 1;

                if reconnect_attempts >= TCP_CLIENT_MAX_RECONNECT_ATTEMPTS {
                    // Đã thử quá nhiều lần, emit disconnected
                    let _ = app_clone.emit("tcp-client-status", TcpConnectionStatus {
                        connection_id: connection_id_clone.clone(),
                        status: "disconnected".to_string(),
                        message: Some(format!("Mất kết nối sau {} lần thử kết nối lại", reconnect_attempts)),
                        timestamp: get_timestamp(),
                    });
                    break 'connection_loop;
                }

                // Emit reconnecting status
                let _ = app_clone.emit("tcp-client-status", TcpConnectionStatus {
                    connection_id: connection_id_clone.clone(),
                    status: "reconnecting".to_string(),
                    message: Some(format!("Mất kết nối, đang thử kết nối lại ({}/{})", reconnect_attempts, TCP_CLIENT_MAX_RECONNECT_ATTEMPTS)),
                    timestamp: get_timestamp(),
                });

                tokio::time::sleep(Duration::from_millis(TCP_CLIENT_RECONNECT_DELAY_MS)).await;
                continue 'connection_loop;
            }
        }

        // Emit final disconnected status nếu chưa emit
        let _ = app_clone.emit("tcp-client-status", TcpConnectionStatus {
            connection_id: connection_id_clone,
            status: "disconnected".to_string(),
            message: None,
            timestamp: get_timestamp(),
        });

        // Cleanup: remove connection từ HashMap để có thể reconnect
        {
            let mut clients = clients_clone.lock();
            clients.remove(&connection_id_for_cleanup);
        }
    });

    // Lưu connection handle
    {
        let mut clients = state.clients.lock();
        clients.insert(connection_id.clone(), TcpClientHandle {
            tx,
            running,
            config,
        });
    }

    Ok(format!("Đang kết nối đến {}", addr_for_response))
}

// Ngắt kết nối TCP client
#[tauri::command]
fn tcp_client_disconnect(state: State<TcpState>, connection_id: String) -> Result<String, String> {
    let mut clients = state.clients.lock();

    if let Some(handle) = clients.remove(&connection_id) {
        handle.running.store(false, Ordering::Relaxed);
        Ok(format!("Đã ngắt kết nối {}", connection_id))
    } else {
        Err(format!("Connection {} không tồn tại", connection_id))
    }
}

// Gửi data qua TCP client
#[tauri::command]
fn tcp_client_send(
    state: State<TcpState>,
    connection_id: String,
    data: String,
    is_hex: bool,
) -> Result<String, String> {
    let clients = state.clients.lock();

    let handle = clients
        .get(&connection_id)
        .ok_or_else(|| format!("Connection {} không tồn tại", connection_id))?;

    let bytes: Vec<u8> = if is_hex {
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

    let total_bytes = bytes.len();

    handle.tx.try_send(bytes)
        .map_err(|_| "Không thể gửi data".to_string())?;

    Ok(format!("Đã gửi {} bytes", total_bytes))
}

// Kiểm tra trạng thái TCP client
#[tauri::command]
fn is_tcp_client_connected(state: State<TcpState>, connection_id: String) -> bool {
    let clients = state.clients.lock();
    clients.contains_key(&connection_id)
}

// ===================== TCP SERVER COMMANDS =====================

// Khởi động TCP server
#[tauri::command]
fn tcp_server_start(
    app: AppHandle,
    state: State<TcpState>,
    config: TcpServerConfig,
) -> Result<String, String> {
    let server_id = config.server_id.clone();

    // Kiểm tra xem server đã tồn tại chưa
    {
        let servers = state.servers.lock();
        if servers.contains_key(&server_id) {
            return Err(format!("Server {} đã đang chạy", server_id));
        }
    }

    let addr = format!("{}:{}", config.bind_address, config.port);
    let running = Arc::new(AtomicBool::new(true));
    let running_clone = running.clone();
    let clients: Arc<Mutex<HashMap<String, TcpServerClientHandle>>> =
        Arc::new(Mutex::new(HashMap::new()));
    let clients_clone = clients.clone();
    let max_clients = config.max_clients;
    let server_id_clone = server_id.clone();
    let app_clone = app.clone();

    // Clone servers Arc để cleanup khi bind fail
    let servers_clone = state.servers.clone();
    let server_id_for_cleanup = server_id.clone();

    // Echo enabled state
    let echo_enabled = Arc::new(AtomicBool::new(false));
    let echo_enabled_clone = echo_enabled.clone();

    // Thêm server vào HashMap trước (để đánh dấu đang khởi động)
    {
        let mut servers = state.servers.lock();
        servers.insert(server_id.clone(), TcpServerHandle {
            config: config.clone(),
            running: running.clone(),
            clients: clients.clone(),
            echo_enabled: echo_enabled.clone(),
        });
    }

    // Spawn async task để listen
    let addr_clone = addr.clone();
    let addr_for_response = addr.clone();
    state.runtime.spawn(async move {
        let listener = match TcpListener::bind(&addr_clone).await {
            Ok(l) => l,
            Err(e) => {
                // Cleanup: remove server khỏi HashMap vì bind thất bại
                {
                    let mut servers = servers_clone.lock();
                    servers.remove(&server_id_for_cleanup);
                }

                let error_msg = if e.kind() == std::io::ErrorKind::AddrInUse {
                    format!("Cổng {} đã được sử dụng bởi ứng dụng khác", addr_clone)
                } else {
                    format!("Không thể bind: {}", e)
                };

                let _ = app_clone.emit("tcp-server-status", TcpConnectionStatus {
                    connection_id: server_id_clone.clone(),
                    status: "error".to_string(),
                    message: Some(error_msg),
                    timestamp: get_timestamp(),
                });
                return;
            }
        };

        // Emit started status
        let _ = app_clone.emit("tcp-server-status", TcpConnectionStatus {
            connection_id: server_id_clone.clone(),
            status: "started".to_string(),
            message: Some(format!("Listening on {}", addr_clone)),
            timestamp: get_timestamp(),
        });

        let mut client_counter: u32 = 0;

        loop {
            if !running_clone.load(Ordering::Relaxed) {
                break;
            }

            tokio::select! {
                result = listener.accept() => {
                    match result {
                        Ok((stream, addr)) => {
                            // Kiểm tra số lượng clients
                            {
                                let current_clients = clients_clone.lock();
                                if current_clients.len() >= max_clients as usize {
                                    // Từ chối kết nối
                                    continue;
                                }
                            }

                            client_counter += 1;
                            let client_id = format!("client-{}", client_counter);
                            let remote_addr = addr.to_string();
                            let connected_at = get_timestamp();

                            // Emit client connected event
                            let _ = app_clone.emit("tcp-server-client-event", TcpServerClientEvent {
                                server_id: server_id_clone.clone(),
                                client_id: client_id.clone(),
                                remote_addr: remote_addr.clone(),
                                event_type: "connected".to_string(),
                                timestamp: connected_at,
                            });

                            // Channel để gửi data đến client này
                            let (tx, mut rx) = mpsc::channel::<Vec<u8>>(100);

                            // Lưu client handle
                            {
                                let mut current_clients = clients_clone.lock();
                                current_clients.insert(client_id.clone(), TcpServerClientHandle {
                                    tx,
                                    info: TcpClientInfo {
                                        client_id: client_id.clone(),
                                        remote_addr: remote_addr.clone(),
                                        connected_at,
                                    },
                                });
                            }

                            // Spawn task để handle client
                            let clients_ref = clients_clone.clone();
                            let server_id_ref = server_id_clone.clone();
                            let client_id_ref = client_id.clone();
                            let remote_addr_ref = remote_addr.clone();
                            let app_ref = app_clone.clone();
                            let running_ref = running_clone.clone();
                            let echo_ref = echo_enabled_clone.clone();

                            tokio::spawn(async move {
                                let (mut read_half, mut write_half) = stream.into_split();
                                let mut buffer = [0u8; 4096];

                                loop {
                                    if !running_ref.load(Ordering::Relaxed) {
                                        break;
                                    }

                                    tokio::select! {
                                        // Đọc data từ client
                                        result = read_half.read(&mut buffer) => {
                                            match result {
                                                Ok(0) => break, // Client disconnected
                                                Ok(n) => {
                                                    let received_data = buffer[..n].to_vec();

                                                    // Emit data to frontend
                                                    let _ = app_ref.emit("tcp-data", TcpData {
                                                        connection_id: server_id_ref.clone(),
                                                        client_id: Some(client_id_ref.clone()),
                                                        data: received_data.clone(),
                                                        timestamp: get_timestamp(),
                                                    });

                                                    // Echo back if enabled
                                                    if echo_ref.load(Ordering::Relaxed) {
                                                        // Convert received bytes to string and prepend "Echo: "
                                                        let received_str = String::from_utf8_lossy(&received_data);
                                                        let echo_response = format!("Echo: {}", received_str);
                                                        let echo_bytes = echo_response.as_bytes().to_vec();

                                                        if let Err(_) = write_half.write_all(&echo_bytes).await {
                                                            break;
                                                        }
                                                        let _ = write_half.flush().await;
                                                    }
                                                }
                                                Err(_) => break,
                                            }
                                        }
                                        // Gửi data đến client (hoặc channel đóng = bị disconnect)
                                        result = rx.recv() => {
                                            match result {
                                                Some(data) => {
                                                    if let Err(_) = write_half.write_all(&data).await {
                                                        break;
                                                    }
                                                    let _ = write_half.flush().await;
                                                }
                                                None => {
                                                    // Channel closed - server disconnected this client
                                                    break;
                                                }
                                            }
                                        }
                                        _ = tokio::time::sleep(Duration::from_millis(100)) => {}
                                    }
                                }

                                // Cleanup: remove client
                                {
                                    let mut current_clients = clients_ref.lock();
                                    current_clients.remove(&client_id_ref);
                                }

                                // Emit client disconnected
                                let _ = app_ref.emit("tcp-server-client-event", TcpServerClientEvent {
                                    server_id: server_id_ref,
                                    client_id: client_id_ref,
                                    remote_addr: remote_addr_ref,
                                    event_type: "disconnected".to_string(),
                                    timestamp: get_timestamp(),
                                });
                            });
                        }
                        Err(_) => {
                            if !running_clone.load(Ordering::Relaxed) {
                                break;
                            }
                        }
                    }
                }
                _ = tokio::time::sleep(Duration::from_millis(100)) => {}
            }
        }

        // Cleanup: remove server khỏi HashMap khi loop kết thúc
        {
            let mut servers = servers_clone.lock();
            servers.remove(&server_id_for_cleanup);
        }

        // Emit stopped status
        let _ = app_clone.emit("tcp-server-status", TcpConnectionStatus {
            connection_id: server_id_clone,
            status: "stopped".to_string(),
            message: None,
            timestamp: get_timestamp(),
        });
    });

    Ok(format!("Server đang khởi động trên {}", addr_for_response))
}

// Dừng TCP server
#[tauri::command]
fn tcp_server_stop(state: State<TcpState>, server_id: String) -> Result<String, String> {
    let mut servers = state.servers.lock();

    if let Some(handle) = servers.remove(&server_id) {
        handle.running.store(false, Ordering::Relaxed);
        Ok(format!("Đã dừng server {}", server_id))
    } else {
        Err(format!("Server {} không tồn tại", server_id))
    }
}

// Bật/tắt echo cho TCP server
#[tauri::command]
fn tcp_server_set_echo(state: State<TcpState>, server_id: String, enabled: bool) -> Result<String, String> {
    let servers = state.servers.lock();

    if let Some(handle) = servers.get(&server_id) {
        handle.echo_enabled.store(enabled, Ordering::Relaxed);
        Ok(format!("Echo {} cho server {}", if enabled { "đã bật" } else { "đã tắt" }, server_id))
    } else {
        Err(format!("Server {} không tồn tại", server_id))
    }
}

// Gửi data đến clients (tất cả hoặc 1 client cụ thể)
#[tauri::command]
fn tcp_server_send(
    state: State<TcpState>,
    server_id: String,
    client_id: Option<String>,
    data: String,
    is_hex: bool,
) -> Result<String, String> {
    let servers = state.servers.lock();

    let handle = servers
        .get(&server_id)
        .ok_or_else(|| format!("Server {} không tồn tại", server_id))?;

    let bytes: Vec<u8> = if is_hex {
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

    let total_bytes = bytes.len();
    let clients = handle.clients.lock();

    if let Some(target_id) = client_id {
        // Gửi đến 1 client cụ thể
        if let Some(client) = clients.get(&target_id) {
            client.tx.try_send(bytes)
                .map_err(|_| "Không thể gửi data".to_string())?;
        } else {
            return Err(format!("Client {} không tồn tại", target_id));
        }
    } else {
        // Gửi đến tất cả clients
        for client in clients.values() {
            let _ = client.tx.try_send(bytes.clone());
        }
    }

    Ok(format!("Đã gửi {} bytes", total_bytes))
}

// Ngắt kết nối 1 client cụ thể
#[tauri::command]
fn tcp_server_disconnect_client(
    state: State<TcpState>,
    server_id: String,
    client_id: String,
) -> Result<String, String> {
    let servers = state.servers.lock();

    let handle = servers
        .get(&server_id)
        .ok_or_else(|| format!("Server {} không tồn tại", server_id))?;

    let mut clients = handle.clients.lock();

    if clients.remove(&client_id).is_some() {
        Ok(format!("Đã ngắt kết nối client {}", client_id))
    } else {
        Err(format!("Client {} không tồn tại", client_id))
    }
}

// Lấy danh sách clients đang kết nối
#[tauri::command]
fn tcp_server_get_clients(state: State<TcpState>, server_id: String) -> Result<Vec<TcpClientInfo>, String> {
    let servers = state.servers.lock();

    let handle = servers
        .get(&server_id)
        .ok_or_else(|| format!("Server {} không tồn tại", server_id))?;

    let clients = handle.clients.lock();
    let client_list: Vec<TcpClientInfo> = clients
        .values()
        .map(|h| h.info.clone())
        .collect();

    Ok(client_list)
}

// Kiểm tra trạng thái TCP server
#[tauri::command]
fn is_tcp_server_running(state: State<TcpState>, server_id: String) -> bool {
    let servers = state.servers.lock();
    servers.contains_key(&server_id)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_process::init())
        .manage(SerialState::default())
        .manage(TcpState::default())
        .invoke_handler(tauri::generate_handler![
            // Serial commands
            list_serial_ports,
            open_port,
            close_port,
            send_data,
            is_port_open,
            // TCP Client commands
            tcp_client_connect,
            tcp_client_disconnect,
            tcp_client_send,
            is_tcp_client_connected,
            // TCP Server commands
            tcp_server_start,
            tcp_server_stop,
            tcp_server_set_echo,
            tcp_server_send,
            tcp_server_disconnect_client,
            tcp_server_get_clients,
            is_tcp_server_running
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
