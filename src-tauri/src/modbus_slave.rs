// Modbus Slave Module for TermiPro
// Supports RTU (Serial) and TCP/IP server modes

use crate::modbus::{
    build_rtu_frame, build_tcp_frame, format_exception_error, get_timestamp,
    verify_crc16, FunctionCode, ModbusMode,
};
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Arc;

// ===================== CONSTANTS =====================

/// Default data storage size per type
const DEFAULT_DATA_SIZE: usize = 10000;

// ===================== DATA STRUCTURES =====================

/// Modbus data storage for a single slave instance
pub struct ModbusSlaveData {
    /// Coils (FC01 read, FC05/15 write) - addresses 0-9999
    pub coils: RwLock<Vec<bool>>,
    /// Discrete Inputs (FC02 read-only from master, but editable in UI)
    pub discrete_inputs: RwLock<Vec<bool>>,
    /// Holding Registers (FC03 read, FC06/16 write)
    pub holding_registers: RwLock<Vec<u16>>,
    /// Input Registers (FC04 read-only from master, but editable in UI)
    pub input_registers: RwLock<Vec<u16>>,
}

impl Default for ModbusSlaveData {
    fn default() -> Self {
        Self {
            coils: RwLock::new(vec![false; DEFAULT_DATA_SIZE]),
            discrete_inputs: RwLock::new(vec![false; DEFAULT_DATA_SIZE]),
            holding_registers: RwLock::new(vec![0u16; DEFAULT_DATA_SIZE]),
            input_registers: RwLock::new(vec![0u16; DEFAULT_DATA_SIZE]),
        }
    }
}

// ===================== CONFIG STRUCTS =====================

/// Modbus Slave RTU configuration
#[derive(Debug, Deserialize, Clone)]
pub struct ModbusSlaveRtuConfig {
    pub port_name: String,
    pub baud_rate: u32,
    pub data_bits: u8,
    pub stop_bits: String,
    pub parity: String,
    pub slave_id: u8,
}

/// Modbus Slave TCP configuration
#[derive(Debug, Deserialize, Clone)]
pub struct ModbusSlaveTcpConfig {
    pub listen_port: u16,
    pub bind_address: String,
    pub unit_id: u8,
}

/// Unified slave configuration
#[derive(Debug, Clone)]
pub enum ModbusSlaveConfig {
    Rtu(ModbusSlaveRtuConfig),
    Tcp(ModbusSlaveTcpConfig),
}

// ===================== SIMULATION TYPES =====================

/// Simulation mode for auto-updating values
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum SimulationType {
    /// No simulation
    None,
    /// Sine wave: oscillates between min and max
    SinWave {
        min: u16,
        max: u16,
        period_ms: u32,
    },
    /// Ramp: increases/decreases linearly
    Ramp {
        min: u16,
        max: u16,
        step: i16,
        interval_ms: u32,
        reverse_at_bounds: bool,
    },
    /// Random: random value within range
    Random {
        min: u16,
        max: u16,
        interval_ms: u32,
    },
}

/// Simulation configuration for a register
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulationConfig {
    pub data_type: String, // "coil", "holding_register", etc.
    pub address: u16,
    pub simulation: SimulationType,
}

// ===================== EXCEPTION MAPPING =====================

/// Exception mapping for testing error responses
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExceptionMapping {
    pub data_type: String,
    pub start_address: u16,
    pub end_address: u16,
    pub exception_code: u8,
}

// ===================== RESPONSE DELAY =====================

/// Response delay configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ResponseDelayConfig {
    /// Global delay for all responses (ms)
    pub global_delay_ms: u32,
    /// Per-function-code delays (FC -> delay_ms)
    #[serde(default)]
    pub fc_delays: HashMap<u8, u32>,
    /// Random delay range (min_ms, max_ms)
    #[serde(default)]
    pub random_delay: Option<(u32, u32)>,
}

// ===================== STATISTICS =====================

/// Detailed statistics for the slave
#[derive(Debug, Clone, Serialize, Default)]
pub struct SlaveStatistics {
    /// Total request count
    pub total_requests: u64,
    /// Per-FC request counts
    pub fc_counts: HashMap<u8, u64>,
    /// Per-FC success counts
    pub fc_success: HashMap<u8, u64>,
    /// Per-FC error counts
    pub fc_errors: HashMap<u8, u64>,
    /// Response times (min, max, sum, count) for averaging
    pub response_time_min_ms: u64,
    pub response_time_max_ms: u64,
    pub response_time_sum_ms: u64,
    pub response_time_count: u64,
    /// Requests per second (calculated)
    pub requests_per_second: f64,
    /// First request timestamp
    pub first_request_time: u64,
    /// Last request timestamp
    pub last_request_time: u64,
}

