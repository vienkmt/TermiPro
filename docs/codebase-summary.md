# TermiPro Codebase Summary

## Overview

TermiPro is a professional desktop serial communication application built with Tauri (Rust + Vue.js 3). The application supports 6 communication protocols: Serial, TCP Client/Server, Modbus Master/Slave, and MQTT.

**Key Metrics**:
- Total Files: 35
- Total Tokens: ~155k
- Frontend: Vue.js 3 (13 components)
- Backend: Rust (3,111 lines in lib.rs)
- Build Tool: Vite + Tauri 2

---

## Project Structure

```
TermiPro/
├── src/                          # Frontend (Vue.js 3)
│   ├── App.vue                   # Main orchestrator (2,106 lines)
│   ├── main.js                   # Entry point
│   ├── components/               # Tab components & modals
│   │   ├── SerialTab.vue         # Serial port connection
│   │   ├── TcpClientTab.vue      # TCP client
│   │   ├── TcpServerTab.vue      # TCP server
│   │   ├── ModbusTab.vue         # Modbus master
│   │   ├── ModbusSlaveTab.vue    # Modbus slave
│   │   ├── MqttTab.vue           # MQTT client
│   │   ├── TabBar.vue            # Tab management
│   │   ├── NewTabModal.vue       # Create new tab dialog
│   │   ├── ConfirmDialog.vue     # Confirmation dialogs
│   │   └── UpdateModal.vue       # App update notification
│   └── stores/
│       └── tabStore.js           # Shared reactive store (475 lines)
│
├── src-tauri/                    # Backend (Rust)
│   ├── src/
│   │   ├── lib.rs               # Main implementation (3,111 lines)
│   │   ├── main.rs              # Entry point
│   │   ├── modbus.rs            # Modbus protocol
│   │   ├── modbus_slave.rs      # Modbus slave implementation
│   │   └── mqtt.rs              # MQTT client
│   ├── Cargo.toml               # Rust dependencies
│   ├── tauri.conf.json          # Tauri configuration
│   └── icons/                   # Application icons
│
├── index.html                    # HTML template
├── package.json                  # Node dependencies
├── vite.config.js               # Vite configuration
├── Cargo.toml                    # Tauri workspace config
└── docs/                         # Documentation
```

---

## Frontend Architecture

### Core Components

#### **App.vue** (Main Orchestrator)
- **Size**: 2,106 lines (16,137 tokens)
- **Responsibilities**:
  - Manages i18n (Vietnamese & English)
  - Tab lifecycle management
  - Global event listeners for all protocols
  - Update checks and notifications
  - 9+ event types: serial-data, tcp-data, tcp-status, modbus-data, modbus-status, mqtt-data, mqtt-status, port-disconnected, etc.

- **Key Features**:
  - Event batching using RAF (requestAnimationFrame)
  - Terminal limit: 500 entries per tab
  - Dynamic component rendering based on tab type
  - Real-time status updates

#### **tabStore.js** (State Management)
- **Size**: 475 lines
- **Pattern**: Custom reactive store (NOT Vuex/Pinia)
- **Manages**:
  - Tab collection (max 8 tabs)
  - Connection states for each tab
  - Terminal data history
  - Terminal display options (text/hex, auto-scroll)
  - Auto-send configuration

**Connection Types**:
```javascript
CONNECTION_TYPES = {
  SERIAL: "serial",
  TCP_CLIENT: "tcp_client",
  TCP_SERVER: "tcp_server",
  MODBUS: "modbus",
  MODBUS_SLAVE: "modbus_slave",
  MQTT: "mqtt"
}
```

#### **Tab Components** (Protocol-Specific)

| Component | Lines | Features |
|-----------|-------|----------|
| SerialTab.vue | 1,800+ | Port selection, baud rate, data bits, stop bits, parity |
| TcpClientTab.vue | 1,200+ | Host/port config, reconnect status, connection state |
| TcpServerTab.vue | 1,600+ | Listening port, client list, echo mode, multi-client support |
| ModbusTab.vue | 1,900+ | RTU/TCP modes, read/write functions, register management |
| ModbusSlaveTab.vue | 1,700+ | Slave ID, register simulation, event logging |
| MqttTab.vue | 1,300+ | Broker config, topic subscribe/publish, message history |

### Performance Optimizations

1. **Event Batching**: RAF-based batching prevents UI jank
2. **Terminal Limit**: Hard limit of 500 entries keeps memory usage constant
3. **v-show for Tabs**: Components not destroyed, just hidden
4. **Lazy Component Loading**: Tabs loaded only when created

### Styling

