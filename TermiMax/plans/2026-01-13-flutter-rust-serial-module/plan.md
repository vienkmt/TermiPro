# Implementation Plan: TermiPro Flutter + Rust - Serial Module

**Date**: 2026-01-13
**Status**: Pending Approval
**Scope**: Viết lại TermiPro từ Tauri (Vue.js + Rust) sang Flutter + Rust, module Serial

---

## Executive Summary

Dự án này port TermiPro từ Tauri sang Flutter + Rust sử dụng `flutter_rust_bridge v2` cho FFI integration. Module đầu tiên là Serial Monitor với giao diện y hệt bản gốc.

### Architecture Overview

```
┌─────────────────────────────────────────────────────────────┐
│                    Flutter UI Layer                          │
│  ┌─────────────┐  ┌─────────────┐  ┌──────────────────────┐ │
│  │ SerialPage  │  │ Sidebar     │  │ Terminal Display     │ │
│  │ (StatefulW) │  │ (Config)    │  │ (TX/RX + Stats)      │ │
│  └─────────────┘  └─────────────┘  └──────────────────────┘ │
│                           │                                  │
│  ┌────────────────────────▼──────────────────────────────┐  │
│  │              Riverpod State Management                 │  │
│  │  - portsProvider      - connectionStateProvider       │  │
│  │  - serialDataProvider - autoSendProvider              │  │
│  └────────────────────────┬──────────────────────────────┘  │
└───────────────────────────┼──────────────────────────────────┘
                            │ FFI (flutter_rust_bridge)
┌───────────────────────────▼──────────────────────────────────┐
│                    Rust Backend (API)                        │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────────────┐  │
│  │ list_ports  │  │ open_port   │  │ stream_serial_data  │  │
│  │ close_port  │  │ send_data   │  │ is_port_open        │  │
│  └─────────────┘  └─────────────┘  └─────────────────────┘  │
│                           │                                  │
│  ┌────────────────────────▼──────────────────────────────┐  │
│  │              SerialState (Arc<Mutex<>>)                │  │
│  │  - ports: HashMap<String, Box<dyn SerialPort>>        │  │
│  │  - running: HashMap<String, AtomicBool>               │  │
│  └────────────────────────────────────────────────────────┘  │
└──────────────────────────────────────────────────────────────┘
```

---

## Project Structure

```
termipro_flutter/
├── lib/
│   ├── main.dart                          # Entry point + RustLib.init()
│   ├── app.dart                           # App configuration
│   ├── theme/
│   │   ├── app_theme.dart                 # Light theme, sky blue accent
│   │   └── app_colors.dart                # Color constants
│   ├── models/
│   │   ├── serial_config.dart             # PortConfig, SerialData
│   │   ├── port_info.dart                 # PortInfo from Rust
│   │   └── terminal_entry.dart            # TX/RX entry model
│   ├── providers/
│   │   ├── serial_provider.dart           # Serial state management
│   │   ├── ports_provider.dart            # Available ports list
│   │   ├── terminal_provider.dart         # Terminal data
│   │   └── auto_send_provider.dart        # Auto send state
│   ├── services/
│   │   └── serial_service.dart            # Rust API wrapper
│   ├── screens/
│   │   └── serial_screen.dart             # Main serial monitor screen
│   └── widgets/
│       ├── sidebar/
│       │   ├── port_selector.dart         # Custom dropdown
│       │   ├── config_card.dart           # Baud, data bits, etc.
│       │   ├── signal_toggles.dart        # DTR/RTS switches
│       │   ├── display_options.dart       # Text/Hex, auto scroll
│       │   └── auto_send_card.dart        # Auto send settings
│       ├── terminal/
│       │   ├── terminal_display.dart      # Message list
│       │   ├── terminal_line.dart         # Single TX/RX line
│       │   ├── terminal_header.dart       # Stats + clear button
│       │   └── terminal_empty.dart        # Empty state
│       ├── send_panel/
│       │   ├── send_container.dart        # Input + buttons
│       │   └── hex_toggle.dart            # Hex mode switch
│       └── common/
│           ├── custom_dropdown.dart       # Styled dropdown
│           ├── toggle_switch.dart         # iOS-style toggle
│           └── connect_button.dart        # Connect/Disconnect
├── rust/
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs                         # Bridge entry
│       └── api/
│           ├── mod.rs                     # Module exports
│           ├── serial.rs                  # Serial commands
│           └── models.rs                  # Shared structs
├── pubspec.yaml
├── macos/
│   └── Runner/
│       └── Release.entitlements           # USB entitlements
├── windows/
└── linux/
```