impl SlaveStatistics {
    pub fn record_request(&mut self, fc: u8, success: bool, response_time_ms: u64) {
        self.total_requests += 1;
        *self.fc_counts.entry(fc).or_insert(0) += 1;

        if success {
            *self.fc_success.entry(fc).or_insert(0) += 1;
        } else {
            *self.fc_errors.entry(fc).or_insert(0) += 1;
        }

        // Update response time stats
        if self.response_time_count == 0 {
            self.response_time_min_ms = response_time_ms;
            self.response_time_max_ms = response_time_ms;
        } else {
            self.response_time_min_ms = self.response_time_min_ms.min(response_time_ms);
            self.response_time_max_ms = self.response_time_max_ms.max(response_time_ms);
        }
        self.response_time_sum_ms += response_time_ms;
        self.response_time_count += 1;

        // Update timestamps
        let now = get_timestamp();
        if self.first_request_time == 0 {
            self.first_request_time = now;
        }
        self.last_request_time = now;

        // Calculate RPS
        let duration_sec = (self.last_request_time - self.first_request_time) as f64 / 1000.0;
        if duration_sec > 0.0 {
            self.requests_per_second = self.total_requests as f64 / duration_sec;
        }
    }

    #[allow(dead_code)]
    pub fn average_response_time_ms(&self) -> f64 {
        if self.response_time_count == 0 {
            0.0
        } else {
            self.response_time_sum_ms as f64 / self.response_time_count as f64
        }
    }
}

// ===================== TCP CLIENT =====================

/// Connected TCP client info
#[derive(Debug, Clone, Serialize)]
pub struct ModbusSlaveTcpClient {
    pub client_id: String,
    pub remote_addr: String,
    pub connected_at: u64,
    pub request_count: u64,
}

// ===================== CONNECTION HANDLE =====================

/// Slave connection handle
#[allow(dead_code)]
pub struct ModbusSlaveHandle {
    pub mode: ModbusMode,
    pub config: ModbusSlaveConfig,
    pub data: Arc<ModbusSlaveData>,
    pub running: AtomicBool,
    pub request_count: AtomicU64,
    pub last_request_time: AtomicU64,

    // Advanced features
    pub simulations: RwLock<Vec<SimulationConfig>>,
    pub exception_mappings: RwLock<Vec<ExceptionMapping>>,
    pub delay_config: RwLock<ResponseDelayConfig>,
    pub statistics: RwLock<SlaveStatistics>,

    // For TCP: connected clients
    pub tcp_clients: Option<Arc<RwLock<HashMap<String, ModbusSlaveTcpClient>>>>,
}

impl ModbusSlaveHandle {
    pub fn new_rtu(config: ModbusSlaveRtuConfig) -> Self {
        Self {
            mode: ModbusMode::Rtu,
            config: ModbusSlaveConfig::Rtu(config),
            data: Arc::new(ModbusSlaveData::default()),
            running: AtomicBool::new(true),
            request_count: AtomicU64::new(0),
            last_request_time: AtomicU64::new(0),
            simulations: RwLock::new(Vec::new()),
            exception_mappings: RwLock::new(Vec::new()),
            delay_config: RwLock::new(ResponseDelayConfig::default()),
            statistics: RwLock::new(SlaveStatistics::default()),
            tcp_clients: None,
        }
    }

    pub fn new_tcp(config: ModbusSlaveTcpConfig) -> Self {
        Self {
            mode: ModbusMode::Tcp,
            config: ModbusSlaveConfig::Tcp(config),
            data: Arc::new(ModbusSlaveData::default()),
            running: AtomicBool::new(true),
            request_count: AtomicU64::new(0),
            last_request_time: AtomicU64::new(0),
            simulations: RwLock::new(Vec::new()),
            exception_mappings: RwLock::new(Vec::new()),
            delay_config: RwLock::new(ResponseDelayConfig::default()),
            statistics: RwLock::new(SlaveStatistics::default()),
            tcp_clients: Some(Arc::new(RwLock::new(HashMap::new()))),
        }
    }

