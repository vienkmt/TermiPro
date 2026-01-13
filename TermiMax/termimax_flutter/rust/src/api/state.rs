use flutter_rust_bridge::frb;
use once_cell::sync::OnceCell;
use parking_lot::Mutex;
use serialport::SerialPort;
use std::collections::HashMap;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;

/// Global state manager for serial ports
/// This struct is internal-only and should not be exposed to FFI
#[frb(ignore)]
pub struct SerialState {
    /// Open serial ports: port_name -> SerialPort
    pub ports: Mutex<HashMap<String, Arc<Mutex<Box<dyn SerialPort>>>>>,
    /// Running flags for read threads: port_name -> running flag
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

impl Default for SerialState {
    fn default() -> Self {
        Self::new()
    }
}

/// Global singleton for serial state
static SERIAL_STATE: OnceCell<Arc<SerialState>> = OnceCell::new();

/// Get the global serial state instance
/// Internal function - not exposed to FFI
#[frb(ignore)]
pub fn get_state() -> &'static Arc<SerialState> {
    SERIAL_STATE.get_or_init(|| Arc::new(SerialState::new()))
}
