# TCP Client - Rust Backend Documentation

## Overview
TCP Client implementation trong TermiPro với auto-reconnect và retry logic, hỗ trợ kết nối đến TCP server từ xa.

## Files
- `src-tauri/src/lib.rs` - Main implementation
- `src-tauri/Cargo.toml` - Dependencies (tokio)

---

## Structs

### TcpClientConfig
```rust
#[derive(Debug, Deserialize, Clone)]
pub struct TcpClientConfig {
    pub host: String,         // Server hostname/IP
    pub port: u16,            // Server port
    pub connection_id: String, // Unique ID (thường là tab ID)
}
```

### TcpClientHandle
```rust
pub struct TcpClientHandle {
    pub tx: mpsc::Sender<Vec<u8>>,  // Channel để gửi data
    pub running: Arc<AtomicBool>,   // Flag để stop connection
    pub config: TcpClientConfig,    // Lưu config để có thể reconnect
}
```

### TcpData (Event payload)
```rust
#[derive(Debug, Serialize, Clone)]
pub struct TcpData {
    pub connection_id: String,
    pub client_id: Option<String>,  // None cho TCP Client
    pub data: Vec<u8>,
    pub timestamp: u64,
}
```

### TcpConnectionStatus (Event payload)
```rust
#[derive(Debug, Serialize, Clone)]
pub struct TcpConnectionStatus {
    pub connection_id: String,
    pub status: String,           // "connected", "disconnected", "error", "reconnecting", "retrying", "write_failed"
    pub message: Option<String>,  // Chi tiết lỗi/trạng thái
    pub timestamp: u64,
}
```

---

## State Management

### TcpState
```rust
pub struct TcpState {
    clients: Arc<Mutex<HashMap<String, TcpClientHandle>>>,
    servers: Arc<Mutex<HashMap<String, TcpServerHandle>>>,
    runtime: tokio::runtime::Runtime,
}
```

**Lý do dùng Arc<Mutex<HashMap>>:**
- `Arc`: Cho phép share ownership giữa main thread và async tasks
- Async task cần cleanup (remove từ HashMap) khi connection kết thúc
- Cho phép mở nhiều tab cùng kết nối đến 1 server (mỗi tab có unique connection_id)

---

## Retry & Reconnect Configuration

```rust
const TCP_CLIENT_MAX_RETRIES: u32 = 3;           // Số lần retry khi gửi thất bại
const TCP_CLIENT_RETRY_DELAY_MS: u64 = 500;      // Delay giữa các lần retry (ms)
const TCP_CLIENT_RECONNECT_DELAY_MS: u64 = 1000; // Delay trước khi reconnect (ms)
const TCP_CLIENT_MAX_RECONNECT_ATTEMPTS: u32 = 3; // Số lần thử reconnect tối đa
```

---

## Tauri Commands

### tcp_client_connect

```rust
#[tauri::command]
fn tcp_client_connect(
    app: AppHandle,
    state: State<TcpState>,
    config: TcpClientConfig,
) -> Result<String, String>
```

**Flow:**
1. Check connection_id đã tồn tại chưa
2. Tạo channel (tx, rx) để communicate với async task
3. Spawn async task với auto-reconnect loop
4. Lưu TcpClientHandle vào HashMap
5. Return success message

**Connection Loop (async task):**
```
'connection_loop: loop {
    1. Try connect to server
       - Success: reset reconnect_attempts, continue
       - Fail: increment attempts, emit "reconnecting", wait, retry

    2. Emit "connected" status

    3. Split stream into read_half + write_half

    4. Spawn read task:
       - Read data from server
       - Emit "tcp-data" event
       - Set connection_lost flag on error

    5. Spawn write task with retry logic:
       - Receive data from channel
       - Try write with 3 retries, 500ms delay
       - Emit "retrying" status on failure
       - Emit "write_failed" if all retries fail

    6. Wait for both tasks to complete

    7. If connection_lost:
       - Increment reconnect_attempts
       - If max reached: emit "disconnected", break
       - Else: emit "reconnecting", wait 1s, continue loop

    8. If user disconnected (running=false): break
}

// Cleanup: remove from HashMap
clients.remove(&connection_id);
```

### tcp_client_disconnect

```rust
#[tauri::command]
fn tcp_client_disconnect(
    state: State<TcpState>,
    connection_id: String,
) -> Result<String, String>
```

**Flow:**
1. Remove TcpClientHandle từ HashMap
2. Set running flag = false (async task sẽ tự cleanup)

### tcp_client_send

```rust
#[tauri::command]
fn tcp_client_send(
    state: State<TcpState>,
    connection_id: String,
    data: String,
    is_hex: bool,
) -> Result<String, String>
```

**Flow:**
1. Get TcpClientHandle từ HashMap
2. Parse data (text hoặc hex)
3. Send qua channel (tx.try_send)
4. Async task sẽ nhận và write với retry logic

### is_tcp_client_connected