    #[allow(dead_code)]
    pub fn get_slave_id(&self) -> u8 {
        match &self.config {
            ModbusSlaveConfig::Rtu(c) => c.slave_id,
            ModbusSlaveConfig::Tcp(c) => c.unit_id,
        }
    }

    pub fn is_running(&self) -> bool {
        self.running.load(Ordering::SeqCst)
    }

    pub fn stop(&self) {
        self.running.store(false, Ordering::SeqCst);
    }

    pub fn increment_request_count(&self) {
        self.request_count.fetch_add(1, Ordering::SeqCst);
        self.last_request_time
            .store(get_timestamp(), Ordering::SeqCst);
    }

    /// Calculate delay for a request
    pub fn get_delay_ms(&self, fc: u8) -> u32 {
        let config = self.delay_config.read();
        let mut delay = config.global_delay_ms;

        // Add per-FC delay
        if let Some(&fc_delay) = config.fc_delays.get(&fc) {
            delay += fc_delay;
        }

        // Add random delay
        if let Some((min, max)) = config.random_delay {
            if max > min {
                delay += min + (rand_u32() % (max - min));
            }
        }

        delay
    }

    /// Check for exception mapping
    pub fn get_exception(&self, data_type: &str, address: u16) -> Option<u8> {
        let mappings = self.exception_mappings.read();
        for mapping in mappings.iter() {
            if mapping.data_type == data_type
                && address >= mapping.start_address
                && address <= mapping.end_address
            {
                return Some(mapping.exception_code);
            }
        }
        None
    }
}

// Simple random number generator (no external crate needed)
fn rand_u32() -> u32 {
    use std::time::{SystemTime, UNIX_EPOCH};
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .subsec_nanos();
    nanos.wrapping_mul(1103515245).wrapping_add(12345)
}

// ===================== EVENT STRUCTS =====================

/// Slave status event
#[derive(Debug, Clone, Serialize)]
pub struct ModbusSlaveStatusEvent {
    pub connection_id: String,
    pub status: String, // "started", "stopped", "error"
    pub message: Option<String>,
    pub timestamp: u64,
}

/// Incoming request event
#[derive(Debug, Clone, Serialize)]
pub struct ModbusSlaveRequestEvent {
    pub connection_id: String,
    pub client_id: Option<String>, // For TCP mode
    pub slave_id: u8,
    pub function_code: u8,
    pub start_address: u16,
    pub quantity: u16,
    pub request_frame: Vec<u8>,
    pub response_frame: Vec<u8>,
    pub success: bool,
    pub error_message: Option<String>,
    pub response_time_ms: u64,
    pub timestamp: u64,
}

/// Data changed event (when master writes)
#[derive(Debug, Clone, Serialize)]
pub struct ModbusSlaveDataChangedEvent {
    pub connection_id: String,
    pub data_type: String, // "coil", "holding_register"
    pub start_address: u16,
    pub values: Vec<u16>, // coils as 0/1
    pub timestamp: u64,
}

/// TCP client event
#[derive(Debug, Clone, Serialize)]
pub struct ModbusSlaveTcpClientEvent {
    pub connection_id: String,
    pub client_id: String,
    pub remote_addr: String,
    pub event_type: String, // "connected", "disconnected"
    pub timestamp: u64,
}

// ===================== REQUEST PROCESSING =====================

/// Result of processing a Modbus request
pub struct ProcessedRequest {
    pub response_frame: Vec<u8>,
    pub success: bool,
    pub error_message: Option<String>,
    pub data_changed: Option<ModbusSlaveDataChangedEvent>,
    pub start_address: u16,
    pub quantity: u16,
}

/// Parse RTU request and extract fields
pub struct ParsedRtuRequest {
    pub slave_id: u8,
    pub function_code: u8,
    pub start_address: u16,
    pub quantity: u16,
    pub write_values: Option<Vec<u16>>,
    pub coil_values: Option<Vec<bool>>,
}

