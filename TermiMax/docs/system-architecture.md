# TermiMax Flutter - System Architecture

## Architecture Overview

TermiMax Flutter employs a layered, cross-language architecture combining Flutter (UI) with Rust (I/O), connected through flutter_rust_bridge FFI bindings.

```
┌─────────────────────────────────────────────────┐
│              Flutter UI Layer                    │
│  (SerialScreen, Widgets, Theme, Models)         │
└──────────────────┬──────────────────────────────┘
                   │ FFI Binding
                   ↓
┌─────────────────────────────────────────────────┐
│          flutter_rust_bridge                     │
│   (frb_generated.dart ↔ frb_generated.rs)       │
└──────────────────┬──────────────────────────────┘
                   │ Native Function Calls
                   ↓
┌─────────────────────────────────────────────────┐
│              Rust Backend                        │
│  (Serial API, State Management, I/O Threads)    │
└──────────────────┬──────────────────────────────┘
                   │ Platform Calls
                   ↓
┌─────────────────────────────────────────────────┐
│          OS Serial Port Driver                   │
│    (macOS, Windows, Linux kernel drivers)       │
└─────────────────────────────────────────────────┘
```

## Layer Breakdown

### 1. Flutter UI Layer

**Location**: `lib/`

**Components**:
- **SerialScreen** (`screens/serial_screen.dart`): Main stateful widget
- **Widgets** (`widgets/`): Sidebar, chart, custom dropdowns
- **Models** (`models/serial_config.dart`): Data classes
- **Theme** (`theme/`): Colors, typography, Material Design 3

**Responsibilities**:
- User interaction handling
- Data display and visualization
- State management via callbacks
- Error presentation to user

**Key Technologies**:
- Flutter 3.10.7+
- Material Design 3
- google_fonts for typography

### 2. FFI Bridge Layer

**Location**: `lib/src/rust/` (auto-generated), `rust/src/frb_generated.rs`

**Purpose**: Bidirectional communication between Dart and Rust

**Generated Files**:
- `frb_generated.dart`: Dart FFI binding stubs
- `frb_generated.rs`: Rust FFI function implementations

**Communication Patterns**:
1. **One-way calls**: Dart → Rust (function calls)
   ```dart
   final result = await listSerialPorts();
   ```

2. **Streaming**: Rust → Dart (StreamSink for data)
   ```dart
   _dataSubscription = streamSerialData(port, baud).listen(onData);
   ```

**Configuration**: `flutter_rust_bridge.yaml`
```yaml
rust_input: crate::api
rust_root: rust/
dart_output: lib/src/rust
```

### 3. Rust Backend Layer

**Location**: `rust/src/api/`

**Core Modules**:
- **serial.rs**: Serial port I/O logic
- **models.rs**: Data structures (SerialConfig, PortInfo, SerialData)
- **state.rs**: Global state management
- **simple.rs**: Utility/demo functions

**Key Functions**:

| Function | Purpose | Thread Context |
|----------|---------|-----------------|
| `list_serial_ports()` | Discover ports | Synchronous |
| `open_port(config)` | Initialize connection | Synchronous + spawns reader |
| `close_port(port_name)` | Shutdown connection | Synchronous |
| `send_data(...)` | Transmit data | Synchronous |
| `stream_serial_data(...)` | Receive data stream | Async/Threaded |
| `is_port_open(port_name)` | Check status | Synchronous |

## Threading Model

### Rust Threading Architecture

```
Main Thread (Dart/FFI)
├─ Sync API calls: list_serial_ports(), open_port(), send_data()
└─ Async spawn: stream_serial_data() → tokio::spawn()
   ↓
Reader Thread (tokio task per port)
├─ Read from serial port
├─ Parse/buffer data
└─ Send via StreamSink to Dart
```

### Data Reception Flow

1. **Rust Reader Thread** (per port):
   - Spawned in tokio multi-threaded runtime
   - Reads from serial port in 5ms timeout blocks
   - Buffers incomplete lines
   - Sends complete frames via StreamSink

2. **Dart Main Thread**:
   - Receives StreamSink events
   - Schedules UI update via `addPostFrameCallback`
   - Batches multiple events before rendering

### Synchronization Primitives

