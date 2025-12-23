# Modbus Master - Documentation

## Overview
Modbus Master implementation trong TermiPro hỗ trợ 2 chế độ:
- **RTU**: Giao tiếp qua Serial port
- **TCP/IP**: Giao tiếp qua TCP socket (port 502)

Hỗ trợ đầy đủ 8 Function Codes chuẩn Modbus và polling.

## Files

### Backend (Rust)
| File | Mô tả |
|------|-------|
| `src-tauri/src/modbus.rs` | Module xử lý Modbus protocol (CRC-16, frame building, parsing) |
| `src-tauri/src/lib.rs` | Tauri commands, state management, polling logic |

### Frontend (Vue.js)
| File | Mô tả |
|------|-------|
| `src/components/ModbusTab.vue` | Component chính cho Modbus tab |
| `src/stores/tabStore.js` | Tab state với `createModbusTabState()` |
| `src/components/TabBar.vue` | Tab title và icon cho Modbus |
| `src/components/NewTabModal.vue` | Option chọn Modbus |
| `src/App.vue` | Event handlers và translations |

---

## Structs & Types

### ModbusMode
```rust
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum ModbusMode {
    Rtu,
    Tcp,
}
```

### ModbusRtuConfig
```rust
#[derive(Debug, Deserialize, Clone)]
pub struct ModbusRtuConfig {
    pub port_name: String,
    pub baud_rate: u32,
    pub data_bits: u8,
    pub stop_bits: String,
    pub parity: String,
    pub slave_id: u8,
    pub response_timeout_ms: u32,
}
```

### ModbusTcpConfig
```rust
#[derive(Debug, Deserialize, Clone)]
pub struct ModbusTcpConfig {
    pub host: String,
    pub port: u16,
    pub unit_id: u8,
    pub response_timeout_ms: u32,
}
```

### ModbusRequest
```rust
#[derive(Debug, Deserialize, Clone)]
pub struct ModbusRequest {
    pub connection_id: String,
    pub function_code: u8,
    pub start_address: u16,
    pub quantity: u16,
    pub values: Option<Vec<u16>>,      // For write register operations
    pub coil_values: Option<Vec<bool>>, // For write coil operations
}
```

### ModbusResponse
```rust
#[derive(Debug, Serialize, Clone)]
pub struct ModbusResponse {
    pub connection_id: String,
    pub transaction_id: u16,
    pub slave_id: u8,
    pub function_code: u8,
    pub success: bool,
    pub data: Option<Vec<u16>>,        // Register values
    pub coils: Option<Vec<bool>>,      // Coil values
    pub error_code: Option<u8>,
    pub error_message: Option<String>,
    pub request_frame: Vec<u8>,
    pub response_frame: Vec<u8>,
    pub response_time_ms: u64,
    pub timestamp: u64,
}
```

### ModbusPollConfig
```rust
#[derive(Debug, Deserialize, Clone)]
pub struct ModbusPollConfig {
    pub connection_id: String,
    pub requests: Vec<ModbusPollRequest>,
    pub interval_ms: u32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ModbusPollRequest {
    pub function_code: u8,
    pub start_address: u16,
    pub quantity: u16,
}
```

---

## State Management

### ModbusState
```rust
pub struct ModbusState {
    connections: Arc<Mutex<HashMap<String, ModbusConnectionHandle>>>,
    runtime: tokio::runtime::Runtime,
}
```

### ModbusConnectionHandle
```rust
pub struct ModbusConnectionHandle {
    pub mode: ModbusMode,
    pub config: ModbusConnectionConfig,
    pub serial_port: Option<Arc<Mutex<Box<dyn SerialPort>>>>,
    pub tcp_tx: Option<mpsc::Sender<Vec<u8>>>,
    pub tcp_rx: Option<Arc<Mutex<mpsc::Receiver<Vec<u8>>>>>,
    pub transaction_id: AtomicU16,  // For TCP transaction ID
    pub polling_active: AtomicBool,
}
```

---

## Function Codes

