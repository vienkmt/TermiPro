// Modbus Protocol Module for TermiPro
// Supports RTU (Serial) and TCP/IP modes

use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

// ===================== CONSTANTS =====================

/// CRC-16 Modbus lookup table (pre-computed)
const CRC_TABLE: [u16; 256] = [
    0x0000, 0xC0C1, 0xC181, 0x0140, 0xC301, 0x03C0, 0x0280, 0xC241,
    0xC601, 0x06C0, 0x0780, 0xC741, 0x0500, 0xC5C1, 0xC481, 0x0440,
    0xCC01, 0x0CC0, 0x0D80, 0xCD41, 0x0F00, 0xCFC1, 0xCE81, 0x0E40,
    0x0A00, 0xCAC1, 0xCB81, 0x0B40, 0xC901, 0x09C0, 0x0880, 0xC841,
    0xD801, 0x18C0, 0x1980, 0xD941, 0x1B00, 0xDBC1, 0xDA81, 0x1A40,
    0x1E00, 0xDEC1, 0xDF81, 0x1F40, 0xDD01, 0x1DC0, 0x1C80, 0xDC41,
    0x1400, 0xD4C1, 0xD581, 0x1540, 0xD701, 0x17C0, 0x1680, 0xD641,
    0xD201, 0x12C0, 0x1380, 0xD341, 0x1100, 0xD1C1, 0xD081, 0x1040,
    0xF001, 0x30C0, 0x3180, 0xF141, 0x3300, 0xF3C1, 0xF281, 0x3240,
    0x3600, 0xF6C1, 0xF781, 0x3740, 0xF501, 0x35C0, 0x3480, 0xF441,
    0x3C00, 0xFCC1, 0xFD81, 0x3D40, 0xFF01, 0x3FC0, 0x3E80, 0xFE41,
    0xFA01, 0x3AC0, 0x3B80, 0xFB41, 0x3900, 0xF9C1, 0xF881, 0x3840,
    0x2800, 0xE8C1, 0xE981, 0x2940, 0xEB01, 0x2BC0, 0x2A80, 0xEA41,
    0xEE01, 0x2EC0, 0x2F80, 0xEF41, 0x2D00, 0xEDC1, 0xEC81, 0x2C40,
    0xE401, 0x24C0, 0x2580, 0xE541, 0x2700, 0xE7C1, 0xE681, 0x2640,
    0x2200, 0xE2C1, 0xE381, 0x2340, 0xE101, 0x21C0, 0x2080, 0xE041,
    0xA001, 0x60C0, 0x6180, 0xA141, 0x6300, 0xA3C1, 0xA281, 0x6240,
    0x6600, 0xA6C1, 0xA781, 0x6740, 0xA501, 0x65C0, 0x6480, 0xA441,
    0x6C00, 0xACC1, 0xAD81, 0x6D40, 0xAF01, 0x6FC0, 0x6E80, 0xAE41,
    0xAA01, 0x6AC0, 0x6B80, 0xAB41, 0x6900, 0xA9C1, 0xA881, 0x6840,
    0x7800, 0xB8C1, 0xB981, 0x7940, 0xBB01, 0x7BC0, 0x7A80, 0xBA41,
    0xBE01, 0x7EC0, 0x7F80, 0xBF41, 0x7D00, 0xBDC1, 0xBC81, 0x7C40,
    0xB401, 0x74C0, 0x7580, 0xB541, 0x7700, 0xB7C1, 0xB681, 0x7640,
    0x7200, 0xB2C1, 0xB381, 0x7340, 0xB101, 0x71C0, 0x7080, 0xB041,
    0x5000, 0x90C1, 0x9181, 0x5140, 0x9301, 0x53C0, 0x5280, 0x9241,
    0x9601, 0x56C0, 0x5780, 0x9741, 0x5500, 0x95C1, 0x9481, 0x5440,
    0x9C01, 0x5CC0, 0x5D80, 0x9D41, 0x5F00, 0x9FC1, 0x9E81, 0x5E40,
    0x5A00, 0x9AC1, 0x9B81, 0x5B40, 0x9901, 0x59C0, 0x5880, 0x9841,
    0x8801, 0x48C0, 0x4980, 0x8941, 0x4B00, 0x8BC1, 0x8A81, 0x4A40,
    0x4E00, 0x8EC1, 0x8F81, 0x4F40, 0x8D01, 0x4DC0, 0x4C80, 0x8C41,
    0x4400, 0x84C1, 0x8581, 0x4540, 0x8701, 0x47C0, 0x4680, 0x8641,
    0x8201, 0x42C0, 0x4380, 0x8341, 0x4100, 0x81C1, 0x8081, 0x4040,
];