**Global State** (`OnceCell<Arc<SerialState>>`):
```rust
struct SerialState {
    ports: parking_lot::Mutex<HashMap<String, Arc<Mutex<SerialPort>>>>,
    running: parking_lot::Mutex<HashMap<String, Arc<AtomicBool>>>,
}
```

**Per-Port Coordination**:
- Port mutex: Guards actual serial port read/write
- Running flag: AtomicBool for graceful shutdown
- 200ms shutdown grace period for thread cleanup

## State Management Architecture

### Dart State (SerialScreen)

**Local State Variables**:
```dart
List<PortInfo> _ports;              // Available ports
String? _selectedPort;              // Currently selected port
bool _isConnected;                  // Connection status
SerialConfigModel _config;          // Serial configuration
DisplayOptions _displayOptions;     // UI display settings
AutoSendSettings _autoSendSettings; // Auto-send config

List<TerminalEntry> _terminalEntries;
int _txCount, _rxCount;
List<double> _chartData;
```

**Data Flow**:
```
User Input (button click, dropdown select)
    ↓
State Update (setState)
    ↓
Call Rust API (FFI)
    ↓
Update Local State
    ↓
Rebuild UI
```

### Rust State (Global)

**Thread-Safe Access**:
```rust
static SERIAL_STATE: OnceCell<Arc<SerialState>> = OnceCell::new();

fn get_state() -> Arc<SerialState> {
    SERIAL_STATE.get_or_init(|| Arc::new(SerialState::new())).clone()
}
```

**Port Storage**:
- Key: Port name (String)
- Value: `Arc<Mutex<SerialPort>>` (thread-safe handle)
- Allows multiple readers, single writer safety

## Data Flow Diagrams

### Connection Establishment

```
User → Select Port & Config
    ↓
_connect() method
    ↓
Create SerialConfig struct
    ↓
Call Rust: open_port(config) [FFI]
    ↓
Rust: Validate config
    ↓
Rust: Open serial port, set DTR/RTS
    ↓
Rust: Store in global state
    ↓
Dart: Set _isConnected = true
    ↓
UI: Show "Connected" indicator
```

### Data Reception

```
Rust Reader Thread: Reads from port
    ↓ (5ms timeout blocks)
Accumulate bytes into RX buffer
    ↓
Check for line ending (CR/LF/CRLF)
    ↓ (when complete line found)
Send SerialData via StreamSink
    ↓ (Dart receives event)
Dart: Add to _pendingRxData batch
    ↓ (if not scheduled)
Schedule addPostFrameCallback()
    ↓ (next frame)
Flush batch → Add to _terminalEntries
    ↓
setState() → Rebuild UI
    ↓
Display with TX/RX badge
```

### Data Transmission

```
User → Input text/hex data
    ↓
User clicks Send button
    ↓
_sendData() method
    ↓
Parse mode (text or hex)
    ↓
Call Rust: send_data(port, data, is_hex, delay)
    ↓
Rust: Lock port mutex
    ↓
Write bytes to serial port
    ↓
Apply inter-byte delay if configured
    ↓
Return success/error
    ↓
Dart: Create TerminalEntry with yellow TX badge
    ↓
Add to _terminalEntries
    ↓
Rebuild UI with new entry
```

## Batching System (Performance Optimization)

### Problem

High-frequency serial data (100+ Hz) creates UI lag when updating for every byte received.

### Solution

Frame-based batching:

```dart
// In SerialData stream listener
_pendingRxData.add(data.data);

if (!_updateScheduled) {
  _updateScheduled = true;
  WidgetsBinding.instance.addPostFrameCallback((_) {
    _flushBatch();
    _updateScheduled = false;
  });
}

void _flushBatch() {
  // Process all accumulated data at once
  for (final dataList in _pendingRxData) {
    // Parse lines and add to terminal
  }
  _pendingRxData.clear();
  setState(() {}); // Single UI update
}
```

**Benefits**:
- Reduces UI rebuilds from per-packet to per-frame
- Maintains 60 FPS even with 100+ Hz data
- Improves battery life on mobile
- Smoother terminal scrolling

## Configuration & Build System

### Build Process

```
flutter pub get
    ↓ (downloads dependencies)
dart run build_runner build
    ↓ (generates FFI bindings)
cargo build --release
    ↓ (compiles Rust backend)
flutter build macos/windows/linux
    ↓ (bundles Rust library + Flutter UI)
```

### Platform-Specific Building