```rust
#[tauri::command]
fn is_tcp_client_connected(
    state: State<TcpState>,
    connection_id: String,
) -> bool
```

---

## Events

### tcp-data
Emitted khi nhận data từ server.

```javascript
{
  connection_id: "tab-1",
  client_id: null,  // Luôn null cho TCP Client
  data: [72, 101, 108, 108, 111],  // Raw bytes
  timestamp: 1703123456789
}
```

### tcp-client-status
Emitted khi trạng thái connection thay đổi.

| Status | Mô tả |
|--------|-------|
| `connected` | Kết nối thành công |
| `disconnected` | Đã ngắt kết nối |
| `error` | Lỗi (không thể kết nối) |
| `reconnecting` | Đang thử kết nối lại |
| `retrying` | Đang retry gửi data |
| `write_failed` | Gửi thất bại sau tất cả retries |

**Payload:**
```javascript
{
  connection_id: "tab-1",
  status: "reconnecting",
  message: "Mất kết nối, đang thử kết nối lại (1/3)",
  timestamp: 1703123456789
}
```

---

## Frontend Integration

### Invoke Commands
```javascript
import { invoke } from "@tauri-apps/api/core";

// Connect
await invoke("tcp_client_connect", {
  config: {
    host: "localhost",
    port: 8080,
    connection_id: tabId,
  }
});

// Disconnect
await invoke("tcp_client_disconnect", {
  connectionId: tabId
});

// Send data
await invoke("tcp_client_send", {
  connectionId: tabId,
  data: "Hello",
  isHex: false
});
```

### Listen Events
```javascript
import { listen } from "@tauri-apps/api/event";

// Listen for data
const unlistenData = await listen("tcp-data", (event) => {
  const { connection_id, data, timestamp } = event.payload;
  // Route to correct tab
});

// Listen for status changes
const unlistenStatus = await listen("tcp-client-status", (event) => {
  const { connection_id, status, message } = event.payload;

  if (status === "connected") {
    tab.isConnected = true;
    tab.isReconnecting = false;
  } else if (status === "reconnecting" || status === "retrying") {
    tab.isReconnecting = true;
    tab.statusMessage = message;
  } else if (status === "disconnected" || status === "error") {
    tab.isConnected = false;
    tab.isReconnecting = false;
  }
});
```

---

## Error Handling

### Connection Errors
- Server không available: Retry 3 lần, emit "error" nếu fail
- Server disconnect: Emit "reconnecting", try reconnect 3 lần

### Write Errors
- Write timeout (5s): Retry với 500ms delay
- Write fail: Retry 3 lần, emit "write_failed" nếu không khắc phục được
- Sau khi write_failed: Set connection_lost flag, trigger reconnect

### Cleanup
- Async task tự cleanup khi kết thúc (remove từ HashMap)
- Cho phép tạo connection mới với cùng connection_id sau khi cleanup

---

## Dependencies (Cargo.toml)

```toml
[dependencies]
tokio = { version = "1", features = ["sync", "time", "net", "rt-multi-thread", "io-util"] }
parking_lot = "0.12"  # For Mutex
```

---

## Multi-Tab Support

**Vấn đề:** Mở 2+ tabs cùng kết nối đến 1 server

**Giải pháp:**
- Mỗi tab có unique `connection_id` (thường là tab ID như "tab-1", "tab-2")
- HashMap key là connection_id, không phải host:port
- Mỗi connection hoàn toàn độc lập

```rust
// HashMap structure
clients: {
  "tab-1": TcpClientHandle { config: { host: "localhost", port: 8080 }, ... },
  "tab-2": TcpClientHandle { config: { host: "localhost", port: 8080 }, ... },
}
```

---

## Receiver Ownership Pattern

**Vấn đề:** Rust ownership - receiver bị move vào async closure, không thể reuse khi reconnect

**Giải pháp:** Wrap receiver trong Option, sử dụng `.take()` để move in/out

```rust
// Wrap trong Option
let mut rx_option: Option<mpsc::Receiver<Vec<u8>>> = Some(rx);

'connection_loop: loop {
    // Take từ Option trước khi spawn task
    let mut rx = match rx_option.take() {
        Some(r) => r,
        None => break,
    };

    let write_task = tokio::spawn(async move {
        // ... use rx ...
        rx // Return receiver
    });

    // Lấy lại từ task result
    if let Ok(returned_rx) = write_result {
        rx_option = Some(returned_rx);
    }
}
```

---

## Testing

### Test với netcat (nc)
```bash
# Start server
nc -l 8080

# TermiPro sẽ connect và có thể gửi/nhận data
```

### Test reconnect
```bash
# Start server
nc -l 8080

# Trong TermiPro: Connect -> Gửi data
# Kill nc (Ctrl+C)
# TermiPro sẽ hiển thị "Đang thử kết nối lại..."

# Restart server trong vòng reconnect timeout
nc -l 8080

# TermiPro tự động kết nối lại
```