// ===================== ENUMS =====================

/// Modbus connection mode
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ModbusMode {
    Rtu,
    Tcp,
}

/// Modbus function codes
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
#[repr(u8)]
pub enum FunctionCode {
    ReadCoils = 0x01,
    ReadDiscreteInputs = 0x02,
    ReadHoldingRegisters = 0x03,
    ReadInputRegisters = 0x04,
    WriteSingleCoil = 0x05,
    WriteSingleRegister = 0x06,
    WriteMultipleCoils = 0x0F,
    WriteMultipleRegisters = 0x10,
}

impl FunctionCode {
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            0x01 => Some(FunctionCode::ReadCoils),
            0x02 => Some(FunctionCode::ReadDiscreteInputs),
            0x03 => Some(FunctionCode::ReadHoldingRegisters),
            0x04 => Some(FunctionCode::ReadInputRegisters),
            0x05 => Some(FunctionCode::WriteSingleCoil),
            0x06 => Some(FunctionCode::WriteSingleRegister),
            0x0F => Some(FunctionCode::WriteMultipleCoils),
            0x10 => Some(FunctionCode::WriteMultipleRegisters),
            _ => None,
        }
    }

    pub fn is_read(&self) -> bool {
        matches!(
            self,
            FunctionCode::ReadCoils
                | FunctionCode::ReadDiscreteInputs
                | FunctionCode::ReadHoldingRegisters
                | FunctionCode::ReadInputRegisters
        )
    }

    pub fn is_coil_operation(&self) -> bool {
        matches!(
            self,
            FunctionCode::ReadCoils
                | FunctionCode::ReadDiscreteInputs
                | FunctionCode::WriteSingleCoil
                | FunctionCode::WriteMultipleCoils
        )
    }
}

// ===================== CONFIG STRUCTS =====================

/// Modbus RTU configuration
#[derive(Debug, Deserialize, Clone)]
pub struct ModbusRtuConfig {
    pub port_name: String,
    pub baud_rate: u32,
    pub data_bits: u8,
    pub stop_bits: String,
    pub parity: String,
    pub slave_id: u8,
    #[serde(default = "default_response_timeout")]
    pub response_timeout_ms: u32,
}

/// Modbus TCP configuration
#[derive(Debug, Deserialize, Clone)]
pub struct ModbusTcpConfig {
    pub host: String,
    pub port: u16,
    pub unit_id: u8,
    #[serde(default = "default_response_timeout")]
    pub response_timeout_ms: u32,
}

fn default_response_timeout() -> u32 {
    1000
}

/// Modbus request parameters
#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct ModbusRequest {
    pub connection_id: String,
    pub function_code: u8,
    pub start_address: u16,
    #[serde(default)]
    pub quantity: u16,
    #[serde(default)]
    pub values: Option<Vec<u16>>,
    #[serde(default)]
    pub coil_values: Option<Vec<bool>>,
}

/// Modbus response
#[derive(Debug, Serialize, Clone)]
pub struct ModbusResponse {
    pub connection_id: String,
    pub transaction_id: u16,
    pub slave_id: u8,
    pub function_code: u8,
    pub start_address: u16,
    pub quantity: u16,
    pub success: bool,
    pub data: Option<Vec<u16>>,
    pub coils: Option<Vec<bool>>,
    pub error_code: Option<u8>,
    pub error_message: Option<String>,
    pub request_frame: Vec<u8>,
    pub response_frame: Vec<u8>,
    pub response_time_ms: u64,
    pub timestamp: u64,
}

