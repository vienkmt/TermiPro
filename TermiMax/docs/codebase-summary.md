# TermiMax Flutter - Codebase Summary

## Quick Reference

**Project**: TermiMax Flutter - Professional Serial Monitor
**Version**: 1.0.0+1
**Type**: Cross-platform desktop/mobile application (Flutter + Rust)
**Language**: Dart, Rust, YAML
**Total Lines of Code**: ~2,500+ (not counting generated code and dependencies)

## Repository Structure

```
termimax_flutter/
├── lib/                              # Flutter/Dart frontend
│   ├── main.dart                     # App entry point (10 lines)
│   ├── screens/                      # Screen widgets
│   │   └── serial_screen.dart        # Main serial monitor (721 lines)
│   ├── models/                       # Data models
│   │   └── serial_config.dart        # Configuration models (140 lines)
│   ├── theme/                        # Material Design 3 theme
│   │   ├── app_theme.dart            # Theme definition (273 lines)
│   │   ├── app_colors.dart           # Color palette
│   │   ├── app_typography.dart       # Font styles
│   │   └── theme.dart                # Exports
│   ├── widgets/                      # Reusable UI components
│   │   ├── sidebar/                  # Left panel controls
│   │   │   ├── port_selector.dart
│   │   │   ├── config_controls.dart
│   │   │   ├── signals_display.dart
│   │   │   ├── display_options.dart
│   │   │   └── sidebar.dart
│   │   ├── chart/                    # Real-time visualization
│   │   │   └── realtime_chart.dart
│   │   └── common/                   # Shared components
│   │       └── custom_dropdown.dart
│   └── src/rust/                     # Auto-generated FFI bindings
│
├── rust/                             # Rust backend for serial I/O
│   ├── Cargo.toml                    # Rust dependencies & config
│   ├── src/
│   │   ├── lib.rs                    # Crate root (2 lines)
│   │   ├── api/
│   │   │   ├── mod.rs                # Module exports
│   │   │   ├── serial.rs             # Serial port logic (366 lines)
│   │   │   ├── models.rs             # Data structures (40 lines)
│   │   │   ├── state.rs              # Global state (30 lines)
│   │   │   └── simple.rs             # Utilities
│   │   └── frb_generated.rs          # Auto-generated (DO NOT EDIT)
│   ├── build.rs                      # Build script
│   └── target/                       # Compiled output
│
├── pubspec.yaml                      # Flutter dependencies
├── flutter_rust_bridge.yaml          # FFI config
├── analysis_options.yaml             # Lint rules
├── Cargo.toml                        # Workspace config
├── README.md                         # Quick start guide
├── docs/                             # Documentation
│   ├── project-overview-pdr.md
│   ├── system-architecture.md
│   ├── code-standards.md
│   └── codebase-summary.md (this file)
├── macos/                            # macOS platform code
├── windows/                          # Windows platform code
├── linux/                            # Linux platform code
├── ios/                              # iOS platform code (stub)
├── android/                          # Android platform code (stub)
└── test/                             # Test files
```

## Core Components

### 1. Flutter Frontend

**Entry Point**: `lib/main.dart`
```dart
Future<void> main() async {
  WidgetsFlutterBinding.ensureInitialized();
  await RustLib.init();  // Initialize Rust FFI
  runApp(const MyApp());
}
```

**Main Application Widget**:
- Class: `MyApp` (StatelessWidget)
- Theme: `AppTheme.light` (Material Design 3)
- Home: `SerialScreen` (StatefulWidget)

### 2. Serial Screen (Main UI)

**File**: `lib/screens/serial_screen.dart` (~721 lines)

**Responsibility**: Orchestrate entire application UI and state

**State Variables**:
```dart
List<PortInfo> _ports;              // Available serial ports
String? _selectedPort;              // Currently selected port
bool _isConnected;                  // Connection status
SerialConfigModel _config;          // Serial configuration
DisplayOptions _displayOptions;     // Display settings
AutoSendSettings _autoSendSettings; // Auto-send config

List<TerminalEntry> _terminalEntries;  // Terminal output
int _txCount = 0;                      // TX counter
int _rxCount = 0;                      // RX counter

List<int> _rxBuffer;                // Line-based parsing buffer
List<double> _chartData;            // Chart data points (~500 max)
List<List<int>> _pendingRxData;     // Batch for high-frequency data
bool _updateScheduled;              // Batching flag

StreamSubscription<SerialData>? _dataSubscription;
TextEditingController _sendController;
ScrollController _terminalScrollController;
```