- **CSS Variables**: Sky blue accent (#0ea5e9)
- **Fonts**:
  - UI: Plus Jakarta Sans (bundled offline)
  - Terminal: JetBrains Mono (bundled offline)
- **Theme**: Light mode only
- **Custom Dropdowns**: Replace native selects with professional styling

---

## Backend Architecture

### Rust Implementation (lib.rs)

**Size**: 3,111 lines (21,591 tokens - 13.9% of codebase)

#### State Managers

```rust
// Serial port state
pub struct SerialState {
    ports: HashMap<String, Arc<Mutex<Box<dyn SerialPort>>>>,
    running: HashMap<String, bool>,
}

// TCP state (clients + servers)
pub struct TcpState {
    clients: HashMap<String, TcpClientHandle>,
    servers: HashMap<String, TcpServerHandle>,
    runtime: tokio::runtime::Runtime, // 4 worker threads
}

// Modbus state
pub struct ModbusState {
    connections: HashMap<String, ModbusConnectionHandle>,
    runtime: tokio::runtime::Runtime, // 2 worker threads
}

// Modbus Slave state
pub struct ModbusSlaveState {
    servers: HashMap<String, ModbusSlaveHandle>,
    runtime: tokio::runtime::Runtime, // 2 worker threads
}

// MQTT state
pub struct MqttState {
    connections: HashMap<String, MqttConnectionHandle>,
    runtime: tokio::runtime::Runtime, // 2 worker threads
}
```

#### Event System

**15+ Event Types**:
- `serial-data`: Raw bytes from serial port
- `port-disconnected`: Serial port unplugged
- `tcp-data`: Data from TCP client/server
- `tcp-client-status`: Connection state changes
- `tcp-server-status`: Server startup/shutdown
- `tcp-server-client-event`: Client connected/disconnected
- `modbus-data`: Register read results
- `modbus-status`: Connection state
- `modbus-slave-status`: Slave server state
- `mqtt-data`: Published message
- `mqtt-status`: Connection state
- `mqtt-subscribed`: Topic subscription result

### Tauri Commands

**Total**: 47 commands across 6 protocols

#### Serial Commands (5)
- `list_serial_ports()` - Enumerate USB serial ports
- `open_port(config)` - Open with full configuration
- `close_port(port_name)` - Close connection
- `send_data(port_name, data, is_hex)` - Send raw bytes
- `is_port_open(port_name)` - Status check

#### TCP Client Commands (4)
- `tcp_client_connect(config)`
- `tcp_client_disconnect(connection_id)`
- `tcp_client_send(connection_id, data, is_hex)`
- `is_tcp_client_connected(connection_id)`

#### TCP Server Commands (7)
- `tcp_server_start(config)`
- `tcp_server_stop(server_id)`
- `tcp_server_send_to_client(server_id, client_id, data, is_hex)`
- `tcp_server_broadcast(server_id, data, is_hex)`
- `tcp_server_get_clients(server_id)`
- `tcp_server_disconnect_client(server_id, client_id)`
- `tcp_server_set_echo_mode(server_id, enabled)`

#### Modbus Master Commands (7)
- `modbus_connect(config)` - RTU or TCP
- `modbus_disconnect(connection_id)`
- `modbus_read_coils(connection_id, address, count)`
- `modbus_read_discrete_inputs(connection_id, address, count)`
- `modbus_read_holding_registers(connection_id, address, count)`
- `modbus_read_input_registers(connection_id, address, count)`
- `modbus_write_registers(connection_id, address, values)`

#### Modbus Slave Commands (13)
- `modbus_slave_start(config)` - RTU or TCP
- `modbus_slave_stop(server_id)`
- `modbus_slave_set_coil(server_id, address, value)`
- `modbus_slave_set_register(server_id, address, value)`
- And 9 more register/coil management commands

#### MQTT Commands (7)
- `mqtt_connect(config)`
- `mqtt_disconnect(connection_id)`
- `mqtt_publish(connection_id, topic, payload, qos, retain)`
- `mqtt_subscribe(connection_id, topic, qos)`
- `mqtt_unsubscribe(connection_id, topic)`
- `mqtt_get_subscriptions(connection_id)`
- `mqtt_is_connected(connection_id)`

### Data Structures

#### Configuration Types
```rust
// Serial
pub struct SerialConfig {
    pub port_name: String,
    pub baud_rate: u32,
    pub data_bits: u8,
    pub stop_bits: String,  // "1", "1.5", "2"
    pub parity: String,      // "None", "Odd", "Even"
    pub dtr: bool,
    pub rts: bool,
}

// TCP Client
pub struct TcpClientConfig {
    pub host: String,
    pub port: u16,
    pub connection_id: String,
}

// TCP Server
pub struct TcpServerConfig {
    pub port: u16,
    pub bind_address: String,
    pub server_id: String,
    pub max_clients: u32,
}

// Modbus (Enum)
pub enum ModbusConnectionConfig {
    Rtu { port_name, baud_rate, slave_id, response_timeout_ms },
    Tcp { host, port, unit_id, response_timeout_ms }
}
```

#### Event Payloads
```rust
// Serial
pub struct SerialData {
    pub port_name: String,
    pub data: Vec<u8>,
    pub timestamp: u64,
}

// TCP
pub struct TcpData {
    pub connection_id: String,
    pub client_id: Option<String>,
    pub data: Vec<u8>,
    pub timestamp: u64,
}

pub struct TcpConnectionStatus {
    pub connection_id: String,
    pub status: String,  // "connected", "disconnected", "error", etc.
    pub message: Option<String>,
    pub timestamp: u64,
}
```

### Async Runtime Configuration

- **TCP**: 4 worker threads (handles many concurrent connections)
- **Modbus**: 2 worker threads (sequential protocol)
- **Modbus Slave**: 2 worker threads (sequential responses)
- **MQTT**: 2 worker threads (single connection per instance)

### Error Handling Patterns

1. **Serial Port Errors**: Emit `port-disconnected` event on unplug
2. **TCP Connection Errors**: Auto-reconnect with exponential backoff
3. **TCP Write Failures**: Retry with configurable delays
4. **Modbus Timeout**: Return error, allow retry
5. **MQTT Disconnects**: Auto-reconnect loop

### Thread Safety

- **Arc<Mutex>**: For shared state across async tasks
- **parking_lot::Mutex**: Faster than std::sync::Mutex
- **atomic types**: For simple flags (running, polling_active)
- **tokio::sync::mpsc**: For task communication

---

## Dependencies

### Rust (Cargo.toml)

| Crate | Version | Purpose |
|-------|---------|---------|
| tauri | 2.0 | Framework & IPC |
| tokio | 1 | Async runtime |
| serialport | 4.3 | Serial port access |
| rumqttc | 0.24 | MQTT client |
| serde | 1 | Serialization |
| serde_json | 1 | JSON parsing |
| parking_lot | 0.12 | Better mutexes |

### Frontend (package.json)

| Package | Version | Purpose |
|---------|---------|---------|
| vue | ^3.5.13 | UI framework |
| @tauri-apps/api | ^2 | IPC & Window APIs |
| @tauri-apps/plugin-updater | ^2.9.0 | Auto-update |
| @tauri-apps/plugin-opener | ^2 | Open links |
| @fontsource/plus-jakarta-sans | ^5.2.8 | UI font |
| @fontsource/jetbrains-mono | ^5.2.8 | Terminal font |
| vite | ^6.0.3 | Build tool |

---

## Communication Flow

### Frontend → Backend
```
User Action (click, input)
    ↓
Vue Component Handler
    ↓
invoke() Tauri Command
    ↓
Rust Command Handler
    ↓
State Update + Async Task
```

### Backend → Frontend
```
Async Task (serial read, TCP recv, etc)
    ↓
Event Emission (app.emit())
    ↓
Frontend Event Listener (listen())
    ↓
tabStore Update
    ↓
Vue Component Reactivity
    ↓
UI Update
```

---

## Code Organization Principles

1. **Separation of Concerns**: Each protocol isolated in separate tabs
2. **Shared State**: tabStore manages cross-tab communication
3. **Async-First**: All I/O is async, UI never blocks
4. **Memory Bounded**: Terminal history capped at 500 entries
5. **Graceful Degradation**: Tab isolation prevents one failure affecting others

---

## Build & Development

### Development
```bash
npm install
npm run tauri dev
```

### Production Build
```bash
npm run tauri build
```

### Output
- **macOS**: .app bundle in src-tauri/target/release/bundle/macos/
- **Windows**: .exe installer in src-tauri/target/release/bundle/msi/
- **Linux**: .deb/.appimage in src-tauri/target/release/bundle/

---

## Key Challenges & Solutions

### 1. Multi-Tab Concurrency
**Challenge**: Multiple tabs accessing same Rust state
**Solution**: HashMap with unique connection_id, Arc<Mutex> for thread-safe access

### 2. Terminal Memory Growth
**Challenge**: Thousands of messages slow UI
**Solution**: Hard limit of 500 entries per tab, circular buffer pattern

### 3. Reconnect Logic
**Challenge**: TCP/Modbus need automatic reconnection
**Solution**: Loop with exponential backoff, configurable timeouts

### 4. Cross-Protocol Events
**Challenge**: Frontend needs unified event handling
**Solution**: Tauri event system with consistent payload structure

### 5. Serial Port Plugin Compatibility
**Challenge**: Serial ports behave differently on macOS/Windows/Linux
**Solution**: abstraction layer using serialport crate

---

## Future Optimization Opportunities

1. **WebSocket Support**: Add WebSocket protocol alongside TCP
2. **Scripting**: Add Lua/JavaScript for automation
3. **Data Logging**: Persist terminal history to database
4. **Custom Protocols**: Allow plugins for custom protocols
5. **Performance Metrics**: Add bandwidth/latency monitoring
6. **Keyboard Shortcuts**: More keyboard navigation
7. **Search in Terminal**: Find text in message history
8. **Export**: Save terminal to CSV/JSON

---

## File Size Breakdown

| Component | Tokens | % |
|-----------|--------|---|
| src-tauri/src/lib.rs | 21,591 | 13.9% |
| src/components/ModbusSlaveTab.vue | 16,718 | 10.8% |
| src/App.vue | 16,137 | 10.4% |
| src/components/SerialTab.vue | 14,786 | 9.5% |
| src/components/ModbusTab.vue | 13,378 | 8.6% |
| Other Components | ~57k | 37.2% |
| **Total** | **~155k** | 100% |

---

**Last Updated**: 2025-12-29
**Repomix Generated**: Yes
**Coverage**: 100% (35 files analyzed)