| FC | Name | Type | Description |
|----|------|------|-------------|
| 01 | Read Coils | Read | Đọc trạng thái coils (1 bit) |
| 02 | Read Discrete Inputs | Read | Đọc discrete inputs (1 bit) |
| 03 | Read Holding Registers | Read | Đọc holding registers (16 bit) |
| 04 | Read Input Registers | Read | Đọc input registers (16 bit) |
| 05 | Write Single Coil | Write | Ghi 1 coil |
| 06 | Write Single Register | Write | Ghi 1 register |
| 15 | Write Multiple Coils | Write | Ghi nhiều coils |
| 16 | Write Multiple Registers | Write | Ghi nhiều registers |

---

## CRC-16 Modbus

### Algorithm
```rust
pub fn calculate_crc16(data: &[u8]) -> u16 {
    let mut crc: u16 = 0xFFFF;
    for byte in data {
        crc ^= *byte as u16;
        for _ in 0..8 {
            if (crc & 0x0001) != 0 {
                crc = (crc >> 1) ^ 0xA001;
            } else {
                crc >>= 1;
            }
        }
    }
    crc
}
```

### Verification
```rust
pub fn verify_crc16(frame: &[u8]) -> bool {
    if frame.len() < 4 { return false; }
    let data = &frame[..frame.len() - 2];
    let received_crc = u16::from_le_bytes([
        frame[frame.len() - 2],
        frame[frame.len() - 1]
    ]);
    calculate_crc16(data) == received_crc
}
```

---

## Frame Structure

### RTU Frame
```
[Slave ID][Function Code][Data...][CRC Low][CRC High]
   1 byte      1 byte      N bytes   1 byte   1 byte
```

**Example FC03 Request (Read 10 registers from address 0):**
```
01 03 00 00 00 0A C5 CD
│  │  └────┘ └────┘ └───┘
│  │    │      │     CRC
│  │    │      Quantity (10)
│  │    Start Address (0)
│  Function Code (03)
Slave ID (1)
```

### TCP Frame (MBAP Header)
```
[Transaction ID][Protocol ID][Length][Unit ID][Function Code][Data...]
    2 bytes        2 bytes    2 bytes  1 byte     1 byte      N bytes
```

**Example FC03 Request:**
```
00 01 00 00 00 06 01 03 00 00 00 0A
└───┘ └───┘ └───┘  │  │  └────┘ └───┘
  │     │     │    │  │    │      Quantity
  │     │     │    │  │    Start Address
  │     │     │    │  Function Code
  │     │     │    Unit ID
  │     │     Length (6 bytes after this)
  │     Protocol ID (0 = Modbus)
  Transaction ID
```

---

## Tauri Commands

### modbus_rtu_connect
```rust
#[tauri::command]
fn modbus_rtu_connect(
    app: AppHandle,
    state: State<ModbusState>,
    config: ModbusRtuConfig,
) -> Result<String, String>
```

**Flow:**
1. Mở serial port với config
2. Set read timeout
3. Tạo `ModbusConnectionHandle` với mode RTU
4. Lưu vào HashMap
5. Return connection_id

### modbus_tcp_connect
```rust
#[tauri::command]
fn modbus_tcp_connect(
    app: AppHandle,
    state: State<ModbusState>,
    config: ModbusTcpConfig,
) -> Result<String, String>
```

**Flow:**
1. Spawn async task kết nối TCP
2. Tạo channels (tx, rx) cho communication
3. Spawn read task để nhận response
4. Lưu `ModbusConnectionHandle` vào HashMap
5. Return connection_id

### modbus_disconnect
```rust
#[tauri::command]
fn modbus_disconnect(
    state: State<ModbusState>,
    connection_id: String,
) -> Result<String, String>
```

### modbus_request
```rust
#[tauri::command]
fn modbus_request(
    app: AppHandle,
    state: State<ModbusState>,
    request: ModbusRequest,
) -> Result<ModbusResponse, String>
```

**Flow:**
1. Build request data based on function code
2. Build frame (RTU with CRC or TCP with MBAP header)
3. Send frame
4. Wait for response with timeout
5. Parse response
6. Emit `modbus-response` event
7. Return `ModbusResponse`

### modbus_start_polling
```rust
#[tauri::command]
fn modbus_start_polling(
    app: AppHandle,
    state: State<ModbusState>,
    config: ModbusPollConfig,
) -> Result<String, String>
```

**Polling Loop:**
```
loop {
    1. Check if polling_active flag is still true
    2. For each request in config.requests:
       a. Build and send request
       b. Wait for response
       c. Emit "modbus-poll-data" event
       d. Small delay (50ms) between requests
    3. Wait interval_ms before next cycle
}
```