**Key Methods**:

| Method | Purpose |
|--------|---------|
| `_refreshPorts()` | Discover available serial ports |
| `_toggleConnection()` | Connect/disconnect from port |
| `_connect()` | Open port and start data streaming |
| `_disconnect()` | Close port gracefully |
| `_sendData()` | Transmit text or hex data |
| `_scheduleAutoSend()` | Start periodic transmission |
| `_processBatch()` | Flush batched RX data to UI |
| `_parseTerminalLine()` | Parse and display data |
| `build()` | Construct UI layout |

**Data Flow**:
1. User selects port and config
2. Calls `_connect()` → Rust FFI `openPort()`
3. Spawns Rust reader thread → streams data via `StreamSink`
4. Data arrives in `_dataSubscription` listener
5. Accumulated in `_pendingRxData` batch
6. `addPostFrameCallback` → `_processBatch()` → `setState()`
7. UI rebuilds with new terminal entries

### 3. Data Models

**File**: `lib/models/serial_config.dart` (~140 lines)

**SerialConfigModel**:
- Port name, baud rate, data bits, stop bits, parity
- DTR/RTS flags
- `copyWith()` for immutable updates
- Static lists: `baudRates`, `dataBitsOptions`, `stopBitsOptions`, `parityOptions`

**DisplayOptions**:
- Display mode (Text/Hex/Chart)
- Auto-scroll, line ending, timestamp flags
- Helper properties: `hexMode`, `chartMode`, `lineEndingChars`

**AutoSendSettings**:
- Interval (50ms-60,000ms)
- Byte delay (microseconds)
- Enabled flag, send counter

**TerminalEntry** (implied struct):
```dart
class TerminalEntry {
  final String data;
  final bool isTransmit;  // TX vs RX
  final DateTime timestamp;
  final bool isHex;
}
```

### 4. Theme System

**AppTheme** (`lib/theme/app_theme.dart`):
- Material Design 3 light theme
- Custom colors, typography, component styles
- 273 lines of theme configuration

**AppColors** (`lib/theme/app_colors.dart`):
- Primary: Sky blue accent
- Surface, background, border colors
- Text colors: primary, secondary, tertiary
- Status colors: success, error, warning

**AppTypography** (`lib/theme/app_typography.dart`):
- **Primary Font**: Plus Jakarta Sans (UI)
- **Terminal Font**: JetBrains Mono (code/data)
- Heading styles (h1-h4), body styles, labels
- Custom input hint and button styles

### 5. Sidebar Widgets

**Component**: `lib/widgets/sidebar/sidebar.dart`

**Sub-widgets**:
1. **Port Selector** - Dropdown with port list and refresh button
2. **Config Controls** - Baud rate, data bits, stop bits, parity
3. **Signals Display** - DTR/RTS toggle switches
4. **Display Options** - Mode selection, auto-scroll, timestamps
5. **Auto-Send Control** - Interval, start/stop buttons

### 6. Rust Backend

**Entry Point**: `rust/src/api/serial.rs` (~366 lines)

**Core Functions**:

```rust
pub fn list_serial_ports() -> Result<Vec<PortInfo>, String>
// Returns available USB serial ports with manufacturer/product info
// Platform-specific filtering (Windows, macOS, Linux)

pub fn open_port(config: SerialConfig) -> Result<String, String>
// Opens port, sets DTR/RTS, stores in global state
// 5ms timeout for responsive polling
// Returns error if port already open or invalid config

pub fn close_port(port_name: String) -> Result<String, String>
// Sets running flag to false, waits 200ms for thread cleanup
// Removes from global state

pub fn send_data(port_name: String, data: String, is_hex: bool, byte_delay_us: Option<u64>) -> Result<String, String>
// Parses data (text or hex mode), applies byte delays
// Hex parsing: "48 65 6C 6C 6F" → bytes [0x48, 0x65, ...]
// Returns sent byte count or error

pub fn stream_serial_data(port: String, baud: u32, sink: StreamSink<SerialData>) -> Result<String, String>
// Spawns tokio task to read from port continuously
// Buffers incomplete lines, sends complete frames to Dart
// Terminates when running flag set to false

pub fn is_port_open(port_name: String) -> bool
// Synchronous check if port in global state
```

