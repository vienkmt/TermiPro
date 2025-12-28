# TermiPro Code Standards & Codebase Structure

## Table of Contents
1. [Codebase Structure](#codebase-structure)
2. [Naming Conventions](#naming-conventions)
3. [Code Organization](#code-organization)
4. [Frontend Standards](#frontend-standards)
5. [Backend Standards](#backend-standards)
6. [Error Handling](#error-handling)
7. [Performance Guidelines](#performance-guidelines)
8. [Testing Approach](#testing-approach)
9. [Documentation Standards](#documentation-standards)

---

## Codebase Structure

### Project Layout

```
TermiPro/
├── src/                              # Frontend source (Vue.js)
│   ├── App.vue                       # Main application orchestrator
│   ├── main.js                       # Vue app entry point
│   ├── components/                   # Vue components
│   │   ├── SerialTab.vue            # Serial protocol UI
│   │   ├── TcpClientTab.vue         # TCP client protocol UI
│   │   ├── TcpServerTab.vue         # TCP server protocol UI
│   │   ├── ModbusTab.vue            # Modbus master protocol UI
│   │   ├── ModbusSlaveTab.vue       # Modbus slave protocol UI
│   │   ├── MqttTab.vue              # MQTT client protocol UI
│   │   ├── TabBar.vue               # Tab management UI
│   │   ├── ConfirmDialog.vue        # Confirmation dialogs
│   │   ├── UpdateModal.vue          # App update notification
│   │   └── NewTabModal.vue          # New tab creation dialog
│   └── stores/
│       └── tabStore.js              # Shared reactive state store
│
├── src-tauri/                        # Backend source (Rust)
│   ├── src/
│   │   ├── lib.rs                   # Main Tauri commands & serial logic
│   │   ├── main.rs                  # Tauri app entry point
│   │   ├── modbus.rs                # Modbus master implementation
│   │   ├── modbus_slave.rs          # Modbus slave implementation
│   │   └── mqtt.rs                  # MQTT client implementation
│   ├── Cargo.toml                   # Rust dependencies
│   ├── tauri.conf.json              # Tauri configuration
│   ├── build.rs                     # Build script
│   └── icons/                       # App icons
│
├── docs/                             # Documentation
│   ├── codebase-summary.md          # Codebase overview
│   ├── code-standards.md            # This file
│   ├── system-architecture.md       # Architecture details
│   ├── project-overview-pdr.md      # Project requirements
│   ├── tcp-client.md                # TCP client protocol guide
│   ├── tcp-server.md                # TCP server protocol guide
│   ├── modbus.md                    # Modbus protocol guide
│   ├── new-tab-modal.md             # Tab modal documentation
│   ├── update_docs.md               # Update system
│   └── helps/                       # User guides
│       ├── getting-started.md       # Quick start guide
│       ├── serial.md                # Serial guide
│       ├── tcp-client.md            # TCP client guide
│       ├── tcp-server.md            # TCP server guide
│       ├── modbus-master.md         # Modbus guide
│       ├── modbus-slave.md          # Modbus slave guide
│       └── mqtt.md                  # MQTT guide
│
├── package.json                      # Node.js dependencies
├── vite.config.js                    # Vite build config
├── index.html                        # HTML template
├── Cargo.toml                        # Tauri workspace config
├── CLAUDE.md                         # Project instructions
└── README.md                         # Project overview
```

### File Naming Conventions

| Type | Pattern | Example |
|------|---------|---------|
| Vue Component | PascalCase | `SerialTab.vue`, `TabBar.vue` |
| Vue Store | camelCase | `tabStore.js` |
| Rust Module | snake_case | `modbus.rs`, `modbus_slave.rs` |
| Rust Struct | PascalCase | `SerialConfig`, `TcpClientHandle` |
| Rust Function | snake_case | `list_serial_ports()`, `tcp_client_connect()` |
| Rust Const | UPPER_SNAKE_CASE | `TCP_CLIENT_MAX_RETRIES` |
| Documentation | kebab-case | `code-standards.md`, `tcp-client.md` |

---

## Naming Conventions

### Frontend (Vue.js)

#### Component Names
```javascript
// ✓ Good: PascalCase
SerialTab, TcpClientTab, ModbusTab, TabBar

// ✗ Bad: camelCase or snake_case
serialTab, serial_tab, tcpClientTab
```

#### Variable Names
```javascript
// ✓ Good: camelCase for variables/functions
const connectionId = "tab-1";
const isConnected = true;
const handleConnect = () => {};

// ✗ Bad: PascalCase for variables
const ConnectionId = "tab-1";
const IsConnected = true;
```

#### Event Names
```javascript
// ✓ Good: kebab-case for custom events
emit('connection-status', { ... });
listen('serial-data', (event) => { ... });

// ✗ Bad: camelCase
emit('connectionStatus', { ... });
```

#### Configuration Keys (Tauri Events)
```javascript
// ✓ Good: snake_case in event payloads (matches Rust)
{
  connection_id: "tab-1",
  port_name: "/dev/ttyUSB0",
  is_hex: true
}

// ✗ Bad: camelCase
{
  connectionId: "tab-1",
  portName: "/dev/ttyUSB0",
  isHex: true
}
```

### Backend (Rust)

#### Struct Names (PascalCase)
```rust
// ✓ Good
pub struct SerialConfig { ... }
pub struct TcpClientHandle { ... }
pub struct ModbusConnectionConfig { ... }

// ✗ Bad
pub struct serial_config { ... }
pub struct TcpClient { ... }
```

#### Function Names (snake_case)
```rust
// ✓ Good: Commands
#[tauri::command]
fn list_serial_ports() { ... }
fn tcp_client_connect(config: TcpClientConfig) { ... }

// ✗ Bad
#[tauri::command]
fn listSerialPorts() { ... }
fn TCPClientConnect() { ... }
```

#### Constants (UPPER_SNAKE_CASE)
```rust
// ✓ Good
const TCP_CLIENT_MAX_RETRIES: u32 = 3;
const TERMINAL_MESSAGE_LIMIT: usize = 500;
const MODBUS_RESPONSE_TIMEOUT_MS: u64 = 1000;

// ✗ Bad
const TcpClientMaxRetries: u32 = 3;
const tcp_client_max_retries: u32 = 3;
```

#### Field Names in Structs (snake_case)
```rust
// ✓ Good
#[derive(Debug, Deserialize)]
pub struct TcpClientConfig {
    pub host: String,
    pub port: u16,
    pub connection_id: String,
}

// ✗ Bad
#[derive(Debug, Deserialize)]
pub struct TcpClientConfig {
    pub Host: String,
    pub Port: u16,
    pub ConnectionId: String,
}
```

---

## Code Organization

### Frontend Module Organization

#### App.vue Structure
```vue
<script setup>
// 1. Imports
import { ref, computed, provide, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { useTabStore } from "./stores/tabStore";

// 2. State definitions
const appVersion = ref("");
const tabs = ref([]);
const activeTabId = ref(null);

// 3. Computed properties
const activeTab = computed(() => {
  return tabs.value.find(tab => tab.id === activeTabId.value);
});

// 4. Methods
const createTab = async (type) => { ... };
const closeTab = async (id) => { ... };
const handleSerialData = async (event) => { ... };

// 5. Lifecycle hooks
onMounted(async () => { ... });
onUnmounted(() => { ... });

// 6. Provide for child components
provide("activeTab", activeTab);
</script>

<template>
  <!-- 1. Header -->
  <!-- 2. Tab bar -->
  <!-- 3. Main content -->
  <!-- 4. Modals -->
</template>
```

#### Component Structure
```vue
<script setup>
// Same order as above
import { ... } from "vue";
import { invoke } from "@tauri-apps/api/core";

const props = defineProps({
  tabId: String,
  tabData: Object
});

const emit = defineEmits(['status-change', 'data-received']);

const state = ref({ ... });
const isConnected = computed(() => state.value.connected);

const connect = async () => { ... };
const disconnect = async () => { ... };
</script>

<template>
  <div class="tab-container">
    <!-- Template -->
  </div>
</template>

<style scoped>
/* Component-specific styles */
.tab-container {
  /* ... */
}
</style>
```

### Backend Module Organization

#### lib.rs Structure
```rust
// 1. Module declarations
mod modbus;
mod modbus_slave;
mod mqtt;

// 2. Imports
use parking_lot::Mutex;
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Emitter, Manager, State};
use tokio::sync::mpsc;

// 3. Data structures (grouped by concern)
// --- Serial structures ---
#[derive(Debug, Serialize, Clone)]
pub struct PortInfo { ... }

#[derive(Debug, Deserialize, Clone)]
pub struct SerialConfig { ... }

// --- TCP structures ---
#[derive(Debug, Deserialize, Clone)]
pub struct TcpClientConfig { ... }

// --- State managers ---
pub struct SerialState { ... }
pub struct TcpState { ... }

// 4. Constants
const TCP_CLIENT_MAX_RETRIES: u32 = 3;
const TERMINAL_MESSAGE_LIMIT: usize = 500;

// 5. Command implementations
#[tauri::command]
fn list_serial_ports() -> Result<Vec<PortInfo>, String> { ... }

#[tauri::command]
fn open_port(state: State<SerialState>, config: SerialConfig) { ... }

// 6. Helper functions
fn parse_hex_string(data: &str) -> Result<Vec<u8>, String> { ... }
fn emit_serial_data(app: &AppHandle, data: SerialData) { ... }

// 7. Main function
#[cfg_attr(mobile, tauri::mobile::entry)]
pub fn run() { ... }
```

#### Module Organization (modbus.rs, mqtt.rs)
```rust
// 1. Imports at top
use parking_lot::Mutex;
use std::sync::Arc;
use tauri::{AppHandle, Emitter};

// 2. Structs
pub struct ModbusConnectionHandle { ... }
pub struct ModbusResponse { ... }

// 3. Trait implementations
impl ModbusConnectionHandle {
    pub fn new() -> Self { ... }
    pub fn connect() -> Result<(), String> { ... }
}

// 4. Helper functions
fn crc16_checksum(data: &[u8]) -> u16 { ... }
fn parse_modbus_response(data: &[u8]) -> Result<ModbusResponse, String> { ... }

// 5. Event emission functions
async fn emit_modbus_data(app: &AppHandle, response: ModbusResponse) { ... }
```

---

## Frontend Standards

### Vue Component Guidelines

#### 1. Composition API Usage
```javascript
// ✓ Good: Using Composition API consistently
<script setup>
import { ref, computed, onMounted } from "vue";

const count = ref(0);
const doubled = computed(() => count.value * 2);

const increment = () => count.value++;

onMounted(() => {
  console.log("Component mounted");
});
</script>
```

#### 2. Reactive State
```javascript
// ✓ Good: ref for primitive, reactive for objects
const connectionStatus = ref("disconnected");
const tabData = reactive({
  name: "Serial Tab",
  isConnected: false,
  config: {}
});

// ✓ Good: computed for derived state
const statusColor = computed(() => {
  return tabData.isConnected ? "green" : "red";
});
```

#### 3. Props & Emits
```javascript
// ✓ Good: Type-safe props
const props = defineProps({
  tabId: {
    type: String,
    required: true
  },
  tabName: {
    type: String,
    default: "Unnamed Tab"
  },
  config: {
    type: Object,
    default: () => ({})
  }
});

// ✓ Good: Explicit emits
const emit = defineEmits(['connected', 'disconnected', 'error']);

const handleConnect = async () => {
  emit('connected', { tabId: props.tabId });
};
```

#### 4. Tauri IPC
```javascript
// ✓ Good: Error handling for invoke
const connectSerial = async () => {
  try {
    const result = await invoke('open_port', {
      portName: config.portName,
      baudRate: config.baudRate
    });
    console.log('Connected:', result);
  } catch (error) {
    console.error('Connection failed:', error);
    emit('error', { message: error });
  }
};

// ✓ Good: Cleanup event listeners
const unlistenData = await listen('serial-data', (event) => {
  handleSerialData(event.payload);
});

onUnmounted(() => {
  unlistenData();
});
```

#### 5. Event Handling
```javascript
// ✓ Good: Consistent event payload structure
listen('serial-data', (event) => {
  const { port_name, data, timestamp } = event.payload;
  addToTerminal({
    type: 'rx',
    data: data,
    timestamp: timestamp,
    port: port_name
  });
});

// ✓ Good: Multiple listeners consolidated
const setupListeners = async () => {
  await listen('serial-data', handleSerialData);
  await listen('port-disconnected', handlePortDisconnected);
  await listen('serial-status', handleStatusChange);
};
```

### Styling Guidelines

#### 1. CSS Variables
```css
/* ✓ Good: Using CSS variables for theming */
:root {
  --color-primary: #0ea5e9;  /* Sky blue */
  --color-bg: #ffffff;
  --color-text: #1f2937;
  --color-border: #e5e7eb;
  --font-family-ui: "Plus Jakarta Sans", sans-serif;
  --font-family-mono: "JetBrains Mono", monospace;
}

/* Component styles */
.button {
  background-color: var(--color-primary);
  color: white;
  font-family: var(--font-family-ui);
}

.terminal {
  font-family: var(--font-family-mono);
}
```

#### 2. Component Scoping
```vue
<style scoped>
/* ✓ Good: Scoped styles prevent conflicts */
.serial-tab {
  /* Only applies to this component */
}

/* ✓ Good: BEM naming for clarity */
.serial-tab__header { }
.serial-tab__content { }
.serial-tab__footer { }

/* ✓ Good: State-based classes */
.serial-tab.is-connected { }
.serial-tab.is-reconnecting { }
```

#### 3. Responsive Design
```css
/* ✓ Good: Mobile-first approach */
.sidebar {
  width: 100%;
}

@media (min-width: 768px) {
  .sidebar {
    width: 320px;
    position: fixed;
  }
}

/* ✓ Good: CSS Grid for layout */
.app-layout {
  display: grid;
  grid-template-columns: 320px 1fr;
  grid-template-rows: auto 1fr auto;
  height: 100vh;
}
```

---

## Backend Standards

### Rust Code Guidelines

#### 1. Error Handling
```rust
// ✓ Good: Explicit error messages
#[tauri::command]
fn open_port(config: SerialConfig) -> Result<String, String> {
    let port = serialport::new(&config.port_name, config.baud_rate)
        .map_err(|e| format!("Failed to open port {}: {}", config.port_name, e))?;

    Ok("Port opened successfully".to_string())
}

// ✓ Good: Error context at each level
async fn read_serial_loop(port: Arc<Mutex<Box<dyn SerialPort>>>) {
    loop {
        let mut buffer = [0u8; 256];

        match port.lock().read_exact(&mut buffer) {
            Ok(n) => {
                emit_serial_data(&app, SerialData {
                    data: buffer[..n].to_vec(),
                    timestamp: get_timestamp(),
                });
            }
            Err(e) => {
                warn!("Serial read error: {}", e);
                emit_port_disconnected(&app, PortDisconnected {
                    reason: format!("Read error: {}", e),
                });
                break;
            }
        }
    }
}

// ✗ Bad: Generic errors
fn open_port() -> Result<String, String> {
    let port = serialport::new(port_name, baud_rate)?;
    Ok("ok".to_string())
}
```

#### 2. State Management
```rust
// ✓ Good: Thread-safe state with Arc<Mutex>
pub struct TcpState {
    clients: Arc<Mutex<HashMap<String, TcpClientHandle>>>,
    servers: Arc<Mutex<HashMap<String, TcpServerHandle>>>,
    runtime: tokio::runtime::Runtime,
}

// ✓ Good: Immutable access pattern
impl TcpState {
    fn get_client(&self, id: &str) -> Option<TcpClientHandle> {
        self.clients.lock().get(id).cloned()
    }

    fn register_client(&self, id: String, handle: TcpClientHandle) {
        self.clients.lock().insert(id, handle);
    }
}

// ✗ Bad: Inconsistent synchronization
pub struct TcpState {
    clients: Vec<TcpClientHandle>,  // Not thread-safe
    mutex_clients: Mutex<Vec<...>>, // Inconsistent pattern
}
```

#### 3. Async/Await Patterns
```rust
// ✓ Good: Async loops with proper cleanup
#[tauri::command]
async fn tcp_client_connect(
    state: State<'_, TcpState>,
    config: TcpClientConfig,
) -> Result<String, String> {
    let (tx, rx) = mpsc::channel(100);
    let running = Arc::new(AtomicBool::new(true));
    let running_clone = running.clone();

    state.runtime.spawn(async move {
        'connection_loop: loop {
            match TcpStream::connect(&config.host, config.port).await {
                Ok(stream) => {
                    // Connection successful
                    emit_event(&app, "tcp-connected", ...);

                    // Split stream
                    let (read_half, write_half) = stream.into_split();

                    // Spawn read and write tasks
                    let read_task = tokio::spawn(read_loop(read_half));
                    let write_task = tokio::spawn(write_loop(write_half, rx));

                    // Wait for completion
                    tokio::select! {
                        _ = read_task => { /* read failed */ },
                        _ = write_task => { /* write failed */ },
                    }
                }
                Err(e) => {
                    emit_event(&app, "tcp-error", ...);
                    if !running_clone.load(Ordering::Relaxed) {
                        break 'connection_loop;
                    }
                }
            }
        }
    });

    Ok("Connection started".to_string())
}

// ✗ Bad: Blocking in async context
async fn bad_tcp_connect() {
    std::thread::sleep(Duration::from_secs(1)); // BLOCKS executor!

    // Missing cleanup patterns
    tokio::spawn(async {
        // Task that might panic and leak resources
    });
}
```

#### 4. Serialization
```rust
// ✓ Good: Consistent serialization structures
#[derive(Debug, Serialize, Clone)]
pub struct SerialData {
    pub port_name: String,
    pub data: Vec<u8>,
    pub timestamp: u64,
}

#[derive(Debug, Serialize, Clone)]
pub struct TcpConnectionStatus {
    pub connection_id: String,
    pub status: String,  // "connected", "disconnected", "error"
    pub message: Option<String>,
    pub timestamp: u64,
}

// ✓ Good: Consistent deserialization
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

// ✗ Bad: Inconsistent field naming
#[derive(Debug, Serialize)]
pub struct BadData {
    port_name: String,
    PortName: String,  // Duplicate with different case
    data: Vec<u8>,
    Data: Vec<u8>,     // Duplicate
}
```

#### 5. Resource Management
```rust
// ✓ Good: Proper cleanup on drop
pub struct TcpClientHandle {
    pub tx: mpsc::Sender<Vec<u8>>,
    pub running: Arc<AtomicBool>,
    pub config: TcpClientConfig,
}

// ✓ Good: Explicit cleanup in commands
#[tauri::command]
fn tcp_client_disconnect(
    state: State<TcpState>,
    connection_id: String,
) -> Result<String, String> {
    // Remove from HashMap (triggers drop)
    if let Some(handle) = state.clients.lock().remove(&connection_id) {
        // Signal async task to stop
        handle.running.store(false, Ordering::Relaxed);
        // Handle is dropped, async task will eventually notice and exit
        Ok("Disconnected".to_string())
    } else {
        Err(format!("Connection {} not found", connection_id))
    }
}

// ✗ Bad: Resource leaks
async fn spawn_task_without_tracking(rx: mpsc::Receiver<Vec<u8>>) {
    tokio::spawn(async move {
        // Task runs forever, never cleans up
        while let Some(data) = rx.recv().await {
            // No exit condition
        }
    });
}
```

---

## Error Handling

### Frontend Error Handling

```javascript
// ✓ Good: User-friendly error messages
const handleConnect = async () => {
  try {
    await invoke('open_port', { ...config });
    isConnected.value = true;
  } catch (error) {
    const message = error.includes('not found')
      ? 'Serial port not found. Check USB connection.'
      : 'Failed to open port. Try another port.';
    emit('error', { message });
  }
};

// ✓ Good: Error boundaries for event handlers
const setupSerialListener = async () => {
  try {
    await listen('serial-data', (event) => {
      try {
        handleSerialData(event.payload);
      } catch (e) {
        console.error('Error processing serial data:', e);
      }
    });
  } catch (e) {
    console.error('Failed to setup listener:', e);
  }
};
```

### Backend Error Handling

```rust
// ✓ Good: Contextual error messages
fn parse_modbus_response(data: &[u8]) -> Result<u16, String> {
    if data.len() < 3 {
        return Err(format!(
            "Invalid response: expected at least 3 bytes, got {}",
            data.len()
        ));
    }

    let crc_bytes = &data[data.len()-2..];
    let expected_crc = u16::from_le_bytes([crc_bytes[0], crc_bytes[1]]);
    let actual_crc = calculate_crc16(&data[..data.len()-2]);

    if actual_crc != expected_crc {
        return Err(format!(
            "CRC mismatch: expected 0x{:04x}, got 0x{:04x}",
            expected_crc, actual_crc
        ));
    }

    Ok(u16::from_be_bytes([data[0], data[1]]))
}

// ✓ Good: Error recovery paths
async fn read_with_retry(stream: &mut TcpStream, buffer: &mut [u8]) -> Result<usize, String> {
    let mut retry_count = 0;
    let max_retries = 3;

    loop {
        match stream.read(buffer).await {
            Ok(n) if n > 0 => return Ok(n),
            Ok(0) => return Err("Connection closed by peer".to_string()),
            Err(e) if e.kind() == io::ErrorKind::Interrupted && retry_count < max_retries => {
                retry_count += 1;
                tokio::time::sleep(Duration::from_millis(100)).await;
                continue;
            }
            Err(e) => return Err(format!("Read error: {}", e)),
        }
    }
}
```

---

## Performance Guidelines

### Frontend Optimization

#### 1. Terminal Message Limit
```javascript
// ✓ Good: Hard limit prevents memory bloat
const addMessage = (message) => {
  messages.value.push(message);

  if (messages.value.length > TERMINAL_LIMIT) {
    messages.value = messages.value.slice(-TERMINAL_LIMIT);
  }
};

const TERMINAL_LIMIT = 500;
```

#### 2. Event Batching
```javascript
// ✓ Good: Batch updates with RAF
let pendingUpdate = false;
let batchedMessages = [];

const queueMessage = (message) => {
  batchedMessages.push(message);

  if (!pendingUpdate) {
    pendingUpdate = true;
    requestAnimationFrame(() => {
      addMessages(batchedMessages);
      batchedMessages = [];
      pendingUpdate = false;
    });
  }
};
```

#### 3. Component Optimization
```vue
<!-- ✓ Good: v-show for frequently toggled content -->
<div v-show="isConnected" class="terminal">
  <!-- Terminal content -->
</div>

<!-- ✓ Good: Key-based list rendering -->
<div v-for="msg in messages" :key="msg.id" class="message">
  {{ msg.text }}
</div>

<!-- ✗ Bad: Unnecessary v-if/v-show combinations -->
<div v-if="visible">
  <div v-show="isConnected">
    <!-- Redundant nesting -->
  </div>
</div>
```

### Backend Optimization

#### 1. Async Task Configuration
```rust
// ✓ Good: Appropriate thread count per use case
let tcp_runtime = tokio::runtime::Builder::new_multi_thread()
    .worker_threads(4)      // Many concurrent connections
    .enable_all()
    .build()?;

let modbus_runtime = tokio::runtime::Builder::new_multi_thread()
    .worker_threads(2)      // Sequential protocol
    .enable_all()
    .build()?;
```

#### 2. Buffer Management
```rust
// ✓ Good: Appropriate buffer sizes
const SERIAL_BUFFER_SIZE: usize = 256;      // Typical serial packets
const TCP_CHANNEL_CAPACITY: usize = 100;    // Reasonable queue depth
const MODBUS_MAX_DATA_SIZE: usize = 252;    // Modbus limit

// ✓ Good: Zero-copy operations
let data = &buffer[..n];  // Slice, don't copy
state.clients.lock().iter().for_each(|client| {
    let _ = client.tx.try_send(data.to_vec());  // Only copy when needed
});
```

#### 3. Lock Contention
```rust
// ✓ Good: Minimize lock duration
fn tcp_client_send(state: State<TcpState>, connection_id: String, data: Vec<u8>) -> Result<String, String> {
    // Acquire lock briefly
    let handle = {
        let clients = state.clients.lock();
        clients.get(&connection_id)
            .map(|h| h.clone())
            .ok_or_else(|| "Connection not found".to_string())?
    }; // Lock released here

    // Send without holding lock
    handle.tx.try_send(data)
        .map_err(|_| "Send queue full".to_string())?;

    Ok("Sent".to_string())
}

// ✗ Bad: Holding lock during async operation
async fn bad_send(state: State<TcpState>, id: String, data: Vec<u8>) {
    let clients = state.clients.lock();  // Locks here
    if let Some(handle) = clients.get(&id) {
        // Long operation with lock held!
        tokio::time::sleep(Duration::from_secs(1)).await;
        let _ = handle.tx.send(data).await;
    }
}
```

---

## Testing Approach

### Frontend Testing

```javascript
// ✓ Good: Testing component integration
describe('SerialTab.vue', () => {
  it('should connect when button clicked', async () => {
    const { getByRole } = render(SerialTab, {
      props: { tabId: 'tab-1' }
    });

    await userEvent.click(getByRole('button', { name: /connect/i }));

    expect(mockInvoke).toHaveBeenCalledWith('open_port', expect.any(Object));
  });

  it('should handle serial data events', async () => {
    const { getByText } = render(SerialTab);

    // Simulate event
    const unsubscribe = await listen('serial-data', handler);
    handler({ payload: { data: [72, 101, 108, 108, 111] } });

    expect(getByText('Hello')).toBeInTheDocument();
  });
});
```

### Backend Testing

```rust
// ✓ Good: Testing protocol parsing
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_modbus_crc_calculation() {
        let data = [0x01, 0x03, 0x00, 0x00, 0x00, 0x02];
        let crc = calculate_crc16(&data);
        assert_eq!(crc, 0x8449);
    }

    #[test]
    fn test_hex_string_parsing() {
        let hex = "48 65 6C 6C 6F";
        let result = parse_hex_string(hex);
        assert_eq!(result, Ok(b"Hello".to_vec()));
    }

    #[tokio::test]
    async fn test_tcp_reconnection() {
        let config = TcpClientConfig {
            host: "localhost".to_string(),
            port: 8080,
            connection_id: "test-1".to_string(),
        };

        // Test would create actual connection
        // Verify reconnect logic works
    }
}
```

---

## Documentation Standards

### Code Comments

```javascript
// ✓ Good: High-level explanation
// RAF batching prevents UI jank when receiving high data rates
let pendingUpdate = false;

// ✓ Good: Why, not what
// Limit to 500 messages to maintain UI responsiveness
const TERMINAL_LIMIT = 500;

// ✗ Bad: Obvious comments
let x = ref(0); // Initialize x to 0
```

```rust
// ✓ Good: Explain non-obvious behavior
// Use Arc<Mutex> to allow ownership sharing between sync and async contexts
// This enables proper cleanup when connections close
pub struct TcpState {
    clients: Arc<Mutex<HashMap<String, TcpClientHandle>>>,
}

// ✓ Good: Document protocol specifics
// Modbus RTU CRC calculation per Modbus specification section A.1
fn calculate_crc16(data: &[u8]) -> u16 {
    // Implementation follows Modbus RTU spec
}

// ✗ Bad: Comments that don't add value
clients.insert(id.clone(), handle); // Insert handle into map
```

### Function Documentation

```rust
// ✓ Good: Doc comments with examples
/// Parses a hex string into a byte vector
///
/// # Arguments
/// * `input` - Space or newline separated hex values (e.g., "48 65 6C 6C 6F")
///
/// # Returns
/// * `Ok(Vec<u8>)` - Parsed bytes
/// * `Err(String)` - Error message if parsing fails
///
/// # Examples
/// ```
/// let result = parse_hex_string("48 65 6C 6C 6F")?;
/// assert_eq!(result, b"Hello");
/// ```
pub fn parse_hex_string(input: &str) -> Result<Vec<u8>, String> {
    // Implementation
}
```

---

## Code Review Checklist

- [ ] Naming follows convention (PascalCase, snake_case, camelCase)
- [ ] Error handling is explicit and user-friendly
- [ ] No hardcoded values (use named constants)
- [ ] Resource cleanup is explicit (close connections, remove listeners)
- [ ] Thread-safety verified (Arc, Mutex, atomic types)
- [ ] Comments explain "why", not "what"
- [ ] Performance implications considered (loops, locks, allocations)
- [ ] Edge cases handled (empty input, network errors, etc.)
- [ ] Code follows project structure conventions
- [ ] No breaking changes to command signatures

---

**Last Updated**: 2025-12-29
**Version**: 1.0
**Applies To**: TermiPro v1.0.0+
