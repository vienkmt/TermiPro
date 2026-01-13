# TermiMax Flutter - Code Standards & Conventions

## Overview

This document defines coding conventions and standards for the TermiMax Flutter project, covering both Dart (Flutter) and Rust backend code.

## Dart/Flutter Conventions

### File Organization

**Location Structure**:
```
lib/
├── main.dart                          # Single app entry point
├── screens/
│   └── serial_screen.dart            # Main screen only
├── models/
│   └── serial_config.dart            # Data models
├── widgets/
│   ├── sidebar/
│   │   ├── port_selector.dart
│   │   └── config_controls.dart
│   ├── chart/
│   │   └── realtime_chart.dart
│   └── common/
│       └── custom_dropdown.dart
├── theme/
│   ├── app_theme.dart
│   ├── app_colors.dart
│   ├── app_typography.dart
│   └── theme.dart
└── src/rust/                         # Auto-generated FFI (DO NOT EDIT)
```

**Guidelines**:
- One widget per file (except small private classes)
- Filenames: `snake_case.dart`
- Classes: `PascalCase`
- Related widgets in subdirectories

### Naming Conventions

**Variables & Functions**:
```dart
// Good: camelCase for variables and functions
final String portName = "COM1";
void openSerialPort() { }
int _internalCounter = 0;  // Leading underscore for private

// Bad: PascalCase for variables
final String PortName = "COM1";
void OpenSerialPort() { }
```

**Classes & Types**:
```dart
// Good: PascalCase for classes and enums
class SerialScreen extends StatefulWidget { }
enum DisplayMode { text, hex, chart }

// Bad: camelCase for classes
class serialScreen extends StatefulWidget { }
```

**Constants**:
```dart
// Good: camelCase (not SCREAMING_SNAKE_CASE)
const int defaultBaudRate = 115200;
const String appTitle = 'TermiMax';

// Bad: SCREAMING_SNAKE_CASE
const int DEFAULT_BAUD_RATE = 115200;
```

**Private Members**:
```dart
class SerialScreen extends StatefulWidget {
  // Private members: leading underscore
  late TextEditingController _sendController;
  List<PortInfo> _ports = [];

  // Public members: no underscore
  final String appVersion = '1.0.0';
}
```

### Code Structure

**Widget Class Template**:
```dart
/// Brief description of widget purpose
class MyWidget extends StatefulWidget {
  const MyWidget({super.key});

  @override
  State<MyWidget> createState() => _MyWidgetState();
}

class _MyWidgetState extends State<MyWidget> {
  // State variables
  late TextEditingController _controller;

  // Lifecycle methods
  @override
  void initState() {
    super.initState();
    _controller = TextEditingController();
  }

  @override
  void dispose() {
    _controller.dispose();
    super.dispose();
  }

  // State management methods
  void _handleAction() {
    setState(() {
      // Update state
    });
  }

  // Build method (last)
  @override
  Widget build(BuildContext context) {
    return Scaffold(
      // Build UI
    );
  }
}
```

**Method Ordering**:
1. Constructor
2. `initState()` / lifecycle methods
3. State modification methods
4. Helper methods
5. `build()` method (always last)

### Documentation

**Doc Comments**:
```dart
/// Brief one-line description ending with period.
///
/// Longer description explaining the widget's purpose,
/// parameters, and behavior. Can span multiple lines.
///
/// Example usage:
/// ```dart
/// MyWidget(
///   title: 'Example',
/// )
/// ```
class MyWidget extends StatelessWidget {
  /// The title to display
  final String title;

  const MyWidget({required this.title});

  @override
  Widget build(BuildContext context) {
    return Text(title);
  }
}
```

**Comments**:
```dart
// Good: Explains WHY, not WHAT
// Batch data updates to prevent UI jank with high-frequency streams
if (!_updateScheduled) {
  _updateScheduled = true;
  // ...
}

// Bad: Obvious from code
int count = 0; // Set count to 0
```

### Error Handling

**Async Error Handling**:
```dart
// Good: Explicit error handling with context
Future<void> _connect() async {
  try {
    final rustConfig = SerialConfig(
      portName: _selectedPort!,
      // ... config
    );
    await openPort(rustConfig);
    setState(() => _isConnected = true);
  } on Exception catch (e) {
    _showError('Failed to connect: $e');
    setState(() => _isConnected = false);
  }
}