**Data Structures** (`rust/src/api/models.rs`):

```rust
pub struct PortInfo {
    pub name: String,                    // Full path: /dev/tty.usbserial
    pub port_type: String,               // Short: usbserial
    pub manufacturer: Option<String>,    // USB info
    pub product: Option<String>,         // USB info
}

pub struct SerialConfig {
    pub port_name: String,
    pub baud_rate: u32,                  // 300-921,600
    pub data_bits: u8,                   // 5,6,7,8
    pub stop_bits: String,               // "1", "1.5", "2"
    pub parity: String,                  // "None", "Odd", "Even"
    pub dtr: bool,
    pub rts: bool,
}

pub struct SerialData {
    pub port_name: String,
    pub data: Vec<u8>,
    pub timestamp: u64,                  // Milliseconds since epoch
}
```

**Global State** (`rust/src/api/state.rs`):

```rust
struct SerialState {
    ports: parking_lot::Mutex<HashMap<String, Arc<Mutex<SerialPort>>>>,
    running: parking_lot::Mutex<HashMap<String, Arc<AtomicBool>>>,
}

static SERIAL_STATE: OnceCell<Arc<SerialState>> = OnceCell::new();
```

- Singleton pattern with `OnceCell`
- Thread-safe access via `parking_lot::Mutex`
- Per-port `AtomicBool` for graceful shutdown

### 7. FFI Bridge

**Generated Files** (Auto-generated, DO NOT EDIT):
- `lib/src/rust/frb_generated.dart` - Dart FFI stubs
- `rust/src/frb_generated.rs` - Rust FFI exports

**Configuration**: `flutter_rust_bridge.yaml`
```yaml
rust_input: crate::api        # Rust module to expose
rust_root: rust/              # Rust project root
dart_output: lib/src/rust     # Where to generate Dart code
```

**Regenerate**:
```bash
dart run build_runner build
```

## Data Flow Diagram

```
User Input (Select port, set config, type data)
    ↓
SerialScreen._handleXxx() callback
    ↓
setState() update local state
    ↓
Call Rust FFI function via RustLib
    ↓
Rust processes request (FFI callstack)
    ↓
For streaming: Rust spawns tokio task
    ↓
Reader thread reads from serial port
    ↓
Accumulates bytes, detects line endings
    ↓
Sends SerialData through StreamSink
    ↓
Dart listener receives Stream event
    ↓
Batches in _pendingRxData
    ↓
addPostFrameCallback schedules flush
    ↓
_processBatch() converts to TerminalEntry
    ↓
setState() rebuilds UI
    ↓
Terminal display shows with TX/RX badges
```

## File Size Analysis

| File | Lines | Purpose |
|------|-------|---------|
| serial_screen.dart | 721 | Main UI orchestration |
| app_theme.dart | 273 | Theme configuration |
| serial.rs | 366 | Serial I/O backend |
| serial_config.dart | 140 | Data models |
| Other Dart | ~300 | Widgets, theme, models |
| Other Rust | ~100 | State, models, utilities |
| **Total Source** | ~1,900 | Not including generated/deps |

## Dependency Graph

```
main.dart
├─ RustLib.init()
├─ SerialScreen
│  ├─ app_theme.dart
│  ├─ serial_config.dart (models)
│  ├─ widgets/sidebar/*.dart
│  ├─ widgets/chart/realtime_chart.dart
│  └─ src/rust/api/serial.dart (FFI)
│
├─ google_fonts (font loading)
└─ cupertino_icons (icons)

Rust lib.rs
└─ api/
   ├─ serial.rs (main logic)
   ├─ models.rs (data structures)
   ├─ state.rs (global state)
   └─ simple.rs (utilities)
```

## Key Algorithms

### 1. Line-Based Parsing (Rust Reader Thread)

```
Incoming bytes → RX buffer
Loop:
  Check for line ending (CR, LF, CRLF)
  If found:
    Extract complete line
    Create SerialData struct
    Send through StreamSink to Dart
    Clear buffer
  Else:
    Continue accumulating
```

