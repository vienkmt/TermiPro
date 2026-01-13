# Phase 2: Rust Serial API Implementation

## Objective
Port Rust serial logic từ Tauri sang flutter_rust_bridge

---

## Tasks

### 2.1 Define Models

```rust
// rust/src/api/models.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PortInfo {
    pub name: String,
    pub port_type: String,
    pub manufacturer: Option<String>,
    pub product: Option<String>,
}

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

#[derive(Debug, Serialize, Clone)]
pub struct SerialData {
    pub port_name: String,
    pub data: Vec<u8>,
    pub timestamp: u64,
}

#[derive(Debug, Serialize, Clone)]
pub struct PortDisconnected {
    pub port_name: String,
    pub reason: String,
    pub timestamp: u64,
}
```

### 2.2 Create State Manager

```rust
// rust/src/api/state.rs
use parking_lot::Mutex;
use serialport::SerialPort;
use std::collections::HashMap;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;
use once_cell::sync::OnceCell;

pub struct SerialState {
    pub ports: Mutex<HashMap<String, Arc<Mutex<Box<dyn SerialPort>>>>>,
    pub running: Mutex<HashMap<String, Arc<AtomicBool>>>,
}

impl SerialState {
    pub fn new() -> Self {
        Self {
            ports: Mutex::new(HashMap::new()),
            running: Mutex::new(HashMap::new()),
        }
    }
}

static SERIAL_STATE: OnceCell<Arc<SerialState>> = OnceCell::new();

pub fn get_state() -> &'static Arc<SerialState> {
    SERIAL_STATE.get_or_init(|| Arc::new(SerialState::new()))
}
```

### 2.3 Implement list_serial_ports()

```rust
// rust/src/api/serial.rs
use super::models::*;
use super::state::get_state;
use flutter_rust_bridge::StreamSink;
use serialport::{DataBits, FlowControl, Parity, StopBits};
use std::io::{Read, Write};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};

/// List available USB serial ports
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

            #[cfg(target_os = "windows")]
            {
                name.starts_with("COM")
            }

            #[cfg(target_os = "linux")]
            {
                name.starts_with("/dev/ttyUSB")
                    || name.starts_with("/dev/ttyACM")
                    || name.starts_with("/dev/ttyS")
            }

            #[cfg(target_os = "macos")]
            {
                name.starts_with("/dev/tty.")
            }

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
```

### 2.4 Implement open_port()

```rust
/// Open serial port with configuration
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
        "1.5" => StopBits::Two,
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

    // Open port
    let mut port = serialport::new(&port_name, config.baud_rate)
        .data_bits(data_bits)
        .stop_bits(stop_bits)
        .parity(parity)
        .flow_control(FlowControl::None)
        .timeout(Duration::from_millis(5))
        .open()
        .map_err(|e| {
            let err_str = e.to_string().to_lowercase();
            if err_str.contains("busy") || err_str.contains("access denied") {
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
```

### 2.5 Implement close_port()

```rust
/// Close serial port
pub fn close_port(port_name: String) -> Result<String, String> {
    let state = get_state();

    // Stop reading thread
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
```

### 2.6 Implement send_data()

```rust
/// Send data through serial port
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

    let mut port = port.lock();
    let total_bytes = bytes.len();
    let delay = byte_delay_us.unwrap_or(0);

    if delay > 0 {
        for byte in bytes.iter() {
            port.write_all(&[*byte])
                .map_err(|e| format!("Send error: {}", e))?;
            thread::sleep(Duration::from_micros(delay));
        }
        port.flush().map_err(|e| format!("Flush error: {}", e))?;
    } else {
        const CHUNK_SIZE: usize = 256;
        for chunk in bytes.chunks(CHUNK_SIZE) {
            port.write_all(chunk)
                .map_err(|e| format!("Send error: {}", e))?;
            port.flush().map_err(|e| format!("Flush error: {}", e))?;
            if total_bytes > CHUNK_SIZE {
                thread::sleep(Duration::from_micros(500));
            }
        }
    }

    Ok(format!("Sent {} bytes", total_bytes))
}
```

### 2.7 Implement is_port_open()

```rust
/// Check if port is open
#[flutter_rust_bridge::frb(sync)]
pub fn is_port_open(port_name: String) -> bool {
    let state = get_state();
    let ports = state.ports.lock();
    ports.contains_key(&port_name)
}
```

### 2.8 Implement stream_serial_data() - KEY DIFFERENCE