// Bad: Silent failure
Future<void> _connect() async {
  await openPort(config);
  setState(() => _isConnected = true);
}
```

**Error Display**:
```dart
void _showError(String message) {
  ScaffoldMessenger.of(context).showSnackBar(
    SnackBar(
      content: Text(message),
      backgroundColor: AppColors.error,
      duration: const Duration(seconds: 4),
    ),
  );
}
```

### Imports

**Organization**:
```dart
// 1. Dart imports
import 'dart:async';
import 'dart:convert';

// 2. Flutter imports
import 'package:flutter/material.dart';
import 'package:google_fonts/google_fonts.dart';

// 3. Local imports
import 'package:termimax_flutter/models/serial_config.dart';
import 'package:termimax_flutter/theme/theme.dart';

// 4. FFI imports (at end)
import 'package:termimax_flutter/src/rust/api/serial.dart';
```

**Relative vs Absolute**:
```dart
// Good: Always use absolute imports
import 'package:termimax_flutter/models/serial_config.dart';

// Bad: Relative imports
import 'models/serial_config.dart';
```

### State Management Patterns

**Callback Pattern** (used in TermiMax):
```dart
class SerialScreen extends StatefulWidget {
  const SerialScreen({super.key});

  @override
  State<SerialScreen> createState() => _SerialScreenState();
}

class _SerialScreenState extends State<SerialScreen> {
  void _handlePortChange(String newPort) {
    setState(() {
      _selectedPort = newPort;
    });
  }

  @override
  Widget build(BuildContext context) {
    return PortSelector(
      selectedPort: _selectedPort,
      onPortChanged: _handlePortChange,  // Callback
    );
  }
}

class PortSelector extends StatelessWidget {
  final String? selectedPort;
  final Function(String) onPortChanged;

  const PortSelector({
    required this.selectedPort,
    required this.onPortChanged,
  });

  @override
  Widget build(BuildContext context) {
    return DropdownButton(
      value: selectedPort,
      onChanged: (value) => onPortChanged(value ?? ''),
    );
  }
}
```

**Avoid**:
- Provider/Riverpod (too heavy for single-screen app)
- GetX (not idiomatic Dart)
- Global state variables (reduces testability)

### Theme & Styling

**Color Usage**:
```dart
// Good: Use AppColors constants
Container(
  color: AppColors.primary,  // Defined in app_colors.dart
  child: Text(
    'Hello',
    style: TextStyle(color: AppColors.textOnPrimary),
  ),
)

// Bad: Inline colors
Container(
  color: const Color(0xFF00B4D8),
  child: Text(
    'Hello',
    style: TextStyle(color: const Color(0xFFFFFFFF)),
  ),
)
```

**Typography Usage**:
```dart
// Good: Use AppTypography styles
Text(
  'Terminal Output',
  style: AppTypography.bodyMedium,
)

// Bad: Define styles inline
Text(
  'Terminal Output',
  style: TextStyle(
    fontSize: 14,
    fontFamily: 'JetBrains Mono',
  ),
)
```

**Spacing**:
```dart
// Use consistent spacing constants
const double _padding = 16;
const double _spacing = 8;

Padding(
  padding: const EdgeInsets.all(_padding),
  child: Row(
    spacing: _spacing,
    children: [...],
  ),
)
```

### Performance Best Practices

**Const Widgets**:
```dart
// Good: Mark constructors as const
class MyWidget extends StatelessWidget {
  const MyWidget({super.key});  // const constructor

  @override
  Widget build(BuildContext context) {
    return const Text('Hello');  // const widget
  }
}

// Good: Const keyword for widgets
final widgets = [
  const SizedBox(height: 16),
  const Divider(),
];
```

**Batching Updates**:
```dart
// Good: Batch multiple setState calls
void _processBatch(List<SerialData> batch) {
  setState(() {
    for (final data in batch) {
      _terminalEntries.add(_parseData(data));
      _rxCount += 1;
    }
  });
}

// Bad: Multiple setState calls
for (final data in batch) {
  setState(() => _terminalEntries.add(_parseData(data)));
}
```

**Stream Cleanup**:
```dart
// Good: Cancel subscriptions in dispose
@override
void dispose() {
  _dataSubscription?.cancel();
  _sendController.dispose();
  super.dispose();
}