/// Parse incoming RTU request frame
pub fn parse_rtu_request(frame: &[u8]) -> Result<ParsedRtuRequest, String> {
    if frame.len() < 8 {
        return Err("Frame too short".to_string());
    }

    if !verify_crc16(frame) {
        return Err("CRC error".to_string());
    }

    let slave_id = frame[0];
    let fc = frame[1];
    let start_address = u16::from_be_bytes([frame[2], frame[3]]);

    match FunctionCode::from_u8(fc) {
        Some(FunctionCode::ReadCoils)
        | Some(FunctionCode::ReadDiscreteInputs)
        | Some(FunctionCode::ReadHoldingRegisters)
        | Some(FunctionCode::ReadInputRegisters) => {
            let quantity = u16::from_be_bytes([frame[4], frame[5]]);
            Ok(ParsedRtuRequest {
                slave_id,
                function_code: fc,
                start_address,
                quantity,
                write_values: None,
                coil_values: None,
            })
        }
        Some(FunctionCode::WriteSingleCoil) => {
            let value = u16::from_be_bytes([frame[4], frame[5]]);
            let coil = value == 0xFF00;
            Ok(ParsedRtuRequest {
                slave_id,
                function_code: fc,
                start_address,
                quantity: 1,
                write_values: None,
                coil_values: Some(vec![coil]),
            })
        }
        Some(FunctionCode::WriteSingleRegister) => {
            let value = u16::from_be_bytes([frame[4], frame[5]]);
            Ok(ParsedRtuRequest {
                slave_id,
                function_code: fc,
                start_address,
                quantity: 1,
                write_values: Some(vec![value]),
                coil_values: None,
            })
        }
        Some(FunctionCode::WriteMultipleCoils) => {
            let quantity = u16::from_be_bytes([frame[4], frame[5]]);
            let byte_count = frame[6] as usize;

            if frame.len() < 9 + byte_count {
                return Err("Incomplete write multiple coils request".to_string());
            }

            let mut coils = Vec::with_capacity(quantity as usize);
            for i in 0..quantity as usize {
                let byte_idx = 7 + (i / 8);
                let bit_idx = i % 8;
                let coil = (frame[byte_idx] >> bit_idx) & 1 == 1;
                coils.push(coil);
            }

            Ok(ParsedRtuRequest {
                slave_id,
                function_code: fc,
                start_address,
                quantity,
                write_values: None,
                coil_values: Some(coils),
            })
        }
        Some(FunctionCode::WriteMultipleRegisters) => {
            let quantity = u16::from_be_bytes([frame[4], frame[5]]);
            let byte_count = frame[6] as usize;

            if frame.len() < 9 + byte_count {
                return Err("Incomplete write multiple registers request".to_string());
            }

            let mut values = Vec::with_capacity(quantity as usize);
            for i in 0..quantity as usize {
                let idx = 7 + i * 2;
                let value = u16::from_be_bytes([frame[idx], frame[idx + 1]]);
                values.push(value);
            }

            Ok(ParsedRtuRequest {
                slave_id,
                function_code: fc,
                start_address,
                quantity,
                write_values: Some(values),
                coil_values: None,
            })
        }
        None => Err(format!("Unknown function code: 0x{:02X}", fc)),
    }
}

