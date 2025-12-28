# TermiPro System Architecture

## Table of Contents
1. [Architecture Overview](#architecture-overview)
2. [Component Architecture](#component-architecture)
3. [Communication Patterns](#communication-patterns)
4. [Data Flow](#data-flow)
5. [State Management](#state-management)
6. [Protocol Integration](#protocol-integration)
7. [Deployment Architecture](#deployment-architecture)
8. [Performance Architecture](#performance-architecture)
9. [Scalability Considerations](#scalability-considerations)

---

## Architecture Overview

### High-Level Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                    TermiPro Desktop App                     │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  ┌──────────────────────┐      ┌──────────────────────┐   │
│  │   Frontend Layer     │      │   User Interface     │   │
│  │  (Vue.js 3)          │      │  (Light Theme)       │   │
│  │                      │      │                      │   │
│  │ • App.vue (2106 L)   │      │ • Tab Management     │   │
│  │ • 6 Tab Components   │      │ • Terminal Display   │   │
│  │ • tabStore.js        │      │ • Configuration UI   │   │
│  └──────────────────────┘      └──────────────────────┘   │
│           ↕                              ↕                  │
│    Tauri IPC Commands              Tauri Events            │
│    (invoke/listen)                                          │
│           ↕                              ↕                  │
│  ┌──────────────────────────────────────────────────┐     │
│  │      Tauri Runtime (Version 2)                    │     │
│  │  • Window Management                             │     │
│  │  • IPC Channel (Command/Event)                   │     │
│  │  • Plugin System (Updater, Opener, Process)     │     │
│  └──────────────────────────────────────────────────┘     │
│           ↕                              ↕                  │
│  ┌──────────────────────────────────────────────────┐     │
│  │      Backend Layer                                │     │
│  │  (Rust 1.70+)                                     │     │
│  │                                                   │     │
│  │  State Managers:                                 │     │
│  │  • SerialState                                   │     │
│  │  • TcpState (4 workers)                          │     │
│  │  • ModbusState (2 workers)                       │     │
│  │  • ModbusSlaveState (2 workers)                  │     │
│  │  • MqttState (2 workers)                         │     │
│  │                                                   │     │
│  │  Protocols:                                      │     │
│  │  • Serial (serialport crate)                     │     │
│  │  • TCP (tokio::net)                              │     │
│  │  • Modbus (custom impl)                          │     │
│  │  • MQTT (rumqttc)                                │     │
│  └──────────────────────────────────────────────────┘     │
│           ↓           ↓              ↓           ↓          │
│     ┌──────────┐  ┌────────┐  ┌───────────┐ ┌────────┐   │
│     │Serial    │  │TCP/IP  │  │Modbus RTU │ │MQTT    │   │
│     │Ports     │  │Sockets │  │/TCP       │ │Broker  │   │
│     └──────────┘  └────────┘  └───────────┘ └────────┘   │
│                                                             │
└─────────────────────────────────────────────────────────────┘
                          ↓
        ┌──────────────────────────────────────┐
        │    Operating System                   │
        │  (macOS, Windows, Linux)              │
        │                                       │
        │ • USB Device Enumeration              │
        │ • Serial Port Drivers                 │
        │ • TCP/IP Network Stack                │
        └──────────────────────────────────────┘
```

### Architecture Principles

1. **Separation of Concerns**: Each protocol isolated in separate modules
2. **Reactive UI**: Vue.js reactivity updates when backend state changes
3. **Async-First**: All I/O is async, never blocks the UI
4. **Event-Driven**: Communication via Tauri events, not polling
5. **Thread-Safe**: Arc<Mutex> for shared state across async tasks
6. **Resource Bounded**: Terminal limited to 500 messages, channel capacity bounded

---

## Component Architecture

### Frontend Component Hierarchy

```
App.vue (Main Orchestrator)
├── Header
│   ├── App Logo
│   ├── App Title
│   └── Connection Counter
├── TabBar
│   ├── Tab List (1-8 tabs)
│   ├── Active Tab Indicator
│   └── New Tab Button (+)
├── Tab Content Area
│   ├── SerialTab
│   ├── TcpClientTab
│   ├── TcpServerTab
│   ├── ModbusTab
│   ├── ModbusSlaveTab
│   └── MqttTab
├── Modal Layer
│   ├── NewTabModal (create new connection)
│   ├── ConfirmDialog (close tab, disconnect)
│   └── UpdateModal (app update notification)
└── Footer
    ├── Input Field
    ├── Send Button
    ├── Clear Button
    └── Mode Toggle (Text/Hex)
```

### Tab Component Architecture

Each tab component follows the same structure:

```
[Protocol]Tab.vue
├── Configuration Section
│   ├── Connection Parameters (host, port, baud rate, etc.)
│   ├── Status Indicator
│   └── Connect/Disconnect Button
├── Terminal Display
│   ├── Message History
│   ├── TX/RX Statistics
│   ├── Auto-Scroll Toggle
│   ├── Clear Button
│   └── Text/Hex Toggle
└── Input Section (shared with parent)
    ├── Message Input Field
    ├── Send Button
    └── Mode Indicators
```

### Data Store Architecture

```
tabStore.js (Reactive Store)
├── tabs: Array<Tab>
│   ├── Tab {
│   │   ├── id: String (unique identifier)
│   │   ├── type: "serial" | "tcp_client" | "tcp_server" | "modbus" | "modbus_slave" | "mqtt"
│   │   ├── name: String (user-defined)
│   │   ├── config: Object (protocol-specific)
│   │   ├── isConnected: Boolean
│   │   ├── terminal: Array<Message> (max 500)
│   │   ├── stats: { txCount, rxCount, bytesSent, bytesReceived }
│   │   ├── displayMode: "text" | "hex"
│   │   └── autoSend: { enabled, interval }
│   │   }
│   └── ...more tabs
├── activeTabId: String
├── actions: {
│   ├── createTab()
│   ├── closeTab()
│   ├── addMessage()
│   ├── setConnectionState()
│   ├── clearTerminal()
│   └── updateStats()
│   }
└── getters: {
    ├── activeTab
    ├── connectedTabs
    └── terminalHistory
    }
```

---

## Communication Patterns

### Frontend → Backend: Command Pattern

```
User Action → Component Handler → invoke() → Tauri IPC → Backend Command Handler → Operation
```

**Example: Opening Serial Port**
```javascript
// Frontend
await invoke('open_port', {
  port_name: '/dev/ttyUSB0',
  baud_rate: 9600,
  data_bits: 8,
  stop_bits: '1',
  parity: 'None',
  dtr: false,
  rts: false
});
```

```rust
// Backend
#[tauri::command]
fn open_port(
  app: AppHandle,
  state: State<SerialState>,
  port_name: String,
  baud_rate: u32,
  data_bits: u8,
  stop_bits: String,
  parity: String,
  dtr: bool,
  rts: bool,
) -> Result<String, String> {
  // Process command, start async task
  // Return result or error
}
```

### Backend → Frontend: Event Pattern

```
Async Task → Data Available → app.emit(event) → Tauri Event → Frontend Listener → tabStore Update → UI Reactive Update
```

**Example: Serial Data Received**
```rust
// Backend
let data = SerialData {
    port_name: "COM3".to_string(),
    data: vec![72, 101, 108, 108, 111],
    timestamp: get_timestamp(),
};
app.emit("serial-data", data)?;
```

```javascript
// Frontend
await listen('serial-data', (event) => {
  const { port_name, data, timestamp } = event.payload;
  tabStore.addMessage('tab-1', {
    type: 'rx',
    data: new Uint8Array(data),
    timestamp
  });
});
```

### Event Type Catalog

**Serial Events** (1-way from backend):
- `serial-data`: Raw bytes received
- `port-disconnected`: Port unplugged or closed

**TCP Events** (1-way from backend):
- `tcp-data`: Data from remote peer
- `tcp-client-status`: Connection state
- `tcp-server-status`: Server state change
- `tcp-server-client-event`: Client connected/disconnected

**Modbus Events** (1-way from backend):
- `modbus-data`: Register read result
- `modbus-status`: Connection state

**Modbus Slave Events** (1-way from backend):
- `modbus-slave-status`: Server state

**MQTT Events** (1-way from backend):
- `mqtt-data`: Published message
- `mqtt-status`: Connection state
- `mqtt-subscribed`: Subscription confirmed

---

## Data Flow

### Typical Connection Lifecycle

```
1. User creates new tab (New Tab Modal)
   ↓
2. Frontend invokes backend command
   ├─ Serial: open_port(config)
   ├─ TCP Client: tcp_client_connect(config)
   ├─ TCP Server: tcp_server_start(config)
   ├─ Modbus: modbus_connect(config)
   ├─ Modbus Slave: modbus_slave_start(config)
   └─ MQTT: mqtt_connect(config)
   ↓
3. Backend creates state handle and starts async task
   ├─ Validate configuration
   ├─ Acquire resource (port, socket, etc.)
   ├─ Spawn async read/write loops
   └─ Register in state HashMap
   ↓
4. Backend emits "connected" status event
   ↓
5. Frontend receives event, updates tab state
   ├─ Set isConnected = true
   ├─ Hide connect button
   └─ Show disconnect button
   ↓
6. Data arrives (user sends or device receives)
   ├─ Backend async task reads/writes data
   └─ Emits [protocol]-data event
   ↓
7. Frontend receives event
   ├─ Adds message to terminal
   ├─ Updates statistics
   ├─ Triggers auto-scroll if enabled
   └─ Maintains 500-message limit
   ↓
8. User disconnects or connection fails
   ├─ Frontend invokes disconnect command
   ├─ Backend sets running flag = false
   ├─ Async task exits and cleans up
   └─ Emits disconnected event
```

### Message Flow Sequence Diagram

```
┌─────────────┐                     ┌──────────┐               ┌──────────────┐
│  User/UI    │                     │ Frontend │               │   Backend    │
└─────────────┘                     └──────────┘               └──────────────┘
      │                                  │                            │
      │ Click Connect Button             │                            │
      │──────────────────────────────────>│                            │
      │                                  │ invoke('open_port', ...)   │
      │                                  │───────────────────────────>│
      │                                  │                            │ Start async task
      │                                  │                            │ (serial read loop)
      │                                  │ <─────────────────────────│ return OK
      │                                  │                            │
      │                                  │ listen('serial-data')      │
      │                                  │ (setup event handler)      │
      │                                  │                            │
      │                                  │  emit('serial-data', msg)  │
      │                                  │ <─────────────────────────│
      │                                  │ (data from device)         │
      │ Update terminal display          │                            │
      │ <──────────────────────────────────                           │
      │                                  │                            │
      │ Type message, click Send         │                            │
      │──────────────────────────────────>│                            │
      │                                  │ invoke('send_data', msg)  │
      │                                  │───────────────────────────>│
      │                                  │                            │ Send via serial port
      │                                  │ <─────────────────────────│ return OK
      │                                  │                            │
      │ Display sent message             │                            │
      │ <──────────────────────────────────                           │
```

---

## State Management

### Frontend State (tabStore.js)

```javascript
// Central reactive store - NOT Vuex/Pinia, custom implementation
const state = reactive({
  tabs: [],
  activeTabId: null
});

// State structure per tab
const tab = {
  id: 'tab-1',
  type: 'serial',
  name: 'Arduino COM3',

  // Configuration (protocol-specific)
  config: {
    portName: '/dev/ttyUSB0',
    baudRate: 9600,
    dataBits: 8,
    stopBits: '1',
    parity: 'None'
  },

  // Connection state
  isConnected: false,
  isReconnecting: false,
  connectionError: null,

  // Terminal data
  terminal: [
    { id: 1, type: 'rx', data: [72, 105], text: 'Hi', timestamp: 1234567890 },
    { id: 2, type: 'tx', data: [72, 105], text: 'Hi', timestamp: 1234567891 }
  ],

  // Statistics
  stats: {
    txCount: 10,
    rxCount: 23,
    bytesSent: 256,
    bytesReceived: 1024
  },

  // Display options
  displayMode: 'text', // 'text' or 'hex'
  autoScroll: true,

  // Auto-send configuration
  autoSend: {
    enabled: false,
    interval: 1000,
    data: '',
    isHex: false,
    count: 0
  }
};
```

### Backend State (Rust)

**SerialState**:
```rust
pub struct SerialState {
    ports: HashMap<String, Arc<Mutex<Box<dyn SerialPort>>>>,
    running: HashMap<String, bool>
}
```

**TcpState**:
```rust
pub struct TcpState {
    clients: Arc<Mutex<HashMap<String, TcpClientHandle>>>,
    servers: Arc<Mutex<HashMap<String, TcpServerHandle>>>,
    runtime: tokio::runtime::Runtime
}
```

**ModbusState**:
```rust
pub struct ModbusState {
    connections: HashMap<String, ModbusConnectionHandle>,
    runtime: tokio::runtime::Runtime
}
```

**State Initialization in main()**:
```rust
#[cfg_attr(mobile, tauri::mobile::entry)]
pub fn run() {
    tauri::Builder::default()
        .manage(SerialState::default())
        .manage(TcpState::default())
        .manage(ModbusState::default())
        .manage(ModbusSlaveState::default())
        .manage(MqttState::default())
        // ... setup commands and listeners
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

---

## Protocol Integration

### Protocol Architecture

```
┌────────────────────────────────────────────┐
│        Frontend Component                   │
│  (e.g., SerialTab.vue)                     │
└────────────────────┬───────────────────────┘
                     │ invoke()/listen()
                     ↓
┌────────────────────────────────────────────┐
│      Tauri Command Handler                  │
│  (e.g., open_port)                         │
├────────────────────────────────────────────┤
│                                             │
│  1. Validate input                          │
│  2. Acquire resource (port/socket/broker)  │
│  3. Create state handle                     │
│  4. Spawn async task(s)                     │
│  5. Register in HashMap                     │
│                                             │
└────────────────────┬───────────────────────┘
                     │
            ┌────────┴────────┐
            ↓                 ↓
    ┌────────────────┐  ┌──────────────────────┐
    │ Async Read     │  │ Async Write          │
    │ Loop           │  │ Loop                 │
    ├────────────────┤  ├──────────────────────┤
    │ 1. Read data   │  │ 1. Receive from ch.  │
    │ 2. Buffer      │  │ 2. Encode data       │
    │ 3. Parse       │  │ 3. Write to device   │
    │ 4. Emit event  │  │ 4. Retry on fail     │
    │ 5. Loop or die │  │ 5. Loop or die       │
    └────────┬───────┘  └──────────┬───────────┘
             │                      │
             └──────────┬───────────┘
                        │
                        ↓
              ┌──────────────────────┐
              │  app.emit(event)     │
              │  Send to frontend    │
              └──────────────────────┘
                        │
                        ↓ Tauri IPC
        ┌───────────────────────────────┐
        │ Frontend Event Listener        │
        │ Update tabStore               │
        │ Vue Reactivity → UI Update    │
        └───────────────────────────────┘
```

### Each Protocol Integration

| Protocol | Module | State Manager | Async Runtime | Key Features |
|----------|--------|---------------|---------------|--------------|
| Serial | lib.rs | SerialState | sync (threads) | Port enumeration, read/write |
| TCP Client | lib.rs | TcpState | 4 workers | Auto-reconnect, retry logic |
| TCP Server | lib.rs | TcpState | 4 workers | Multi-client, broadcast, echo |
| Modbus Master | modbus.rs | ModbusState | 2 workers | RTU/TCP, CRC validation |
| Modbus Slave | modbus_slave.rs | ModbusSlaveState | 2 workers | Register simulation |
| MQTT | mqtt.rs | MqttState | 2 workers | QoS, retain, authentication |

---

## Deployment Architecture

### Build Outputs

```
npm run tauri build
    ↓
┌─────────────────────────────────────────────┐
│     Vite + Tauri Build Process              │
├─────────────────────────────────────────────┤
│                                             │
│  1. Build Frontend (Vue.js → JavaScript)    │
│  2. Bundle Assets                           │
│  3. Compile Backend (Rust → Machine Code)  │
│  4. Link Binary                             │
│  5. Package Distribution Format             │
│                                             │
└─────────────────────────────────────────────┘
                        │
        ┌───────────────┼───────────────┐
        ↓               ↓               ↓
    ┌────────┐   ┌─────────────┐  ┌────────┐
    │ macOS  │   │  Windows    │  │ Linux  │
    │ .app   │   │ .msi/.exe   │  │.deb    │
    │Bundle  │   │ Installer   │  │Package │
    └────────┘   └─────────────┘  └────────┘
```

### Application Folder Structure (Runtime)

```
TermiPro.app (macOS)
├── Contents/
│   ├── MacOS/
│   │   └── termipro (executable)
│   ├── Resources/
│   │   ├── app/               (bundled web assets)
│   │   │   ├── index.html
│   │   │   ├── js/
│   │   │   ├── css/
│   │   │   └── fonts/         (offline fonts)
│   │   ├── termipro.svg
│   │   └── icon.icns
│   └── Info.plist

TermiPro.exe (Windows)
├── termipro.exe               (executable)
├── resources/
│   ├── app/                   (bundled assets)
│   └── ...
└── ...

termipro (AppImage / Linux)
├── termipro                   (executable)
├── resources/
│   └── app/                   (bundled assets)
└── ...
```

---

## Performance Architecture

### Memory Management

```
Frontend Memory:
├── Vue Component Tree: ~10-20MB
├── Terminal History (500 msgs): ~5-10MB
├── Active Tab Data: ~1-5MB
├── Event Listeners: <1MB
└── Total: ~20-50MB

Backend Memory:
├── State Managers (hashmaps): ~2-5MB
├── Serial Port Buffers: <1MB
├── TCP Connections: 1-10MB (depends on client count)
├── Async Task Overhead: ~5MB
└── Total: ~10-25MB

Overall Expected: 30-75MB for typical usage
```

### Event Processing

```
High-frequency data scenario (100 Hz serial data):
├─ Backend receives: 100 messages/sec
├─ Event batching (RAF): Batch every 16.67ms
├─ Frontend processes: Max 7 messages/batch
├─ Terminal update: Append + limit to 500
├─ UI re-render: ~60 FPS
└─ Result: Smooth UI, no jank
```

### Latency Profile

```
User Action → UI Update:
├─ Click connect button: 10-50ms
├─ Tauri IPC overhead: 1-5ms
├─ Backend command execution: 5-100ms (depends on operation)
├─ Event emission: 0.5-1ms
├─ Frontend event handler: 5-20ms
└─ Vue reactivity update: 16ms (next frame)
└─ Total: ~30-200ms (appears instantaneous)

Data arrival → Display:
├─ Serial/Network receive: 0-1ms (whenever data available)
├─ Backend event emission: 0.5-1ms
├─ Frontend listener invocation: 1-5ms
├─ tabStore update: 5-10ms
├─ Vue reactivity trigger: <1ms
├─ RAF batch accumulation: 0-16.67ms
└─ DOM update + render: 16ms
└─ Total: ~20-50ms (user perceives as real-time)
```

---

## Scalability Considerations

### Scaling Limits

| Dimension | Current | Limit | Impact |
|-----------|---------|-------|--------|
| Concurrent Tabs | Max 8 | 8 | By design |
| Terminal Messages | 500/tab | 500 | Memory bounded |
| TCP Connections | Unlimited | System FD limit | ~1000+ typically |
| MQTT Subscriptions | Unlimited | Broker limit | 100+ typical |
| Modbus Slaves | Unlimited | Port/addr space | 10+ typical |
| Data Rate | 921.6kbps serial | Protocol limit | No app bottleneck |

### Future Optimization Opportunities

1. **Message Compression**: Store terminal history compressed
2. **Circular Buffer**: Replace array slice with true circular buffer
3. **Worker Threads**: Offload terminal processing to worker
4. **Virtual Scrolling**: Only render visible messages
5. **Lazy Tab Loading**: Create components only when needed
6. **WebAssembly**: Move hot paths to WASM for speed

---

## Security Architecture

### Input Validation

```
User Input → Frontend Validation → Backend Validation → Safe Operation
   ↑                    ↑                    ↑
   │                    │                    │
   Type checking     Length limits      Range checking
   Pattern matching  Format validation  State verification
```

### IPC Security

- **One-way Commands**: Frontend sends commands, backend processes
- **Event-based Response**: Backend emits events, frontend listens
- **No Shell Execution**: All commands are typed, no eval()
- **Input Sanitization**: Hex strings, URLs, hostnames validated

### Data Privacy

- **No Telemetry**: No usage tracking
- **No Data Logging**: Terminal history not saved by default
- **No Network Calls**: All processing local
- **No Credentials Storage**: User enters credentials each session

---

## Disaster Recovery

### Fault Tolerance

```
Scenario: Serial Port Unplugged
├─ OS detects removal
├─ Serial read returns error
├─ Backend emits port-disconnected event
├─ Frontend catches event, updates UI
└─ User can reconnect when ready

Scenario: TCP Connection Lost
├─ Read or write fails
├─ Auto-reconnect timer triggers
├─ Attempts 3 reconnections with backoff
├─ If all fail, emit disconnected event
└─ User can manually reconnect

Scenario: Backend Task Panics
├─ tokio runtime catches panic
├─ Task exits cleanly
├─ HashMap entry removed on drop
├─ Frontend never receives response
└─ User sees timeout, can retry

Scenario: Frontend Crashes
├─ User reloads window
├─ Backend connections remain active
├─ Backend continues collecting events
└─ New frontend session receives buffered events
```

---

## Monitoring & Observability

### Logging Strategy

```
Backend (Rust):
├─ Debug logs: Command execution, async task lifecycle
├─ Info logs: Connection events, major state changes
├─ Warn logs: Retry attempts, timeouts, recoveries
├─ Error logs: Fatal errors, resource exhaustion
└─ Trace logs: (optional) Low-level operation details

Frontend (Vue.js):
├─ Console logs: User actions, Tauri invocation
├─ Event logs: Received events, state updates
├─ Error logs: Try/catch failures, promise rejections
└─ Performance logs: (optional) Component render times
```

### Metrics Worth Tracking

- Connection uptime per tab
- Message throughput (msg/sec)
- UI frame rate (fps)
- Memory usage over time
- Async task count
- Event queue depth
- Error rate

---

## Appendix: Architecture Decision Records (ADR)

### ADR-001: Why Arc<Mutex> Instead of Channels?

**Decision**: Use Arc<Mutex<HashMap>> for state instead of channel-per-connection.

**Rationale**:
- Multiple Tauri commands need to query/modify same state
- Channels work for single-reader, but we have multiple command handlers
- Arc<Mutex> allows clean HashMap insert/remove semantics
- Parking_lot::Mutex faster than std::sync::Mutex

### ADR-002: Why 500 Terminal Limit?

**Decision**: Hard-code 500 messages maximum per terminal.

**Rationale**:
- Prevents unbounded memory growth
- 500 messages = ~5-10MB (acceptable)
- Users rarely scroll past 500 messages
- Performance constant instead of O(n)

### ADR-003: Why Tokio Runtime per Protocol?

**Decision**: Create separate Tokio runtime for TCP (4 workers), Modbus (2), MQTT (2).

**Rationale**:
- TCP handles concurrent connections → needs many workers
- Modbus/MQTT sequential → 2 workers sufficient
- Isolation prevents one protocol starving another
- Easier to tune per-protocol performance

### ADR-004: Why Custom Store Instead of Pinia?

**Decision**: Implement custom tabStore.js instead of Pinia.

**Rationale**:
- Simple reactive needs, no complex mutations
- Avoid heavy dependency for simple use case
- Easier debugging (direct property access)
- Smaller bundle size
- Custom patterns clearer for team understanding

---

**Last Updated**: 2025-12-29
**Version**: 1.0
**Reviewed By**: Architecture Team