// Bad: Leak resources
@override
void dispose() {
  _sendController.dispose();
  super.dispose();  // Never canceled _dataSubscription
}
```

## Rust Conventions

### File Organization

**Module Structure**:
```
rust/src/
├── lib.rs                    # Crate root, declare modules
├── api/
│   ├── mod.rs               # Module exports
│   ├── serial.rs            # Serial port operations
│   ├── models.rs            # Data structures
│   ├── state.rs             # Global state
│   └── simple.rs            # Demo/utilities
└── frb_generated.rs          # Auto-generated (DO NOT EDIT)
```

**Filenames**:
```rust
// Good: snake_case
serial.rs
models.rs
state.rs

// Bad: kebab-case
serial-port.rs
```

### Naming Conventions

**Functions**:
```rust
// Good: snake_case for functions
pub fn list_serial_ports() -> Result<Vec<PortInfo>, String> { }
fn get_timestamp() -> u64 { }

// Bad: camelCase
pub fn listSerialPorts() -> Result<Vec<PortInfo>, String> { }
```

**Structs & Enums**:
```rust
// Good: PascalCase for types
pub struct SerialData {
    pub port_name: String,
    pub data: Vec<u8>,
}

pub enum FlowControl {
    None,
    Software,
    Hardware,
}

// Bad: snake_case for types
pub struct serial_data {
    pub port_name: String,
}
```

**Constants & Statics**:
```rust
// Good: SCREAMING_SNAKE_CASE
pub const DEFAULT_TIMEOUT_MS: u64 = 5;
pub const MAX_CHART_POINTS: usize = 500;

// Bad: PascalCase or camelCase
pub const DefaultTimeoutMs: u64 = 5;
```

**Private Items**:
```rust
// Good: prefix with underscore for intentionally unused
fn _unused_helper() { }

// Unused parameters
fn process_data(_config: &SerialConfig) { }
```

### Error Handling

**Result Pattern**:
```rust
// Good: Return Result for fallible operations
pub fn open_port(config: SerialConfig) -> Result<String, String> {
    // Validate
    // Perform operation
    // Return Ok or Err
    Ok(format!("Port opened: {}", config.port_name))
}

// Good: Map errors with context
let port = serialport::new(&port_name, config.baud_rate)
    .open()
    .map_err(|e| format!("Failed to open {}: {}", port_name, e))?;

// Bad: Panic in library code
pub fn open_port(config: SerialConfig) -> String {
    let port = serialport::new(&config.port_name, config.baud_rate)
        .open()
        .expect("Should never fail");  // BAD: Panics in FFI!
    "OK".to_string()
}
```

**Error Messages**:
```rust
// Good: Descriptive, actionable errors
Err(format!("BUSY:{}", port_name))
Err(format!("Cannot set DTR: {}", e))
Err("Invalid data bits".to_string())

// Bad: Vague errors
Err("Error".to_string())
Err(e.to_string())  // Loses context
```

### Thread Safety

**Mutex Usage**:
```rust
// Good: parking_lot::Mutex (faster than std::Mutex)
let ports = parking_lot::Mutex::new(HashMap::new());
let mut guard = ports.lock();
guard.insert(port_name, port);

// Good: Arc for shared ownership
let port = Arc::new(parking_lot::Mutex::new(serial_port));

// OK: std::Mutex if needed
let guard = std::sync::Mutex::new(value);
```

**Atomic Operations**:
```rust
// Good: AtomicBool for simple flags
let running = Arc::new(AtomicBool::new(true));
running.store(false, Ordering::Relaxed);

// Good: Use appropriate Ordering
// Relaxed: No synchronization needed
// Release/Acquire: Cross-thread communication
if flag.load(Ordering::Acquire) {
    // critical section
}
```

**Global State**:
```rust
// Good: Using OnceCell for singleton pattern
use once_cell::sync::OnceCell;

static SERIAL_STATE: OnceCell<Arc<SerialState>> = OnceCell::new();

fn get_state() -> Arc<SerialState> {
    SERIAL_STATE.get_or_init(|| {
        Arc::new(SerialState::new())
    }).clone()
}
```

### Documentation

**Doc Comments**:
```rust
/// List available USB serial ports.
///
/// Performs platform-specific filtering to show only USB ports:
/// - Windows: COM ports
/// - macOS: /dev/tty.* devices
/// - Linux: /dev/ttyUSB*, /dev/ttyACM*, /dev/ttyS*
///
/// # Returns
/// A vector of `PortInfo` containing port details including
/// manufacturer and product information if available.
///
/// # Errors
/// Returns an error if port enumeration fails at OS level.
///
/// # Example
/// ```ignore
/// let ports = list_serial_ports()?;
/// for port in ports {
///     println!("Port: {}", port.port_type);
/// }
/// ```
pub fn list_serial_ports() -> Result<Vec<PortInfo>, String> {
    // Implementation
}
```

**Comments**:
```rust
// Good: Explain non-obvious behavior
// 1.5 stop bits not supported by serialport crate, map to 2
let stop_bits = match config.stop_bits.as_str() {
    "1.5" => StopBits::Two,
    // ...
};