/// Parse incoming TCP request frame (MBAP header + PDU)
pub fn parse_tcp_request(frame: &[u8]) -> Result<(u16, ParsedRtuRequest), String> {
    if frame.len() < 12 {
        return Err("TCP frame too short".to_string());
    }

    let transaction_id = u16::from_be_bytes([frame[0], frame[1]]);
    let _protocol_id = u16::from_be_bytes([frame[2], frame[3]]);
    let length = u16::from_be_bytes([frame[4], frame[5]]) as usize;
    let unit_id = frame[6];
    let fc = frame[7];
    let start_address = u16::from_be_bytes([frame[8], frame[9]]);

    if frame.len() < 6 + length {
        return Err("Incomplete TCP frame".to_string());
    }

    match FunctionCode::from_u8(fc) {
        Some(FunctionCode::ReadCoils)
        | Some(FunctionCode::ReadDiscreteInputs)
        | Some(FunctionCode::ReadHoldingRegisters)
        | Some(FunctionCode::ReadInputRegisters) => {
            let quantity = u16::from_be_bytes([frame[10], frame[11]]);
            Ok((
                transaction_id,
                ParsedRtuRequest {
                    slave_id: unit_id,
                    function_code: fc,
                    start_address,
                    quantity,
                    write_values: None,
                    coil_values: None,
                },
            ))
        }
        Some(FunctionCode::WriteSingleCoil) => {
            let value = u16::from_be_bytes([frame[10], frame[11]]);
            let coil = value == 0xFF00;
            Ok((
                transaction_id,
                ParsedRtuRequest {
                    slave_id: unit_id,
                    function_code: fc,
                    start_address,
                    quantity: 1,
                    write_values: None,
                    coil_values: Some(vec![coil]),
                },
            ))
        }
        Some(FunctionCode::WriteSingleRegister) => {
            let value = u16::from_be_bytes([frame[10], frame[11]]);
            Ok((
                transaction_id,
                ParsedRtuRequest {
                    slave_id: unit_id,
                    function_code: fc,
                    start_address,
                    quantity: 1,
                    write_values: Some(vec![value]),
                    coil_values: None,
                },
            ))
        }
        Some(FunctionCode::WriteMultipleCoils) => {
            let quantity = u16::from_be_bytes([frame[10], frame[11]]);
            let _byte_count = frame[12] as usize;

            let mut coils = Vec::with_capacity(quantity as usize);
            for i in 0..quantity as usize {
                let byte_idx = 13 + (i / 8);
                let bit_idx = i % 8;
                if byte_idx < frame.len() {
                    let coil = (frame[byte_idx] >> bit_idx) & 1 == 1;
                    coils.push(coil);
                }
            }

            Ok((
                transaction_id,
                ParsedRtuRequest {
                    slave_id: unit_id,
                    function_code: fc,
                    start_address,
                    quantity,
                    write_values: None,
                    coil_values: Some(coils),
                },
            ))
        }
        Some(FunctionCode::WriteMultipleRegisters) => {
            let quantity = u16::from_be_bytes([frame[10], frame[11]]);
            let _byte_count = frame[12] as usize;

            let mut values = Vec::with_capacity(quantity as usize);
            for i in 0..quantity as usize {
                let idx = 13 + i * 2;
                if idx + 1 < frame.len() {
                    let value = u16::from_be_bytes([frame[idx], frame[idx + 1]]);
                    values.push(value);
                }
            }

            Ok((
                transaction_id,
                ParsedRtuRequest {
                    slave_id: unit_id,
                    function_code: fc,
                    start_address,
                    quantity,
                    write_values: Some(values),
                    coil_values: None,
                },
            ))
        }
        None => Err(format!("Unknown function code: 0x{:02X}", fc)),
    }
}