### modbus_stop_polling
```rust
#[tauri::command]
fn modbus_stop_polling(
    state: State<ModbusState>,
    connection_id: String,
) -> Result<String, String>
```

---

## Events

### modbus-response
Emitted sau mỗi request thành công/thất bại.

```javascript
{
  connection_id: "tab-1",
  transaction_id: 1,
  slave_id: 1,
  function_code: 3,
  success: true,
  data: [100, 200, 300],  // Register values
  coils: null,
  error_code: null,
  error_message: null,
  request_frame: [1, 3, 0, 0, 0, 3, ...],
  response_frame: [1, 3, 6, 0, 100, 0, 200, 0, 300, ...],
  response_time_ms: 45,
  timestamp: 1703123456789
}
```

### modbus-poll-data
Emitted khi polling nhận được data.

```javascript
{
  connection_id: "tab-1",
  function_code: 3,
  start_address: 0,
  data: [100, 200, 300],
  coils: null,
  timestamp: 1703123456789
}
```

### modbus-status
Emitted khi trạng thái connection thay đổi.

```javascript
{
  connection_id: "tab-1",
  status: "connected",  // "connected", "disconnected", "error"
  message: null
}
```

---

## Frontend Integration

### Tab State (tabStore.js)
```javascript
function createModbusTabState(id) {
  return reactive({
    id,
    connectionType: 'modbus',
    connectionId: id,
    isConnected: false,

    // Mode
    mode: 'rtu',  // 'rtu' | 'tcp'

    // RTU Config
    selectedPort: '',
    baudRate: 9600,
    dataBits: 8,
    stopBits: '1',
    parity: 'even',  // Modbus default
    slaveId: 1,

    // TCP Config
    host: 'localhost',
    port: 502,
    unitId: 1,

    // Common
    responseTimeout: 1000,

    // Request
    functionCode: 3,
    startAddress: 0,
    quantity: 10,
    writeValues: [],
    coilValues: [],

    // Data Display
    dataFormat: 'unsigned',  // unsigned, signed, hex, float32, binary
    registerData: [],
    coilData: [],

    // Transaction Log
    transactionLog: [],
    maxLogEntries: 100,

    // Polling
    pollingEnabled: false,
    pollingInterval: 1000,

    // Status
    connectionStatus: 'idle',
    statusMessage: null,
    lastResponseTime: null,
  });
}
```

### Invoke Commands
```javascript
import { invoke } from "@tauri-apps/api/core";

// RTU Connect
await invoke("modbus_rtu_connect", {
  config: {
    port_name: "/dev/ttyUSB0",
    baud_rate: 9600,
    data_bits: 8,
    stop_bits: "1",
    parity: "even",
    slave_id: 1,
    response_timeout_ms: 1000,
  }
});

// TCP Connect
await invoke("modbus_tcp_connect", {
  config: {
    host: "192.168.1.100",
    port: 502,
    unit_id: 1,
    response_timeout_ms: 1000,
  }
});

// Send Request
const response = await invoke("modbus_request", {
  request: {
    connection_id: tabId,
    function_code: 3,
    start_address: 0,
    quantity: 10,
    values: null,
    coil_values: null,
  }
});

// Start Polling
await invoke("modbus_start_polling", {
  config: {
    connection_id: tabId,
    requests: [{ function_code: 3, start_address: 0, quantity: 10 }],
    interval_ms: 1000,
  }
});

// Stop Polling
await invoke("modbus_stop_polling", { connectionId: tabId });

// Disconnect
await invoke("modbus_disconnect", { connectionId: tabId });
```

### Listen Events (App.vue)
```javascript
import { listen } from "@tauri-apps/api/event";

// Response listener
await listen("modbus-response", (event) => {
  const { connection_id, success, data, coils, ... } = event.payload;
  const tab = getModbusTabByConnectionId(connection_id);
  if (tab) {
    // Update transaction log
    tab.transactionLog.unshift({ ... });

    // Update data display
    if (success && data) {
      tab.registerData = data.map((value, index) => ({
        address: tab.startAddress + index,
        value,
        rawHex: value.toString(16).padStart(4, '0'),
      }));
    }
  }
});

// Poll data listener
await listen("modbus-poll-data", (event) => {
  const { connection_id, data, coils } = event.payload;
  // Update register/coil data in real-time
});
```

