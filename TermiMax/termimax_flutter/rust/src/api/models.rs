use serde::{Deserialize, Serialize};

/// Serial port information
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PortInfo {
    pub name: String,
    pub port_type: String,
    pub manufacturer: Option<String>,
    pub product: Option<String>,
}

/// Serial port configuration
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

/// Data received from serial port
#[derive(Debug, Serialize, Clone)]
pub struct SerialData {
    pub port_name: String,
    pub data: Vec<u8>,
    pub timestamp: u64,
}

/// Port disconnection event
#[derive(Debug, Serialize, Clone)]
pub struct PortDisconnected {
    pub port_name: String,
    pub reason: String,
    pub timestamp: u64,
}