/// Process a Modbus request and generate response
pub fn process_request(
    request: &ParsedRtuRequest,
    data: &ModbusSlaveData,
    mode: ModbusMode,
    transaction_id: u16,
    connection_id: &str,
    handle: &ModbusSlaveHandle,
) -> ProcessedRequest {
    let fc = request.function_code;
    let addr = request.start_address;
    let qty = request.quantity;

    // Check for configured exception
    let data_type = match FunctionCode::from_u8(fc) {
        Some(FunctionCode::ReadCoils) | Some(FunctionCode::WriteSingleCoil) | Some(FunctionCode::WriteMultipleCoils) => "coil",
        Some(FunctionCode::ReadDiscreteInputs) => "discrete_input",
        Some(FunctionCode::ReadHoldingRegisters) | Some(FunctionCode::WriteSingleRegister) | Some(FunctionCode::WriteMultipleRegisters) => "holding_register",
        Some(FunctionCode::ReadInputRegisters) => "input_register",
        None => "",
    };

    if let Some(exception_code) = handle.get_exception(data_type, addr) {
        let response = build_exception_response(request.slave_id, fc, exception_code, mode, transaction_id);
        return ProcessedRequest {
            response_frame: response,
            success: false,
            error_message: Some(format_exception_error(exception_code)),
            data_changed: None,
            start_address: addr,
            quantity: qty,
        };
    }

    // Validate address range
    let end_addr = addr as usize + qty as usize;
    if end_addr > DEFAULT_DATA_SIZE {
        let response = build_exception_response(request.slave_id, fc, 0x02, mode, transaction_id);
        return ProcessedRequest {
            response_frame: response,
            success: false,
            error_message: Some("Illegal Data Address".to_string()),
            data_changed: None,
            start_address: addr,
            quantity: qty,
        };
    }

    match FunctionCode::from_u8(fc) {
        Some(FunctionCode::ReadCoils) => {
            let coils = data.coils.read();
            let values: Vec<bool> = coils[addr as usize..end_addr].to_vec();
            let response = build_read_coils_response(request.slave_id, fc, &values, mode, transaction_id);
            ProcessedRequest {
                response_frame: response,
                success: true,
                error_message: None,
                data_changed: None,
                start_address: addr,
                quantity: qty,
            }
        }
        Some(FunctionCode::ReadDiscreteInputs) => {
            let discrete = data.discrete_inputs.read();
            let values: Vec<bool> = discrete[addr as usize..end_addr].to_vec();
            let response = build_read_coils_response(request.slave_id, fc, &values, mode, transaction_id);
            ProcessedRequest {
                response_frame: response,
                success: true,
                error_message: None,
                data_changed: None,
                start_address: addr,
                quantity: qty,
            }
        }
        Some(FunctionCode::ReadHoldingRegisters) => {
            let registers = data.holding_registers.read();
            let values: Vec<u16> = registers[addr as usize..end_addr].to_vec();
            let response = build_read_registers_response(request.slave_id, fc, &values, mode, transaction_id);
            ProcessedRequest {
                response_frame: response,
                success: true,
                error_message: None,
                data_changed: None,
                start_address: addr,
                quantity: qty,
            }
        }
        Some(FunctionCode::ReadInputRegisters) => {
            let registers = data.input_registers.read();
            let values: Vec<u16> = registers[addr as usize..end_addr].to_vec();
            let response = build_read_registers_response(request.slave_id, fc, &values, mode, transaction_id);
            ProcessedRequest {
                response_frame: response,
                success: true,
                error_message: None,
                data_changed: None,
                start_address: addr,
                quantity: qty,
            }
        }
        Some(FunctionCode::WriteSingleCoil) => {
            if let Some(ref coil_values) = request.coil_values {
                if let Some(&value) = coil_values.first() {
                    let mut coils = data.coils.write();
                    coils[addr as usize] = value;
                    drop(coils);

                    let response = build_write_single_coil_response(request.slave_id, addr, value, mode, transaction_id);
                    let changed_event = ModbusSlaveDataChangedEvent {
                        connection_id: connection_id.to_string(),
                        data_type: "coil".to_string(),
                        start_address: addr,
                        values: vec![if value { 1 } else { 0 }],
                        timestamp: get_timestamp(),
                    };
                    return ProcessedRequest {
                        response_frame: response,
                        success: true,
                        error_message: None,
                        data_changed: Some(changed_event),
                        start_address: addr,
                        quantity: 1,
                    };
                }
            }
            let response = build_exception_response(request.slave_id, fc, 0x03, mode, transaction_id);
            ProcessedRequest {
                response_frame: response,
                success: false,
                error_message: Some("Illegal Data Value".to_string()),
                data_changed: None,
                start_address: addr,
                quantity: qty,
            }
        }
        Some(FunctionCode::WriteSingleRegister) => {
            if let Some(ref write_values) = request.write_values {
                if let Some(&value) = write_values.first() {
                    let mut registers = data.holding_registers.write();
                    registers[addr as usize] = value;
                    drop(registers);

                    let response = build_write_single_register_response(request.slave_id, addr, value, mode, transaction_id);
                    let changed_event = ModbusSlaveDataChangedEvent {
                        connection_id: connection_id.to_string(),
                        data_type: "holding_register".to_string(),
                        start_address: addr,
                        values: vec![value],
                        timestamp: get_timestamp(),
                    };
                    return ProcessedRequest {
                        response_frame: response,
                        success: true,
                        error_message: None,
                        data_changed: Some(changed_event),
                        start_address: addr,
                        quantity: 1,
                    };
                }
            }
            let response = build_exception_response(request.slave_id, fc, 0x03, mode, transaction_id);
            ProcessedRequest {
                response_frame: response,
                success: false,
                error_message: Some("Illegal Data Value".to_string()),
                data_changed: None,
                start_address: addr,
                quantity: qty,
            }
        }
        Some(FunctionCode::WriteMultipleCoils) => {
            if let Some(ref coil_values) = request.coil_values {
                let mut coils = data.coils.write();
                for (i, &value) in coil_values.iter().enumerate() {
                    coils[addr as usize + i] = value;
                }
                drop(coils);

                let response = build_write_multiple_response(request.slave_id, fc, addr, qty, mode, transaction_id);
                let changed_event = ModbusSlaveDataChangedEvent {
                    connection_id: connection_id.to_string(),
                    data_type: "coil".to_string(),
                    start_address: addr,
                    values: coil_values.iter().map(|&c| if c { 1 } else { 0 }).collect(),
                    timestamp: get_timestamp(),
                };
                return ProcessedRequest {
                    response_frame: response,
                    success: true,
                    error_message: None,
                    data_changed: Some(changed_event),
                    start_address: addr,
                    quantity: qty,
                };
            }
            let response = build_exception_response(request.slave_id, fc, 0x03, mode, transaction_id);
            ProcessedRequest {
                response_frame: response,
                success: false,
                error_message: Some("Illegal Data Value".to_string()),
                data_changed: None,
                start_address: addr,
                quantity: qty,
            }
        }
        Some(FunctionCode::WriteMultipleRegisters) => {
            if let Some(ref write_values) = request.write_values {
                let mut registers = data.holding_registers.write();
                for (i, &value) in write_values.iter().enumerate() {
                    registers[addr as usize + i] = value;
                }
                drop(registers);

                let response = build_write_multiple_response(request.slave_id, fc, addr, qty, mode, transaction_id);
                let changed_event = ModbusSlaveDataChangedEvent {
                    connection_id: connection_id.to_string(),
                    data_type: "holding_register".to_string(),
                    start_address: addr,
                    values: write_values.clone(),
                    timestamp: get_timestamp(),
                };
                return ProcessedRequest {
                    response_frame: response,
                    success: true,
                    error_message: None,
                    data_changed: Some(changed_event),
                    start_address: addr,
                    quantity: qty,
                };
            }
            let response = build_exception_response(request.slave_id, fc, 0x03, mode, transaction_id);
            ProcessedRequest {
                response_frame: response,
                success: false,
                error_message: Some("Illegal Data Value".to_string()),
                data_changed: None,
                start_address: addr,
                quantity: qty,
            }
        }
        None => {
            let response = build_exception_response(request.slave_id, fc, 0x01, mode, transaction_id);
            ProcessedRequest {
                response_frame: response,
                success: false,
                error_message: Some("Illegal Function".to_string()),
                data_changed: None,
                start_address: addr,
                quantity: qty,
            }
        }
    }
}

