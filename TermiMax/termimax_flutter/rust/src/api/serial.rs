use super::models::*;
use super::state::get_state;
use crate::frb_generated::StreamSink;
use flutter_rust_bridge::frb;
use serialport::{DataBits, FlowControl, Parity, StopBits};
use std::io::{Read, Write};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};

/// Get current timestamp in milliseconds
fn get_timestamp() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64
}

/// List available USB serial ports
/// Filters to show only USB ports, platform-specific handling
pub fn list_serial_ports() -> Result<Vec<PortInfo>, String> {
    let ports = serialport::available_ports().map_err(|e| e.to_string())?;

    let port_list: Vec<PortInfo> = ports
        .into_iter()
        .filter(|p| {
            // Only USB ports
            if !matches!(p.port_type, serialport::SerialPortType::UsbPort(_)) {
                return false;
            }

            let name = &p.port_name;

            // Windows: COMx
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

            // macOS: /dev/tty.*
            #[cfg(target_os = "macos")]
            {
                name.starts_with("/dev/tty.")
            }

            // Fallback for other OS
            #[cfg(not(any(target_os = "windows", target_os = "linux", target_os = "macos")))]
            {
                true
            }
        })
        .map(|p| {
            let short_name = if cfg!(target_os = "windows") {
                p.port_name.clone()
            } else {
                p.port_name
                    .split('/')
                    .last()
                    .unwrap_or(&p.port_name)
                    .to_string()
            };

            // Extract USB info if available
            let (manufacturer, product) = match &p.port_type {
                serialport::SerialPortType::UsbPort(usb_info) => {
                    #[allow(unused_mut)]
                    let mut prod = usb_info.product.clone();
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

/// Open a serial port with the given configuration
pub fn open_port(config: SerialConfig) -> Result<String, String> {
    let state = get_state();
    let port_name = config.port_name.clone();

    // Check if already open
    {
        let ports = state.ports.lock();
        if ports.contains_key(&port_name) {
            return Err(format!("Port {} is already open", port_name));
        }
    }

    // Parse data bits
    let data_bits = match config.data_bits {
        5 => DataBits::Five,
        6 => DataBits::Six,
        7 => DataBits::Seven,
        8 => DataBits::Eight,
        _ => return Err("Invalid data bits".to_string()),
    };

    // Parse stop bits
    let stop_bits = match config.stop_bits.as_str() {
        "1" => StopBits::One,
        "1.5" => StopBits::Two, // serialport crate doesn't support 1.5, use 2
        "2" => StopBits::Two,
        _ => return Err("Invalid stop bits".to_string()),
    };

    // Parse parity
    let parity = match config.parity.to_lowercase().as_str() {
        "none" => Parity::None,
        "odd" => Parity::Odd,
        "even" => Parity::Even,
        _ => return Err("Invalid parity".to_string()),
    };

    // Open port with short timeout for responsive polling
    let mut port = serialport::new(&port_name, config.baud_rate)
        .data_bits(data_bits)
        .stop_bits(stop_bits)
        .parity(parity)
        .flow_control(FlowControl::None)
        .timeout(Duration::from_millis(5))
        .open()
        .map_err(|e| {
            let err_str = e.to_string().to_lowercase();
            if err_str.contains("busy")
                || err_str.contains("resource busy")
                || err_str.contains("access denied")
                || err_str.contains("permission denied")
                || err_str.contains("already in use")
            {
                format!("BUSY:{}", port_name)
            } else {
                format!("ERROR:{}:{}", port_name, e)
            }
        })?;

    // Set DTR and RTS
    port.write_data_terminal_ready(config.dtr)
        .map_err(|e| format!("Cannot set DTR: {}", e))?;
    port.write_request_to_send(config.rts)
        .map_err(|e| format!("Cannot set RTS: {}", e))?;

    let port = Arc::new(parking_lot::Mutex::new(port));
    let running = Arc::new(AtomicBool::new(true));

    // Store in state
    {
        let mut ports = state.ports.lock();
        ports.insert(port_name.clone(), port);
    }
    {
        let mut running_map = state.running.lock();
        running_map.insert(port_name.clone(), running);
    }

    Ok(format!("Port {} opened successfully", port_name))
}

/// Close a serial port
pub fn close_port(port_name: String) -> Result<String, String> {
    let state = get_state();

    // Stop reading thread by setting running flag to false
    {
        let running = state.running.lock();
        if let Some(flag) = running.get(&port_name) {
            flag.store(false, Ordering::Relaxed);
        }
    }

    // Wait for thread to stop
    thread::sleep(Duration::from_millis(200));

    // Remove from state
    {
        let mut ports = state.ports.lock();
        ports.remove(&port_name);
    }
    {
        let mut running = state.running.lock();
        running.remove(&port_name);
    }

    Ok(format!("Port {} closed", port_name))
}

/// Send data through a serial port
///
/// # Arguments
/// * `port_name` - Name of the port to send to
/// * `data` - Data string to send
/// * `is_hex` - If true, parse data as hex string (e.g., "48 65 6C 6C 6F")
/// * `byte_delay_us` - Optional inter-byte delay in microseconds (0 = disabled)
pub fn send_data(
    port_name: String,
    data: String,
    is_hex: bool,
    byte_delay_us: Option<u64>,
) -> Result<String, String> {
    let state = get_state();
    let ports = state.ports.lock();
    let port = ports
        .get(&port_name)
        .ok_or_else(|| format!("Port {} is not open", port_name))?;

    // Parse data
    let bytes: Vec<u8> = if is_hex {
        // Parse hex string - only take valid hex characters
        let hex_chars: String = data.chars().filter(|c| c.is_ascii_hexdigit()).collect();

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
        // Inter-byte delay mode: send byte by byte with delay
        for byte in bytes.iter() {
            port.write_all(&[*byte])
                .map_err(|e| format!("Send error: {}", e))?;
            thread::sleep(Duration::from_micros(delay));
        }
        port.flush().map_err(|e| format!("Flush error: {}", e))?;
    } else {
        // Chunking mode: send in chunks to prevent character loss
        const CHUNK_SIZE: usize = 256;

        for chunk in bytes.chunks(CHUNK_SIZE) {
            port.write_all(chunk)
                .map_err(|e| format!("Send error: {}", e))?;
            port.flush().map_err(|e| format!("Flush error: {}", e))?;

            // Small delay between chunks
            if total_bytes > CHUNK_SIZE {
                thread::sleep(Duration::from_micros(500));
            }
        }
    }

    Ok(format!("Sent {} bytes", total_bytes))
}

/// Check if a port is currently open
#[frb(sync)]
pub fn is_port_open(port_name: String) -> bool {
    let state = get_state();
    let ports = state.ports.lock();
    ports.contains_key(&port_name)
}

/// Stream serial data from a port to Flutter via StreamSink
/// This is the key function that replaces Tauri's event emitter
///
/// # Arguments
/// * `port_name` - Name of the port to read from
/// * `baud_rate` - Baud rate (used for calculating gap timeout)
/// * `sink` - StreamSink to send data to Flutter
pub fn stream_serial_data(
    port_name: String,
    baud_rate: u32,
    sink: StreamSink<SerialData>,
) {
    thread::spawn(move || {
        let state = get_state();
        let mut buffer = [0u8; 4096];
        let _ = baud_rate; // Unused for now, kept for API compatibility

        // Simple approach: emit immediately when data is available
        // For high-frequency data, Flutter side handles line parsing

        loop {
            // Check if should continue running
            let should_run = {
                let running = state.running.lock();
                running
                    .get(&port_name)
                    .map(|r| r.load(Ordering::Relaxed))
                    .unwrap_or(false)
            };

            if !should_run {
                break;
            }

            // Read data from port
            let bytes_read = {
                let ports = state.ports.lock();
                if let Some(port) = ports.get(&port_name) {
                    let mut port = port.lock();
                    match port.read(&mut buffer) {
                        Ok(n) => Some(n),
                        Err(ref e) if e.kind() == std::io::ErrorKind::TimedOut => None,
                        Err(e) => {
                            // Port error - emit disconnect event
                            let _ = sink.add(SerialData {
                                port_name: port_name.clone(),
                                data: vec![], // Empty data signals disconnection
                                timestamp: get_timestamp(),
                            });
                            eprintln!("Serial read error on {}: {}", port_name, e);
                            break;
                        }
                    }
                } else {
                    break;
                }
            };

            // Emit immediately when data available
            if let Some(n) = bytes_read {
                if n > 0 {
                    let data = SerialData {
                        port_name: port_name.clone(),
                        data: buffer[..n].to_vec(),
                        timestamp: get_timestamp(),
                    };

                    if sink.add(data).is_err() {
                        break;
                    }
                }
            }
        }
    });
}

/// Simple test function to verify bridge works
#[frb(sync)]
pub fn greet_from_serial(name: String) -> String {
    format!("Hello {} from TermiMax Serial module! Rust is ready.", name)
}