---

## Data Format Conversion

```javascript
function formatValue(value, format) {
  switch (format) {
    case 'unsigned':
      return value.toString();
    case 'signed':
      return value > 32767 ? (value - 65536).toString() : value.toString();
    case 'hex':
      return '0x' + value.toString(16).toUpperCase().padStart(4, '0');
    case 'binary':
      return value.toString(2).padStart(16, '0');
    case 'float32':
      // Requires 2 consecutive registers
      return value.toString();
  }
}
```

---

## Exception Codes

| Code | Name | Description |
|------|------|-------------|
| 01 | Illegal Function | Function code không được hỗ trợ |
| 02 | Illegal Data Address | Địa chỉ không hợp lệ |
| 03 | Illegal Data Value | Giá trị không hợp lệ |
| 04 | Slave Device Failure | Lỗi thiết bị slave |
| 05 | Acknowledge | Slave đã nhận, đang xử lý |
| 06 | Slave Device Busy | Slave đang bận |

**Exception Response:**
- Function code = Original FC + 0x80
- Followed by exception code

---

## Tab Title Display

| Trạng thái | Hiển thị |
|------------|----------|
| Chưa kết nối (RTU) | `Modbus RTU` |
| Chưa kết nối (TCP) | `Modbus TCP` |
| Đã kết nối RTU | `MB:cu.usbserial` |
| Đã kết nối TCP | `MB:192.168.1.100:502` |

**Icon:** Lightning bolt (⚡) màu cam `#f59e0b`

---

## UI Layout

```
┌─────────────────────────────────────────────────────┐
│  SIDEBAR (280px)        │  MAIN AREA               │
├─────────────────────────┼───────────────────────────┤
│  [RTU] [TCP] toggle     │  DATA TABLE              │
│                         │  ┌───────────────────┐   │
│  ── RTU/TCP Config ──   │  │ Addr │ Value │Hex │   │
│  Port/Host, Baud/Port   │  │ 0000 │ 1234  │04D2│   │
│  [Connect/Disconnect]   │  └───────────────────┘   │
│                         │                          │
│  ── Modbus Settings ──  │  Format: [U][S][H][F][B] │
│  Slave/Unit ID          │                          │
│  Timeout (ms)           │  ────────────────────    │
│                         │                          │
│  ── Request ──          │  TRANSACTION LOG         │
│  Function Code dropdown │  ┌───────────────────┐   │
│  Start Address          │  │ 14:32:05 FC03 OK  │   │
│  Quantity               │  │ TX: 01 03 00 00.. │   │
│  [Read/Write]           │  │ RX: 01 03 14 00.. │   │
│                         │  │ 45ms              │   │
│  ── Polling ──          │  └───────────────────┘   │
│  Interval (ms)          │                          │
│  [Start] [Stop]         │                          │
└─────────────────────────┴───────────────────────────┘
```

---

## Testing

### Test với Modbus Simulator

**diagslave (RTU):**
```bash
# Linux/macOS
./diagslave -m rtu /dev/ttyUSB0 -b 9600 -p even

# Sau đó connect từ TermiPro với cùng port settings
```

**diagslave (TCP):**
```bash
./diagslave -m tcp -p 502

# Connect từ TermiPro: localhost:502
```

### Test Polling
1. Connect đến Modbus slave
2. Set Function Code = 03, Address = 0, Quantity = 10
3. Set Polling Interval = 1000ms
4. Click Start
5. Data table sẽ update mỗi giây

---

## Known Issues & TODOs

### Pending Improvements
- [ ] Float32 format (2 registers) với byte order selection
- [ ] Import/Export poll configurations
- [ ] Logging to file
- [ ] Slave mode (Modbus Slave simulator)
- [ ] Scheduled requests với different intervals

### Fixed Issues
- [x] TCP polling không hoạt động - Fixed: Đợi response trong polling loop
- [x] Tab title không hiển thị cho Modbus - Fixed: Thêm case trong TabBar

---

## Dependencies

### Rust (Cargo.toml)
```toml
[dependencies]
tokio = { version = "1", features = ["sync", "time", "net", "rt-multi-thread"] }
parking_lot = "0.12"
serialport = "4.3"
serde = { version = "1", features = ["derive"] }
```

### Frontend
- Vue 3 Composition API
- @tauri-apps/api v2