// ===================== RESPONSE BUILDING =====================

/// Build exception response
fn build_exception_response(
    slave_id: u8,
    fc: u8,
    exception_code: u8,
    mode: ModbusMode,
    transaction_id: u16,
) -> Vec<u8> {
    let data = vec![exception_code];
    let exception_fc = fc | 0x80;

    match mode {
        ModbusMode::Rtu => build_rtu_frame(slave_id, exception_fc, &data),
        ModbusMode::Tcp => build_tcp_frame(transaction_id, slave_id, exception_fc, &data),
    }
}

/// Build read coils/discrete inputs response
fn build_read_coils_response(
    slave_id: u8,
    fc: u8,
    values: &[bool],
    mode: ModbusMode,
    transaction_id: u16,
) -> Vec<u8> {
    let byte_count = ((values.len() + 7) / 8) as u8;
    let mut data = vec![byte_count];

    // Pack coils into bytes
    let mut byte_val: u8 = 0;
    for (i, &coil) in values.iter().enumerate() {
        if coil {
            byte_val |= 1 << (i % 8);
        }
        if (i + 1) % 8 == 0 || i == values.len() - 1 {
            data.push(byte_val);
            byte_val = 0;
        }
    }

    match mode {
        ModbusMode::Rtu => build_rtu_frame(slave_id, fc, &data),
        ModbusMode::Tcp => build_tcp_frame(transaction_id, slave_id, fc, &data),
    }
}

/// Build read registers response
fn build_read_registers_response(
    slave_id: u8,
    fc: u8,
    values: &[u16],
    mode: ModbusMode,
    transaction_id: u16,
) -> Vec<u8> {
    let byte_count = (values.len() * 2) as u8;
    let mut data = vec![byte_count];

    for &value in values {
        data.extend_from_slice(&value.to_be_bytes());
    }

    match mode {
        ModbusMode::Rtu => build_rtu_frame(slave_id, fc, &data),
        ModbusMode::Tcp => build_tcp_frame(transaction_id, slave_id, fc, &data),
    }
}