/// Modbus poll configuration
#[derive(Debug, Deserialize, Clone)]
pub struct ModbusPollConfig {
    pub connection_id: String,
    pub requests: Vec<ModbusPollRequest>,
    pub interval_ms: u32,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct ModbusPollRequest {
    pub function_code: u8,
    pub start_address: u16,
    pub quantity: u16,
}

/// Connection status event
#[derive(Debug, Serialize, Clone)]
pub struct ModbusConnectionStatus {
    pub connection_id: String,
    pub status: String,
    pub message: Option<String>,
    pub timestamp: u64,
}

// ===================== CRC FUNCTIONS =====================

/// Calculate CRC-16 (Modbus)
pub fn calculate_crc16(data: &[u8]) -> u16 {
    let mut crc: u16 = 0xFFFF;
    for byte in data {
        let index = ((crc ^ (*byte as u16)) & 0xFF) as usize;
        crc = (crc >> 8) ^ CRC_TABLE[index];
    }
    crc
}

/// Verify CRC-16 for received RTU frame
pub fn verify_crc16(frame: &[u8]) -> bool {
    if frame.len() < 4 {
        return false;
    }
    let data_len = frame.len() - 2;
    let calculated = calculate_crc16(&frame[..data_len]);
    let received = u16::from_le_bytes([frame[data_len], frame[data_len + 1]]);
    calculated == received
}

// ===================== FRAME BUILDING =====================

/// Build Modbus RTU request frame
pub fn build_rtu_frame(slave_id: u8, function_code: u8, data: &[u8]) -> Vec<u8> {
    let mut frame = Vec::with_capacity(data.len() + 4);
    frame.push(slave_id);
    frame.push(function_code);
    frame.extend_from_slice(data);
    let crc = calculate_crc16(&frame);
    frame.extend_from_slice(&crc.to_le_bytes());
    frame
}

/// Build Modbus TCP frame with MBAP header
pub fn build_tcp_frame(
    transaction_id: u16,
    unit_id: u8,
    function_code: u8,
    data: &[u8],
) -> Vec<u8> {
    let pdu_length = data.len() + 2; // FC (1) + Unit ID (1) + data
    let mut frame = Vec::with_capacity(pdu_length + 7);

    // MBAP Header (7 bytes)
    frame.extend_from_slice(&transaction_id.to_be_bytes()); // Transaction ID
    frame.extend_from_slice(&[0x00, 0x00]); // Protocol ID (0 = Modbus)
    frame.extend_from_slice(&((pdu_length) as u16).to_be_bytes()); // Length (Unit ID + PDU)
    frame.push(unit_id); // Unit ID

    // PDU
    frame.push(function_code);
    frame.extend_from_slice(data);

    frame
}

/// Build read request data (FC01, FC02, FC03, FC04)
pub fn build_read_request_data(start_address: u16, quantity: u16) -> Vec<u8> {
    let mut data = Vec::with_capacity(4);
    data.extend_from_slice(&start_address.to_be_bytes());
    data.extend_from_slice(&quantity.to_be_bytes());
    data
}

/// Build write single register data (FC06)
pub fn build_write_single_register_data(address: u16, value: u16) -> Vec<u8> {
    let mut data = Vec::with_capacity(4);
    data.extend_from_slice(&address.to_be_bytes());
    data.extend_from_slice(&value.to_be_bytes());
    data
}

/// Build write single coil data (FC05)
pub fn build_write_single_coil_data(address: u16, value: bool) -> Vec<u8> {
    let mut data = Vec::with_capacity(4);
    data.extend_from_slice(&address.to_be_bytes());
    data.extend_from_slice(if value { &[0xFF, 0x00] } else { &[0x00, 0x00] });
    data
}

/// Build write multiple registers data (FC16)
pub fn build_write_multiple_registers_data(start_address: u16, values: &[u16]) -> Vec<u8> {
    let byte_count = (values.len() * 2) as u8;
    let mut data = Vec::with_capacity(5 + values.len() * 2);
    data.extend_from_slice(&start_address.to_be_bytes());
    data.extend_from_slice(&(values.len() as u16).to_be_bytes());
    data.push(byte_count);
    for value in values {
        data.extend_from_slice(&value.to_be_bytes());
    }
    data
}

/// Build write multiple coils data (FC15)
pub fn build_write_multiple_coils_data(start_address: u16, values: &[bool]) -> Vec<u8> {
    let quantity = values.len() as u16;
    let byte_count = ((values.len() + 7) / 8) as u8;
    let mut data = Vec::with_capacity(5 + byte_count as usize);
    data.extend_from_slice(&start_address.to_be_bytes());
    data.extend_from_slice(&quantity.to_be_bytes());
    data.push(byte_count);

    // Pack coils into bytes (LSB first)
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

    data
}

/// Build request data based on function code
pub fn build_request_data(request: &ModbusRequest) -> Result<Vec<u8>, String> {
    let fc = FunctionCode::from_u8(request.function_code)
        .ok_or_else(|| format!("Invalid function code: {}", request.function_code))?;

    match fc {
        FunctionCode::ReadCoils
        | FunctionCode::ReadDiscreteInputs
        | FunctionCode::ReadHoldingRegisters
        | FunctionCode::ReadInputRegisters => {
            if request.quantity == 0 || request.quantity > 125 {
                return Err("Quantity must be between 1 and 125".to_string());
            }
            Ok(build_read_request_data(request.start_address, request.quantity))
        }
        FunctionCode::WriteSingleCoil => {
            let value = request
                .coil_values
                .as_ref()
                .and_then(|v| v.first())
                .copied()
                .unwrap_or(false);
            Ok(build_write_single_coil_data(request.start_address, value))
        }
        FunctionCode::WriteSingleRegister => {
            let value = request
                .values
                .as_ref()
                .and_then(|v| v.first())
                .copied()
                .unwrap_or(0);
            Ok(build_write_single_register_data(request.start_address, value))
        }
        FunctionCode::WriteMultipleCoils => {
            let values = request
                .coil_values
                .as_ref()
                .ok_or("Coil values required for FC15")?;
            if values.is_empty() || values.len() > 1968 {
                return Err("Number of coils must be between 1 and 1968".to_string());
            }
            Ok(build_write_multiple_coils_data(request.start_address, values))
        }
        FunctionCode::WriteMultipleRegisters => {
            let values = request
                .values
                .as_ref()
                .ok_or("Register values required for FC16")?;
            if values.is_empty() || values.len() > 123 {
                return Err("Number of registers must be between 1 and 123".to_string());
            }
            Ok(build_write_multiple_registers_data(request.start_address, values))
        }
    }
}

// ===================== RESPONSE PARSING =====================

/// Parsed Modbus response
pub struct ParsedResponse {
    pub slave_id: u8,
    pub function_code: u8,
    pub data: Option<Vec<u16>>,
    pub coils: Option<Vec<bool>>,
    pub is_exception: bool,
    pub exception_code: Option<u8>,
}

/// Parse Modbus RTU response frame
pub fn parse_rtu_response(frame: &[u8], expected_fc: u8) -> Result<ParsedResponse, String> {
    if frame.len() < 5 {
        return Err("Response too short".to_string());
    }

    if !verify_crc16(frame) {
        return Err("CRC error".to_string());
    }

    let slave_id = frame[0];
    let fc = frame[1];

    // Check for exception response
    if fc & 0x80 != 0 {
        let exception_code = frame[2];
        return Ok(ParsedResponse {
            slave_id,
            function_code: fc & 0x7F,
            data: None,
            coils: None,
            is_exception: true,
            exception_code: Some(exception_code),
        });
    }

    if fc != expected_fc {
        return Err(format!(
            "Function code mismatch: expected {}, got {}",
            expected_fc, fc
        ));
    }

    // Parse data based on function code (excluding CRC)
    let data_frame = &frame[2..frame.len() - 2];
    parse_response_data(fc, data_frame, slave_id)
}

/// Parse Modbus TCP response frame
pub fn parse_tcp_response(frame: &[u8], expected_fc: u8) -> Result<ParsedResponse, String> {
    if frame.len() < 9 {
        return Err("Response too short".to_string());
    }

    // Parse MBAP header
    let _transaction_id = u16::from_be_bytes([frame[0], frame[1]]);
    let _protocol_id = u16::from_be_bytes([frame[2], frame[3]]);
    let _length = u16::from_be_bytes([frame[4], frame[5]]);
    let unit_id = frame[6];
    let fc = frame[7];

    // Check for exception response
    if fc & 0x80 != 0 {
        let exception_code = frame[8];
        return Ok(ParsedResponse {
            slave_id: unit_id,
            function_code: fc & 0x7F,
            data: None,
            coils: None,
            is_exception: true,
            exception_code: Some(exception_code),
        });
    }

    if fc != expected_fc {
        return Err(format!(
            "Function code mismatch: expected {}, got {}",
            expected_fc, fc
        ));
    }

    // Parse data (after MBAP header + FC)
    let data_frame = &frame[8..];
    parse_response_data(fc, data_frame, unit_id)
}

/// Parse response data based on function code
fn parse_response_data(fc: u8, data_frame: &[u8], slave_id: u8) -> Result<ParsedResponse, String> {
    match FunctionCode::from_u8(fc) {
        Some(FunctionCode::ReadCoils) | Some(FunctionCode::ReadDiscreteInputs) => {
            if data_frame.is_empty() {
                return Err("Empty response data".to_string());
            }
            let byte_count = data_frame[0] as usize;
            if data_frame.len() < 1 + byte_count {
                return Err("Incomplete coil data".to_string());
            }

            // Unpack coils from bytes
            let mut coils = Vec::new();
            for i in 0..byte_count {
                let byte_val = data_frame[1 + i];
                for bit in 0..8 {
                    coils.push((byte_val >> bit) & 1 == 1);
                }
            }

            Ok(ParsedResponse {
                slave_id,
                function_code: fc,
                data: None,
                coils: Some(coils),
                is_exception: false,
                exception_code: None,
            })
        }
        Some(FunctionCode::ReadHoldingRegisters) | Some(FunctionCode::ReadInputRegisters) => {
            if data_frame.is_empty() {
                return Err("Empty response data".to_string());
            }
            let byte_count = data_frame[0] as usize;
            if data_frame.len() < 1 + byte_count {
                return Err("Incomplete register data".to_string());
            }

            // Parse registers (big-endian)
            let mut registers = Vec::with_capacity(byte_count / 2);
            for i in (0..byte_count).step_by(2) {
                let value = u16::from_be_bytes([data_frame[1 + i], data_frame[2 + i]]);
                registers.push(value);
            }

            Ok(ParsedResponse {
                slave_id,
                function_code: fc,
                data: Some(registers),
                coils: None,
                is_exception: false,
                exception_code: None,
            })
        }
        Some(FunctionCode::WriteSingleCoil)
        | Some(FunctionCode::WriteSingleRegister)
        | Some(FunctionCode::WriteMultipleCoils)
        | Some(FunctionCode::WriteMultipleRegisters) => {
            // Write responses echo the address and value/quantity
            Ok(ParsedResponse {
                slave_id,
                function_code: fc,
                data: None,
                coils: None,
                is_exception: false,
                exception_code: None,
            })
        }
        None => Err(format!("Unknown function code: {}", fc)),
    }
}

/// Format exception code to human-readable message
pub fn format_exception_error(code: u8) -> String {
    match code {
        0x01 => "Illegal Function".to_string(),
        0x02 => "Illegal Data Address".to_string(),
        0x03 => "Illegal Data Value".to_string(),
        0x04 => "Slave Device Failure".to_string(),
        0x05 => "Acknowledge".to_string(),
        0x06 => "Slave Device Busy".to_string(),
        0x08 => "Memory Parity Error".to_string(),
        0x0A => "Gateway Path Unavailable".to_string(),
        0x0B => "Gateway Target Device Failed to Respond".to_string(),
        _ => format!("Unknown Exception: 0x{:02X}", code),
    }
}

// ===================== TIMING UTILITIES =====================

/// Calculate RTU inter-frame delay (3.5 character times) in microseconds
pub fn calculate_inter_frame_delay_us(baud_rate: u32) -> u64 {
    // 1 character = 11 bits (1 start + 8 data + 1 parity + 1 stop)
    // 3.5 characters = 38.5 bits
    // Minimum is 1750us for baud rates > 19200
    let char_time_us = (11_000_000 / baud_rate) as u64;
    let delay = (char_time_us * 7) / 2; // 3.5 characters
    delay.max(1750) // Minimum 1.75ms for high baud rates
}

/// Calculate expected response length for a request
pub fn calculate_expected_response_length(fc: u8, quantity: u16, mode: ModbusMode) -> usize {
    let base_len = match mode {
        ModbusMode::Rtu => 5, // slave + fc + data + crc(2)
        ModbusMode::Tcp => 9, // MBAP(7) + fc + data
    };

    match FunctionCode::from_u8(fc) {
        Some(FunctionCode::ReadCoils) | Some(FunctionCode::ReadDiscreteInputs) => {
            let byte_count = ((quantity as usize) + 7) / 8;
            base_len + byte_count
        }
        Some(FunctionCode::ReadHoldingRegisters) | Some(FunctionCode::ReadInputRegisters) => {
            base_len + (quantity as usize) * 2
        }
        Some(FunctionCode::WriteSingleCoil) | Some(FunctionCode::WriteSingleRegister) => {
            base_len + 2 // Address + value
        }
        Some(FunctionCode::WriteMultipleCoils) | Some(FunctionCode::WriteMultipleRegisters) => {
            base_len + 2 // Address + quantity
        }
        None => base_len,
    }
}

// ===================== HELPER FUNCTIONS =====================

/// Get current timestamp in milliseconds
pub fn get_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as u64
}