```rust
/// Stream serial data from port to Flutter
/// This replaces Tauri's event emitter with StreamSink
pub fn stream_serial_data(
    port_name: String,
    baud_rate: u32,
    sink: StreamSink<SerialData>,
) {
    thread::spawn(move || {
        let state = get_state();
        let mut buffer = [0u8; 4096];

        // Dynamic gap timeout based on baud rate
        let gap_timeout_ms: u64 = {
            let time_for_256b = (256 * 10 * 1000) / baud_rate as u64;
            time_for_256b.clamp(5, 50)
        };

        let mut accumulated_data: Vec<u8> = Vec::with_capacity(8192);
        let mut last_data_received = Instant::now();
        let mut has_pending_data = false;

        loop {
            // Check if should continue
            let should_run = {
                let running = state.running.lock();
                *running.get(&port_name).map(|r| r.load(Ordering::Relaxed)).unwrap_or(&false)
            };

            if !should_run {
                break;
            }

            // Read data
            let bytes_read = {
                let ports = state.ports.lock();
                if let Some(port) = ports.get(&port_name) {
                    let mut port = port.lock();
                    match port.read(&mut buffer) {
                        Ok(n) => Some(n),
                        Err(ref e) if e.kind() == std::io::ErrorKind::TimedOut => None,
                        Err(_) => {
                            // Port error - emit disconnect and break
                            let _ = sink.add(SerialData {
                                port_name: port_name.clone(),
                                data: vec![], // Empty data signals error
                                timestamp: get_timestamp(),
                            });
                            break;
                        }
                    }
                } else {
                    break;
                }
            };

            // Accumulate data
            if let Some(n) = bytes_read {
                if n > 0 {
                    accumulated_data.extend_from_slice(&buffer[..n]);
                    last_data_received = Instant::now();
                    has_pending_data = true;
                }
            }

            // Emit when gap detected
            let should_emit = has_pending_data
                && last_data_received.elapsed() > Duration::from_millis(gap_timeout_ms);

            if should_emit {
                let data = SerialData {
                    port_name: port_name.clone(),
                    data: accumulated_data.clone(),
                    timestamp: get_timestamp(),
                };

                if sink.add(data).is_err() {
                    // Stream closed on Dart side
                    break;
                }

                accumulated_data.clear();
                has_pending_data = false;
            }
        }

        // Emit remaining data before exit
        if !accumulated_data.is_empty() {
            let data = SerialData {
                port_name: port_name.clone(),
                data: accumulated_data,
                timestamp: get_timestamp(),
            };
            let _ = sink.add(data);
        }
    });
}

fn get_timestamp() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64
}
```

### 2.9 Update mod.rs

```rust
// rust/src/api/mod.rs
pub mod models;
mod state;
pub mod serial;

pub use models::*;
pub use serial::*;
```

---

## Key Differences from Tauri Implementation

| Aspect | Tauri | flutter_rust_bridge |
|--------|-------|---------------------|
| Command decorator | `#[tauri::command]` | None (plain function) |
| State management | `State<SerialState>` | `static OnceCell<Arc<SerialState>>` |
| Event emission | `app.emit("serial-data", data)` | `sink.add(data)` |
| App handle | `AppHandle` parameter | Not needed |
| Sync annotation | Default async | `#[frb(sync)]` for sync |

---

## Verification Checklist

- [ ] Models compile without errors
- [ ] State manager works with OnceCell
- [ ] `list_serial_ports()` returns correct ports
- [ ] `open_port()` opens port with all config options
- [ ] `close_port()` properly cleans up
- [ ] `send_data()` works in both text and hex mode
- [ ] `is_port_open()` returns correct status
- [ ] `stream_serial_data()` streams data continuously
- [ ] Stream properly handles disconnection
- [ ] No memory leaks in long-running streams

---

## Testing Commands

```bash
# Regenerate bridge
flutter_rust_bridge_codegen generate

# Build Rust only
cd rust && cargo build

# Run Flutter app
flutter run -d macos
```

---

## Expected Dart Usage

```dart
// List ports
final ports = await listSerialPorts();

// Open port
await openPort(config: SerialConfig(
  portName: '/dev/ttyUSB0',
  baudRate: 115200,
  dataBits: 8,
  stopBits: '1',
  parity: 'none',
  dtr: false,
  rts: false,
));

// Check status
final isOpen = isPortOpen(portName: '/dev/ttyUSB0');

// Stream data
final stream = streamSerialData(
  portName: '/dev/ttyUSB0',
  baudRate: 115200,
);

stream.listen((data) {
  print('Received: ${data.data.length} bytes');
});

// Send data
await sendData(
  portName: '/dev/ttyUSB0',
  data: 'Hello',
  isHex: false,
  byteDelayUs: null,
);

// Close port
await closePort(portName: '/dev/ttyUSB0');
```