/// Build write single coil response (echo)
fn build_write_single_coil_response(
    slave_id: u8,
    address: u16,
    value: bool,
    mode: ModbusMode,
    transaction_id: u16,
) -> Vec<u8> {
    let mut data = Vec::with_capacity(4);
    data.extend_from_slice(&address.to_be_bytes());
    data.extend_from_slice(if value { &[0xFF, 0x00] } else { &[0x00, 0x00] });

    match mode {
        ModbusMode::Rtu => build_rtu_frame(slave_id, 0x05, &data),
        ModbusMode::Tcp => build_tcp_frame(transaction_id, slave_id, 0x05, &data),
    }
}

/// Build write single register response (echo)
fn build_write_single_register_response(
    slave_id: u8,
    address: u16,
    value: u16,
    mode: ModbusMode,
    transaction_id: u16,
) -> Vec<u8> {
    let mut data = Vec::with_capacity(4);
    data.extend_from_slice(&address.to_be_bytes());
    data.extend_from_slice(&value.to_be_bytes());

    match mode {
        ModbusMode::Rtu => build_rtu_frame(slave_id, 0x06, &data),
        ModbusMode::Tcp => build_tcp_frame(transaction_id, slave_id, 0x06, &data),
    }
}

/// Build write multiple coils/registers response
fn build_write_multiple_response(
    slave_id: u8,
    fc: u8,
    start_address: u16,
    quantity: u16,
    mode: ModbusMode,
    transaction_id: u16,
) -> Vec<u8> {
    let mut data = Vec::with_capacity(4);
    data.extend_from_slice(&start_address.to_be_bytes());
    data.extend_from_slice(&quantity.to_be_bytes());

    match mode {
        ModbusMode::Rtu => build_rtu_frame(slave_id, fc, &data),
        ModbusMode::Tcp => build_tcp_frame(transaction_id, slave_id, fc, &data),
    }
}

// ===================== DATA PERSISTENCE =====================

/// Data export structure for persistence
#[derive(Debug, Serialize, Deserialize)]
pub struct ModbusSlaveDataExport {
    pub coils: Vec<bool>,
    pub discrete_inputs: Vec<bool>,
    pub holding_registers: Vec<u16>,
    pub input_registers: Vec<u16>,
}

impl ModbusSlaveData {
    /// Export all data for persistence
    pub fn export(&self) -> ModbusSlaveDataExport {
        ModbusSlaveDataExport {
            coils: self.coils.read().clone(),
            discrete_inputs: self.discrete_inputs.read().clone(),
            holding_registers: self.holding_registers.read().clone(),
            input_registers: self.input_registers.read().clone(),
        }
    }

    /// Import data from export
    pub fn import(&self, data: ModbusSlaveDataExport) {
        *self.coils.write() = data.coils;
        *self.discrete_inputs.write() = data.discrete_inputs;
        *self.holding_registers.write() = data.holding_registers;
        *self.input_registers.write() = data.input_registers;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_rtu_read_holding_registers() {
        // Read 10 holding registers starting at address 0 from slave 1
        let frame = [0x01, 0x03, 0x00, 0x00, 0x00, 0x0A, 0xC5, 0xCD];
        let result = parse_rtu_request(&frame);
        assert!(result.is_ok());
        let req = result.unwrap();
        assert_eq!(req.slave_id, 1);
        assert_eq!(req.function_code, 0x03);
        assert_eq!(req.start_address, 0);
        assert_eq!(req.quantity, 10);
    }

    #[test]
    fn test_build_read_registers_response() {
        let values = vec![100, 200, 300];
        let response = build_read_registers_response(1, 0x03, &values, ModbusMode::Rtu, 0);
        assert_eq!(response[0], 1); // Slave ID
        assert_eq!(response[1], 0x03); // FC
        assert_eq!(response[2], 6); // Byte count
    }

    #[test]
    fn test_statistics() {
        let mut stats = SlaveStatistics::default();
        stats.record_request(0x03, true, 5);
        stats.record_request(0x03, true, 10);
        stats.record_request(0x03, false, 15);

        assert_eq!(stats.total_requests, 3);
        assert_eq!(stats.fc_counts.get(&0x03), Some(&3));
        assert_eq!(stats.fc_success.get(&0x03), Some(&2));
        assert_eq!(stats.fc_errors.get(&0x03), Some(&1));
        assert_eq!(stats.response_time_min_ms, 5);
        assert_eq!(stats.response_time_max_ms, 15);
        assert!((stats.average_response_time_ms() - 10.0).abs() < 0.01);
    }
}