### 2. High-Frequency Data Batching (Dart)

```
StreamListener receives SerialData
  Add to _pendingRxData batch
  If not already scheduled:
    Schedule addPostFrameCallback()
  Return

addPostFrameCallback fires:
  For each batch item:
    Parse line (hex, text, timestamp)
    Create TerminalEntry
    Add to _terminalEntries
  Clear batch
  setState() → rebuild UI
```

### 3. Platform-Specific Port Filtering (Rust)

```
list_serial_ports():
  Get all ports via serialport crate
  Filter by port type (USB only)
  Platform-specific name matching:
    macOS: /dev/tty.*
    Windows: COM*
    Linux: /dev/ttyUSB*, /dev/ttyACM*, /dev/ttyS*
  Extract USB manufacturer/product info
  Return filtered list
```

## Configuration Files

### pubspec.yaml
```yaml
flutter: ^3.24
dependencies:
  flutter_rust_bridge: 2.11.1
  google_fonts: ^7.0.1
  termipro_rust: {path: rust_builder}
```

### Cargo.toml (Rust)
```toml
[dependencies]
flutter_rust_bridge = "2.11.1"
serialport = "4.3"
tokio = { version = "1", features = ["rt-multi-thread", "sync", "time"] }
parking_lot = "0.12"
once_cell = "1.19"
serde = { version = "1", features = ["derive"] }

[profile.release]
opt-level = 3
lto = true
codegen-units = 1
```

### flutter_rust_bridge.yaml
```yaml
rust_input: crate::api
rust_root: rust/
dart_output: lib/src/rust
namespace_dart: relative
```

## Build Process

```
flutter pub get
    ↓ (install Dart dependencies)

dart run build_runner build
    ↓ (generate FFI bindings, Freezed classes)

cargo build --release
    ↓ (compile Rust → dylib/dll/so)

flutter build <platform>
    ↓ (bundle app with Rust library)

Final binary with embedded Rust FFI
```

## Testing Structure

```
test/
├── serial_config_test.dart      # Model tests
└── widget_test.dart             # Widget tests

integration_test/
└── simple_test.dart             # Full app integration

test_driver/
└── integration_test.dart        # Driver for integration tests
```

## Performance Characteristics

| Aspect | Target | Method |
|--------|--------|--------|
| Serial data rate | Up to 921,600 bps | serialport library support |
| UI responsiveness | 60 FPS | Batching + frame-based updates |
| Memory | < 150MB | Efficient buffers, limited history |
| Startup | < 2s | Pre-compiled Rust, lazy init |
| Data latency | < 50ms | 5ms read timeout + frame timing |

## Known Limitations & Future Work

### Current Limitations (v1.0)
- Single port monitoring only (multi-port planned v1.2)
- No data logging (planned v1.1)
- Light theme only (dark mode planned v1.2)
- No session save/load (planned v1.1)
- No iOS/Android builds (planned v2.0)

### Architecture for Future Features
- **Plugin system**: Add `plugin/` directory with trait-based design
- **Data logging**: File writer module with rotation
- **Multi-port**: Refactor state management to support multiple ports
- **Dark theme**: Add second ThemeData to AppTheme
- **Network serial**: Add network module to Rust backend
- **Protocol analyzers**: Plugin-based modular design

## Documentation Index

- **README.md** - Quick start, setup, build instructions
- **project-overview-pdr.md** - Requirements, features, acceptance criteria
- **system-architecture.md** - FFI architecture, threading, data flow
- **code-standards.md** - Naming, style, patterns for Dart and Rust
- **codebase-summary.md** - This file, code overview

## Quick Navigation

- Add new serial command: Edit `rust/src/api/serial.rs` + regenerate FFI
- Add UI component: Create in `lib/widgets/` + use in SerialScreen
- Change colors: Update `lib/theme/app_colors.dart`
- Change fonts: Update `lib/theme/app_typography.dart`
- Debug serial issue: Check `rust/src/api/serial.rs` error handling

---

**Document Version**: 1.0
**Last Updated**: January 13, 2025
**Codebase Size**: ~2,500 LOC (excluding generated code and dependencies)