---

## Phase Implementation

### Phase 1: Project Setup & Rust Bridge

**Objective**: Khởi tạo dự án Flutter + Rust với flutter_rust_bridge v2

**Tasks**:
1. Tạo Flutter project với flutter_rust_bridge
2. Cấu hình Cargo.toml với dependencies (serialport, tokio, serde)
3. Setup platform-specific configurations (macOS entitlements, Windows COM access)
4. Verify bridge hoạt động với simple ping function

**Files to create**:
- `termipro_flutter/` (new Flutter project)
- `rust/Cargo.toml`
- `rust/src/lib.rs`
- `rust/src/api/mod.rs`
- `macos/Runner/Release.entitlements`

**Dependencies**:
```toml
# Cargo.toml
[dependencies]
flutter_rust_bridge = "2"
serialport = "4.3"
tokio = { version = "1", features = ["full"] }
serde = { version = "1", features = ["derive"] }
parking_lot = "0.12"
```

```yaml
# pubspec.yaml
dependencies:
  flutter_rust_bridge: ^2.0.0
  flutter_riverpod: ^2.4.0
  google_fonts: ^6.1.0
  freezed_annotation: ^2.4.0

dev_dependencies:
  freezed: ^2.4.0
  build_runner: ^2.4.0
```

---

### Phase 2: Rust Serial API Implementation

**Objective**: Port Rust serial logic từ Tauri sang flutter_rust_bridge

**Tasks**:
1. Implement `list_serial_ports()` - liệt kê USB ports
2. Implement `open_port(config)` - mở connection với DTR/RTS
3. Implement `close_port(port_name)` - đóng connection
4. Implement `send_data(port_name, data, is_hex, byte_delay)` - gửi dữ liệu
5. Implement `stream_serial_data(port_name, sink)` - stream RX data
6. Implement `is_port_open(port_name)` - check status

**Key Differences from Tauri**:
- Replace `#[tauri::command]` với plain functions
- Replace `State<>` với `static OnceLock<Arc<Mutex<SerialState>>>`
- Replace `app.emit()` với `StreamSink<T>.add()`
- Remove `AppHandle` parameters

**Rust API Design**:

```rust
// rust/src/api/models.rs
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PortInfo {
    pub name: String,
    pub port_type: String,
    pub manufacturer: Option<String>,
    pub product: Option<String>,
}

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

#[derive(Debug, Serialize, Clone)]
pub struct SerialData {
    pub port_name: String,
    pub data: Vec<u8>,
    pub timestamp: u64,
}

#[derive(Debug, Serialize, Clone)]
pub struct PortDisconnected {
    pub port_name: String,
    pub reason: String,
    pub timestamp: u64,
}
```

```rust
// rust/src/api/serial.rs
use flutter_rust_bridge::StreamSink;

pub fn list_serial_ports() -> Result<Vec<PortInfo>, String>;
pub fn open_port(config: SerialConfig) -> Result<String, String>;
pub fn close_port(port_name: String) -> Result<String, String>;
pub fn send_data(port_name: String, data: String, is_hex: bool, byte_delay_us: Option<u64>) -> Result<String, String>;
pub fn is_port_open(port_name: String) -> bool;

// Streaming - key difference from Tauri
pub fn stream_serial_data(port_name: String, baud_rate: u32, sink: StreamSink<SerialData>);
pub fn stream_port_status(sink: StreamSink<PortDisconnected>);
```

---

### Phase 3: Flutter Theme & Base Widgets

**Objective**: Thiết lập theme và base components giống UI gốc