**macOS**:
- Rust target: `aarch64-apple-darwin` (M1/M2) or `x86_64-apple-darwin`
- Output: `.app` bundle with embedded dylib

**Windows**:
- Rust target: `x86_64-pc-windows-msvc`
- Output: `.exe` with .dll dependencies

**Linux**:
- Rust target: `x86_64-unknown-linux-gnu`
- Output: Binary with .so dependencies

## Error Handling Architecture

### Rust Layer

**Error Patterns**:
```rust
pub fn open_port(config: SerialConfig) -> Result<String, String> {
    // Validation errors
    // I/O errors → detailed messages
    // State errors (already open)
}
```

**Special Error Cases**:
- Port busy: Returns "BUSY:port_name" for special handling
- Permission denied: Clear message about USB driver
- Timeout during read: Graceful recovery, retry

### Dart Layer

**Error Handling**:
```dart
try {
  await _connect();
} catch (e) {
  _showError('Failed to connect: $e');
  setState(() => _isConnected = false);
}
```

**User Feedback**:
- SnackBar messages for non-blocking errors
- AlertDialog for critical errors
- Status indicators for connection issues

## Performance Characteristics

### Metrics

| Aspect | Target | Method |
|--------|--------|--------|
| Data rate | 921,600 bps | serialport library native support |
| UI responsiveness | 60 FPS @ 100 Hz | Batching system |
| Memory | < 150MB | Efficient data structures, no buffering |
| Latency | < 50ms | 5ms serial read timeout + frame timing |
| Startup | < 2s | Pre-compiled Rust, lazy initialization |

### Bottlenecks & Mitigations

| Bottleneck | Mitigation |
|-----------|-----------|
| High serial data rate | Batching, efficient parsing |
| String allocation overhead | Reuse buffers, minimal copying |
| UI rebuild overhead | Only full screen rebuild, not per-widget |
| Serialization cost | Direct byte handling, lazy parsing |

## Security Considerations

### Data Safety

- No persistent data storage in v1.0
- No encryption of serial data (pass-through only)
- No logging to disk by default

### Access Control

- Serial ports: Delegated to OS (macOS requires permission)
- State access: Thread-safe via mutex and atomic types
- No external network access

### Future Considerations (v2.0+)

- Data logging: Implement user consent and clear visibility
- File handling: Follow platform security guidelines
- Remote serial: Implement TLS for network support

## Extensibility Points

### Adding New Serial Commands

1. **Rust**: Add function to `rust/src/api/serial.rs`
   ```rust
   pub fn new_command(args: String) -> Result<String, String> {
       // Implementation
   }
   ```

2. **FFI**: Expose in `rust/src/api/mod.rs`
   ```rust
   pub use serial::new_command;
   ```

3. **Regenerate**: `dart run build_runner build`

4. **Dart**: Call from `lib/screens/serial_screen.dart`
   ```dart
   final result = await newCommand(args);
   ```

### Adding Display Modes

1. Add mode string to `DisplayOptions.displayModes`
2. Implement parsing logic in `_parseTerminalLine()`
3. Create display widget in build tree
4. Add mode selection to sidebar

### Adding Platform Support

1. Configure Flutter platform in `pubspec.yaml`
2. Add Rust compilation targets in `Cargo.toml`
3. Handle platform-specific paths (serial ports)
4. Test on target platform

## Testing Architecture

### Unit Tests (Rust)

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn test_port_open_invalid_config() { }

    #[test]
    fn test_hex_parsing() { }
}
```

### Integration Tests

```dart
testWidgets('Serial connection flow', (tester) async {
    await tester.pumpWidget(const MyApp());
    // Test full connection cycle
});
```

### Manual Testing

- Real USB devices
- Various baud rates
- High-frequency data streams
- Error conditions

## Deployment Architecture

### Development Build

```
flutter run
├─ Attaches debugger
├─ Hot reload enabled
└─ Unoptimized Rust
```

### Release Build

```
flutter build <platform> --release
├─ Optimized Rust (lto = true, opt-level = 3)
├─ Tree-shaken Dart
├─ Stripped symbols
└─ ~50-100MB app size (platform-dependent)
```

## Version History

| Version | Date | Changes |
|---------|------|---------|
| 1.0 | Jan 2025 | Initial architecture definition |

---

**Document Version**: 1.0
**Last Updated**: January 13, 2025
