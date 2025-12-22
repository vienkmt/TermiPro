# TCP Server - Rust Backend Documentation

## Overview
TCP Server implementation trong TermiPro, hỗ trợ tối đa 20 clients kết nối đồng thời với tính năng Echo tự động.

## Files
- `src-tauri/src/lib.rs` - Main implementation
- `src/components/TcpServerTab.vue` - Frontend UI

---

## Structs

### TcpServerConfig
```rust
#[derive(Debug, Deserialize, Clone)]
pub struct TcpServerConfig {
    pub port: u16,              // Port lắng nghe
    pub bind_address: String,   // "0.0.0.0" hoặc "127.0.0.1"
    pub server_id: String,      // Unique ID (thường là tab ID)
    pub max_clients: u32,       // Giới hạn clients (max 20)
}
```

### TcpServerHandle
```rust
pub struct TcpServerHandle {
    pub config: TcpServerConfig,
    pub running: Arc<AtomicBool>,
    pub clients: Arc<Mutex<HashMap<String, TcpServerClientHandle>>>,
    pub echo_enabled: Arc<AtomicBool>,  // Echo data back to client
}
```

### TcpServerClientHandle
```rust
pub struct TcpServerClientHandle {
    pub tx: mpsc::Sender<Vec<u8>>,  // Channel để gửi data đến client
    pub info: TcpClientInfo,         // Thông tin client
}
```

### TcpClientInfo
```rust
#[derive(Debug, Serialize, Clone)]
pub struct TcpClientInfo {
    pub client_id: String,      // "client-1", "client-2", ...
    pub remote_addr: String,    // "192.168.1.100:54321"
    pub connected_at: u64,      // Timestamp
}
```

### TcpServerClientEvent
```rust
#[derive(Debug, Serialize, Clone)]
pub struct TcpServerClientEvent {
    pub server_id: String,
    pub client_id: String,
    pub remote_addr: String,
    pub event_type: String,  // "connected" | "disconnected"
    pub timestamp: u64,
}
```

---

## Tauri Commands

### tcp_server_start
```rust
#[tauri::command]
fn tcp_server_start(
    app: AppHandle,
    state: State<TcpState>,
    config: TcpServerConfig,
) -> Result<String, String>
```

**Flow:**
1. Check server_id đã tồn tại chưa
2. Tạo TcpServerHandle với echo_enabled = false
3. Thêm vào HashMap
4. Spawn async task để listen
5. Bind TCP listener (emit error nếu port đã được sử dụng)
6. Accept connections loop

**Server Loop:**
```
loop {
    1. Accept new connection
    2. Check max_clients limit
    3. Emit "tcp-server-client-event" (connected)
    4. Spawn client handler task
}
```

**Client Handler:**
```
loop {
    select! {
        // Đọc data từ client
        read_result => {
            - Emit "tcp-data" event
            - If echo_enabled: send "Echo: " + data back
        }

        // Gửi data đến client (từ channel)
        rx_result => {
            - Some(data): write to client
            - None: channel closed, break (server disconnect)
        }
    }
}

// Cleanup & emit disconnected event
```

### tcp_server_stop
```rust
#[tauri::command]
fn tcp_server_stop(
    state: State<TcpState>,
    server_id: String,
) -> Result<String, String>
```

### tcp_server_set_echo
```rust
#[tauri::command]
fn tcp_server_set_echo(
    state: State<TcpState>,
    server_id: String,
    enabled: bool,
) -> Result<String, String>
```

Bật/tắt tính năng Echo. Khi bật, server tự động gửi "Echo: " + data về client.

### tcp_server_send
```rust
#[tauri::command]
fn tcp_server_send(
    state: State<TcpState>,
    server_id: String,
    client_id: Option<String>,  // None = gửi tất cả
    data: String,
    is_hex: bool,
) -> Result<String, String>
```

### tcp_server_disconnect_client
```rust
#[tauri::command]
fn tcp_server_disconnect_client(
    state: State<TcpState>,
    server_id: String,
    client_id: String,
) -> Result<String, String>
```

Ngắt kết nối 1 client cụ thể bằng cách remove khỏi HashMap (channel đóng -> client handler break).

### tcp_server_get_clients
```rust
#[tauri::command]
fn tcp_server_get_clients(
    state: State<TcpState>,
    server_id: String,
) -> Result<Vec<TcpClientInfo>, String>
```

---

## Events

### tcp-data
```javascript
{
  connection_id: "tab-1",      // server_id
  client_id: "client-1",       // ID của client gửi data
  data: [72, 101, 108, 108, 111],
  timestamp: 1703123456789
}
```

### tcp-server-status
| Status | Mô tả |
|--------|-------|
| `started` | Server đang listen |
| `stopped` | Server đã dừng |
| `error` | Lỗi (vd: port đã sử dụng) |

```javascript
{
  connection_id: "tab-1",
  status: "started",
  message: "Listening on 0.0.0.0:5000",
  timestamp: 1703123456789
}
```

### tcp-server-client-event
```javascript
{
  server_id: "tab-1",
  client_id: "client-1",
  remote_addr: "192.168.1.100:54321",
  event_type: "connected",  // hoặc "disconnected"
  timestamp: 1703123456789
}
```

---

## Frontend Integration

### Start Server
```javascript
await invoke("tcp_server_start", {
  config: {
    port: 5000,
    bind_address: "0.0.0.0",
    server_id: tabId,
    max_clients: 20,
  }
});
```

### Set Echo
```javascript
await invoke("tcp_server_set_echo", {
  serverId: tabId,
  enabled: true,
});
```

### Send to Client(s)
```javascript
// Gửi đến 1 client
await invoke("tcp_server_send", {
  serverId: tabId,
  clientId: "client-1",
  data: "Hello",
  isHex: false,
});

// Gửi đến tất cả clients
await invoke("tcp_server_send", {
  serverId: tabId,
  clientId: null,
  data: "Broadcast",
  isHex: false,
});
```

### Disconnect Client
```javascript
await invoke("tcp_server_disconnect_client", {
  serverId: tabId,
  clientId: "client-1",
});
```

---

## Echo Feature

Khi Echo được bật:
1. Client gửi data đến server
2. Server nhận và emit "tcp-data" event (hiển thị trong terminal)
3. Server tự động gửi "Echo: " + data về client

**Rust implementation:**
```rust
if echo_ref.load(Ordering::Relaxed) {
    let received_str = String::from_utf8_lossy(&received_data);
    let echo_response = format!("Echo: {}", received_str);
    write_half.write_all(echo_response.as_bytes()).await;
}
```

---

## Error Handling

### Port Already in Use
```rust
if e.kind() == std::io::ErrorKind::AddrInUse {
    // Emit error với message tiếng Việt
    "Cổng {} đã được sử dụng bởi ứng dụng khác"
}
```

### Client Disconnect
- Server tự động cleanup khi client disconnect
- Emit "tcp-server-client-event" với event_type="disconnected"
- Remove client khỏi connected list

---

## Testing

### Test với netcat (nc)
```bash
# TermiPro: Start server on port 5000

# Connect as client
nc localhost 5000

# Gửi data, sẽ thấy trong TermiPro terminal
# Nếu Echo bật, sẽ nhận "Echo: ..." response
```

### Test multiple clients
```bash
# Terminal 1
nc localhost 5000

# Terminal 2
nc localhost 5000

# Cả 2 hiển thị trong connected clients list
```