**Theme Specs** (from SerialTab.vue):
- **Primary Font**: Plus Jakarta Sans (UI)
- **Mono Font**: JetBrains Mono (terminal, code)
- **Accent**: Sky Blue (#0ea5e9 / Colors.blue.shade500)
- **Background**: Light mode với bg-primary, bg-secondary, bg-tertiary
- **Radius**: 4px (sm), 8px (md), 12px (lg)

**Color Palette**:
```dart
// app_colors.dart
class AppColors {
  static const accent = Color(0xFF0EA5E9);      // Sky blue
  static const accentLight = Color(0xFFE0F2FE);
  static const danger = Color(0xFFEF4444);
  static const dangerLight = Color(0xFFFEE2E2);
  static const success = Color(0xFF22C55E);
  static const warning = Color(0xFFF59E0B);
  static const txColor = Color(0xFFEA580C);     // Orange for TX
  static const rxColor = Color(0xFF0EA5E9);     // Blue for RX

  static const bgPrimary = Color(0xFFFFFFFF);
  static const bgSecondary = Color(0xFFF8FAFC);
  static const bgTertiary = Color(0xFFF1F5F9);
  static const borderColor = Color(0xFFE2E8F0);
  static const textPrimary = Color(0xFF1E293B);
  static const textSecondary = Color(0xFF64748B);
  static const textTertiary = Color(0xFF94A3B8);
}
```

**Base Widgets**:
1. `CustomDropdown` - Styled dropdown với animation
2. `ToggleSwitch` - iOS-style toggle (DTR, RTS, Hex mode)
3. `ConnectButton` - Gradient button với connected/disconnected states
4. `ConfigCard` - Card container với header icon

---

### Phase 4: Sidebar UI Components

**Objective**: Implement sidebar widgets giống SerialTab.vue

**Components**:

1. **PortSelector** (`port_selector.dart`)
   - Custom dropdown với port list
   - Hiển thị port_type + product/manufacturer
   - "In use" badge cho ports đang được dùng
   - Auto refresh khi click

2. **ConfigCard** (`config_card.dart`)
   - Baud Rate dropdown (9600, 19200, 57600, 115200, 460800, 921600)
   - Data Bits dropdown (5, 6, 7, 8)
   - Stop Bits dropdown (1, 1.5, 2)
   - Parity dropdown (None, Odd, Even)

3. **SignalToggles** (`signal_toggles.dart`)
   - DTR toggle switch
   - RTS toggle switch
   - Help button (?) với modal giải thích

4. **DisplayOptions** (`display_options.dart`)
   - Text/Hex toggle buttons
   - Auto Scroll switch
   - Line Ending selector (None, CR, LF, CRLF)

5. **AutoSendCard** (`auto_send_card.dart`)
   - Interval input (50-60000 ms)
   - Byte Delay input (0-10000 µs)
   - Send count display
   - ON badge khi active

---

### Phase 5: Terminal Display & Send Panel

**Objective**: Implement terminal hiển thị và send controls

**Terminal Components**:

1. **TerminalHeader** (`terminal_header.dart`)
   - Title với icon
   - TX/RX statistics badges
   - Clear button

2. **TerminalDisplay** (`terminal_display.dart`)
   - ListView.builder cho entries
   - Auto scroll khi enabled
   - MAX_TERMINAL_ENTRIES = 500

3. **TerminalLine** (`terminal_line.dart`)
   - Timestamp `[HH:mm:ss]`
   - TX/RX badge với màu (TX=orange, RX=blue)
   - Data text (text mode hoặc hex mode)
   - Border-left color theo type

4. **TerminalEmpty** (`terminal_empty.dart`)
   - Icon + "No data" message
   - "Connect to start" subtitle

**Send Panel Components**:

1. **SendContainer** (`send_container.dart`)
   - Hex toggle (compact)
   - Text input với placeholder
   - Clear button (X) trong input
   - Send button (gradient)
   - Auto button / Stop button

---

### Phase 6: State Management & Integration

**Objective**: Connect UI với Rust backend qua Riverpod

**Providers**:

```dart
// ports_provider.dart
final portsProvider = FutureProvider<List<PortInfo>>((ref) async {
  return await SerialService.listPorts();
});

// serial_provider.dart
final serialConfigProvider = StateProvider<SerialConfig>((ref) => SerialConfig.default());
final connectionStateProvider = StateProvider<ConnectionState>((ref) => ConnectionState.disconnected);

// terminal_provider.dart
final terminalDataProvider = StateNotifierProvider<TerminalNotifier, List<TerminalEntry>>((ref) {
  return TerminalNotifier();
});

// auto_send_provider.dart
final autoSendProvider = StateNotifierProvider<AutoSendNotifier, AutoSendState>((ref) {
  return AutoSendNotifier();
});
```

**Data Flow**:
1. User selects port → updates `serialConfigProvider`
2. User clicks Connect → calls `open_port()`, starts `stream_serial_data()`
3. Stream emits data → `terminalDataProvider.addRx()`
4. User sends message → calls `send_data()`, `terminalDataProvider.addTx()`
5. User clicks Disconnect → calls `close_port()`, cancels stream

---

### Phase 7: Polish & Platform Testing

**Objective**: Hoàn thiện UI và test trên 3 platforms

**Tasks**:
1. Fine-tune animations (dropdown slide, button hover)
2. Implement Signal Help Modal
3. Add keyboard shortcuts (Enter to send)
4. Test trên macOS với actual USB serial device
5. Test trên Windows với COM ports
6. Test trên Linux với /dev/ttyUSB*
7. Fix platform-specific issues

**Platform Checklist**:
- [ ] macOS: USB entitlements, code signing
- [ ] Windows: COM port access, MSVC runtime
- [ ] Linux: udev rules documentation

---

## Vue → Flutter Widget Mapping

| Vue Component | Flutter Widget | Notes |
|---------------|----------------|-------|
| `<div class="sidebar">` | `Container(width: 320)` | Fixed width sidebar |
| `<div class="config-card">` | `Card` + `ConfigCard` widget | Custom styled card |
| `<div class="dropdown-item">` | `CustomDropdown` | Custom animated dropdown |
| `<button class="btn-connect">` | `ConnectButton` | Gradient + states |
| `<label class="toggle-switch">` | `ToggleSwitch` | Custom iOS-style |
| `<div class="terminal">` | `ListView.builder` | Efficient list rendering |
| `<div class="terminal-line">` | `TerminalLine` | TX/RX với badges |
| `<input>` | `TextField` | Styled input |
| `v-model` | Provider state | Riverpod cho state |
| `@click` | `onTap` | GestureDetector/InkWell |
| `:class="{ active }"` | Conditional styling | Tính toán trong build |

---

## Risk Assessment

| Risk | Impact | Mitigation |
|------|--------|------------|
| flutter_rust_bridge compatibility | High | Test early với simple functions |
| Serial port access permissions | Medium | Document platform-specific setup |
| Stream backpressure | Medium | Implement throttling như bản gốc |
| Font loading issues | Low | Use google_fonts package |
| Different UI behavior across platforms | Medium | Test frequently on all 3 platforms |

---

## Success Criteria

1. ✅ List USB serial ports (filter như bản gốc)
2. ✅ Connect/Disconnect với full config (baud, data bits, stop bits, parity, DTR, RTS)
3. ✅ Send data (text mode với line ending, hex mode)
4. ✅ Receive data real-time với StreamSink
5. ✅ Terminal display với TX/RX badges, timestamps
6. ✅ Auto send với interval và counter
7. ✅ Display mode toggle (Text/Hex)
8. ✅ Auto scroll
9. ✅ Clear terminal
10. ✅ TX/RX statistics
11. ✅ UI giống 95%+ so với bản gốc
12. ✅ Hoạt động trên macOS, Windows, Linux

---

## Estimated Effort

| Phase | Tasks | Complexity |
|-------|-------|------------|
| Phase 1: Setup | 4 | Medium |
| Phase 2: Rust API | 6 | High |
| Phase 3: Theme | 4 | Low |
| Phase 4: Sidebar | 5 | Medium |
| Phase 5: Terminal | 5 | Medium |
| Phase 6: Integration | 4 | High |
| Phase 7: Polish | 7 | Medium |

---

## Next Steps

1. Approve plan
2. Start Phase 1: Project setup
3. Implement Rust API first (most critical)
4. Build UI incrementally
5. Test frequently with real hardware

---

**Plan Version**: 1.0
**Last Updated**: 2026-01-13