/// Format bytes as hex string for display
pub fn format_hex(data: &[u8]) -> String {
    data.iter()
        .map(|b| format!("{:02X}", b))
        .collect::<Vec<_>>()
        .join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crc16() {
        // Test vector: slave 1, FC03, address 0, quantity 10
        let data = [0x01, 0x03, 0x00, 0x00, 0x00, 0x0A];
        let crc = calculate_crc16(&data);
        assert_eq!(crc, 0x04C5); // Expected CRC
    }

    #[test]
    fn test_build_rtu_frame() {
        let data = build_read_request_data(0, 10);
        let frame = build_rtu_frame(1, 0x03, &data);
        assert_eq!(frame.len(), 8); // slave + fc + addr(2) + qty(2) + crc(2)
        assert_eq!(frame[0], 1); // Slave ID
        assert_eq!(frame[1], 0x03); // FC
    }

    #[test]
    fn test_build_tcp_frame() {
        let data = build_read_request_data(0, 10);
        let frame = build_tcp_frame(1, 1, 0x03, &data);
        assert_eq!(frame.len(), 12); // MBAP(7) + fc + data(4)
        assert_eq!(frame[0], 0); // Transaction ID high
        assert_eq!(frame[1], 1); // Transaction ID low
        assert_eq!(frame[6], 1); // Unit ID
        assert_eq!(frame[7], 0x03); // FC
    }

    #[test]
    fn test_verify_crc() {
        let frame = [0x01, 0x03, 0x00, 0x00, 0x00, 0x0A, 0xC5, 0xCD];
        assert!(verify_crc16(&frame));
    }
}