// Bad: State the obvious
let x = 5; // Set x to 5
```

### Code Style

**Spacing & Formatting**:
```rust
// Good: Standard Rust formatting (use rustfmt)
pub fn send_data(
    port_name: String,
    data: String,
    is_hex: bool,
    byte_delay_us: Option<u64>,
) -> Result<String, String> {
    // Implementation
}

// Use `cargo fmt` to auto-format
```

**Match Expressions**:
```rust
// Good: Exhaustive, readable patterns
let stop_bits = match config.stop_bits.as_str() {
    "1" => StopBits::One,
    "1.5" => StopBits::Two,
    "2" => StopBits::Two,
    _ => return Err("Invalid stop bits".to_string()),
};

// Bad: Missing cases
let stop_bits = match config.stop_bits.as_str() {
    "1" => StopBits::One,
    _ => StopBits::Two,  // Falls through all others
};
```

**Result Propagation**:
```rust
// Good: Use ? operator
pub fn open_port(config: SerialConfig) -> Result<String, String> {
    let mut port = serialport::new(&config.port_name, config.baud_rate)
        .open()
        .map_err(|e| e.to_string())?;  // ? propagates error

    port.write_data_terminal_ready(config.dtr)
        .map_err(|e| format!("Cannot set DTR: {}", e))?;

    Ok("Success".to_string())
}

// Avoid: Excessive unwrap()
let port = serialport::new(&config.port_name, config.baud_rate)
    .open()
    .unwrap();  // DANGEROUS: panics in FFI!
```

### Performance Considerations

**Memory Allocation**:
```rust
// Good: Preallocate when size is known
let mut bytes = Vec::with_capacity(total_bytes);

// Good: Reuse buffers
let mut buffer = vec![0u8; 4096];
port.read(&mut buffer)?;

// Avoid: Allocating in tight loops
loop {
    let mut buf = vec![0u8; 1024];  // Allocates every iteration
    port.read(&mut buf)?;
}
```

**String Handling**:
```rust
// Good: Format once
let message = format!("Port {} opened", port_name);

// Avoid: Multiple allocations
let msg = "Port ".to_string() + &port_name + " opened";
```

## Testing

### Dart Test Example

```dart
void main() {
  group('SerialConfig', () {
    test('copyWith preserves values', () {
      const config = SerialConfigModel(baudRate: 115200);
      final updated = config.copyWith(dataBits: 7);

      expect(updated.baudRate, 115200);
      expect(updated.dataBits, 7);
    });
  });
}
```

### Rust Test Example

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_port_validation() {
        let config = SerialConfig {
            port_name: "COM1".to_string(),
            baud_rate: 115200,
            data_bits: 8,
            stop_bits: "1".to_string(),
            parity: "None".to_string(),
            dtr: true,
            rts: true,
        };

        // Assert validation passes
        assert!(validate_config(&config).is_ok());
    }
}
```

## Code Review Checklist

- [ ] Follows naming conventions (camelCase, snake_case, PascalCase)
- [ ] Proper error handling (no silent failures)
- [ ] Resource cleanup (dispose, cancel subscriptions)
- [ ] Thread safety (no race conditions)
- [ ] Documentation (doc comments for public API)
- [ ] No hardcoded values (use constants)
- [ ] Consistent code style (runs formatter)
- [ ] No panics in library code
- [ ] Proper imports organization
- [ ] Performance considerations addressed

## Tools & Automation

**Dart Formatting**:
```bash
dart format lib/
flutter analyze
```

**Rust Formatting**:
```bash
cargo fmt
cargo clippy
```

**CI Integration**:
```bash
# Run all checks
dart format --set-exit-if-changed lib/
flutter analyze
cargo fmt -- --check
cargo clippy --all-targets
```

---

**Document Version**: 1.0
**Last Updated**: January 13, 2025
