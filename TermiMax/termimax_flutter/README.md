# TermiMax Flutter - Professional Serial Monitor

A high-performance, cross-platform serial monitor built with Flutter and Rust. Professional-grade interface for embedded systems development and serial port communication.

## Overview

TermiMax Flutter provides a modern, user-friendly serial terminal with real-time data visualization, hex/text modes, and advanced configuration options. Built with Flutter for the UI and Rust for high-speed serial I/O operations.

**Key Features:**
- Real-time serial port monitoring with TX/RX statistics
- Text, Hex, and Chart display modes
- Configurable line endings (None, CR, LF, CRLF)
- Auto-send functionality with adjustable intervals
- USB serial port detection (platform-aware)
- Full serial configuration (baud rate, data bits, stop bits, parity, DTR/RTS control)
- Timestamp display for received data
- High-frequency data batching for smooth UI performance

## Supported Platforms

- macOS (primary)
- Windows
- Linux (planned)
- iOS (planned)
- Android (planned)

## Quick Start

### Prerequisites

- Flutter SDK 3.10.7+
- Rust 1.70+ with cargo
- XCode (macOS) / Visual Studio (Windows) / build-essential (Linux)
- Xcode Command Line Tools (macOS)

### Installation

1. Clone the repository

```bash
cd termimax_flutter
```

2. Install dependencies

```bash
flutter pub get
```

3. Generate Rust bindings (if needed)

```bash
dart run build_runner build
```

### Running the App

**Development mode:**

```bash
flutter run
```

**Desktop (macOS):**

```bash
flutter run -d macos
```

**Desktop (Windows):**

```bash
flutter run -d windows
```

**Build for release:**

```bash
# macOS
flutter build macos --release

# Windows
flutter build windows --release

# Linux
flutter build linux --release
```

## Architecture Overview

The application follows a layered architecture:

- **Flutter UI Layer** (`lib/screens/`, `lib/widgets/`): Single-screen StatefulWidget with callback-based state management
- **Data Models** (`lib/models/`): Configuration and display option models with copyWith patterns
- **Theme System** (`lib/theme/`): Material Design 3 theming with Plus Jakarta Sans + JetBrains Mono fonts
- **Rust Backend** (`rust/src/api/`): FFI-exposed serial port management with thread-safe global state
- **FFI Bridge** (`lib/src/rust/`): Auto-generated flutter_rust_bridge bindings

## Project Structure

```
termimax_flutter/
├── lib/
│   ├── main.dart                      # Entry point + RustLib initialization
│   ├── screens/
│   │   └── serial_screen.dart         # Main app screen (~721 lines)
│   ├── models/
│   │   └── serial_config.dart         # Config and display models
│   ├── theme/
│   │   ├── app_theme.dart            # Material Design 3 theme
│   │   ├── app_colors.dart           # Color palette
│   │   ├── app_typography.dart       # Font styles
│   │   └── theme.dart                # Theme exports
│   ├── widgets/
│   │   ├── sidebar/                  # Port selector, config controls
│   │   ├── chart/                    # Real-time data visualization
│   │   └── common/                   # Custom dropdown components
│   └── src/rust/                     # Auto-generated FFI bindings
├── rust/
│   ├── Cargo.toml
│   ├── src/
│   │   ├── lib.rs                    # Entry point
│   │   ├── api/
│   │   │   ├── mod.rs               # Module exports
│   │   │   ├── serial.rs            # Serial port logic (366 lines)
│   │   │   ├── models.rs            # Data structures
│   │   │   ├── state.rs             # Global state management
│   │   │   └── simple.rs            # Demo functions
│   │   └── frb_generated.rs          # Auto-generated
│   └── target/                       # Compiled binaries
├── pubspec.yaml                      # Dependencies
├── flutter_rust_bridge.yaml          # FFI configuration
└── docs/                             # Documentation
```

## Configuration

### Serial Port Configuration

The app supports full serial port configuration:

| Parameter | Range | Default |
|-----------|-------|---------|
| Baud Rate | 300 - 921,600 | 115,200 |
| Data Bits | 5, 6, 7, 8 | 8 |
| Stop Bits | 1, 1.5, 2 | 1 |
| Parity | None, Odd, Even | None |
| DTR | Enable/Disable | Enabled |
| RTS | Enable/Disable | Enabled |

### Display Options

- **Display Mode**: Text, Hex, or Chart (real-time data visualization)
- **Auto-scroll**: Automatically scroll to latest messages
- **Line Ending**: None, CR, LF, or CRLF when sending
- **Timestamps**: Show/hide received data timestamps

### Auto-Send Configuration

- **Interval**: 50ms - 60,000ms between sends
- **Byte Delay**: Microsecond delay between individual bytes
- **Send Counter**: Track total sends during session

## Dart/Flutter Dependencies

| Package | Version | Purpose |
|---------|---------|---------|
| `flutter_rust_bridge` | 2.11.1 | FFI code generation |
| `google_fonts` | ^7.0.1 | Font management |
| `cupertino_icons` | ^1.0.8 | iOS-style icons |
| `termipro_rust` | path: rust_builder | Rust FFI library |

## Rust Dependencies

| Crate | Version | Purpose |
|-------|---------|---------|
| `flutter_rust_bridge` | 2.11.1 | FFI bindings |
| `serialport` | 4.3 | Serial port I/O |
| `tokio` | 1 | Async runtime |
| `serde` | 1 | Serialization |
| `parking_lot` | 0.12 | Synchronization |
| `once_cell` | 1.19 | Global state |

## Key Components

### SerialScreen (lib/screens/serial_screen.dart)

Main stateful widget managing:
- Port listing and connection state
- Data reception with batching system
- Terminal display and scrolling
- Chart data accumulation
- Error handling and user feedback

### Serial Backend (rust/src/api/serial.rs)

Core functions:
- `list_serial_ports()` - Platform-aware port discovery
- `open_port(config)` - Open connection with config
- `close_port(port_name)` - Graceful shutdown
- `send_data(port, data, is_hex, byte_delay)` - TX with mode support
- `stream_serial_data(port, baud, sink)` - RX streaming
- `is_port_open(port_name)` - Status check

### State Management

Thread-safe global state using:
- `OnceCell<Arc<SerialState>>` for singleton state
- `parking_lot::Mutex<HashMap<String, Arc<Mutex<SerialPort>>>>` for ports
- `Arc<AtomicBool>` per-port running flags

## Development Workflow

### Adding New Serial Features

1. Add Rust function to `rust/src/api/serial.rs`
2. Expose in `rust/src/api/mod.rs`
3. Regenerate bindings: `dart run build_runner build`
4. Call from Flutter using generated `lib/src/rust/api/serial.dart`

### Styling

- Use `AppColors` for all colors (app_colors.dart)
- Use `AppTypography` for font styles (app_typography.dart)
- Material Design 3 tokens defined in `AppTheme`

### Adding New Screens

1. Create file in `lib/screens/`
2. Use callback pattern for state management (no Provider/Riverpod)
3. Apply `AppTheme.light` from theme configuration

## Building for Distribution

### macOS

```bash
flutter build macos --release
# Output: build/macos/Build/Products/Release/termipro.app
```

### Windows

```bash
flutter build windows --release
# Output: build/windows/runner/Release/
```

### Deployment

- Requires code signing for macOS distribution
- Windows build includes MSVC runtime dependencies
- Rust FFI library is embedded in app bundle

## Testing

Run integration tests:

```bash
flutter test
flutter drive --target=test_driver/integration_test.dart
```

## Documentation

- `docs/project-overview-pdr.md` - Product requirements and features
- `docs/system-architecture.md` - FFI architecture and threading model
- `docs/code-standards.md` - Dart and Rust coding conventions
- `docs/codebase-summary.md` - Comprehensive codebase overview

## Troubleshooting

**Port not appearing in list:**
- Verify USB driver installed
- Check USB permissions on macOS: `ls -la /dev/tty.*`

**Connection fails with "BUSY" error:**
- Port may be in use by another application
- Try different port or restart device

**Data not displaying:**
- Check baud rate matches device configuration
- Verify line ending settings match device output
- Ensure DTR/RTS settings are correct for device

**High CPU usage:**
- Reduce chart update frequency
- Disable timestamps if not needed
- Close unused port connections

## License

Proprietary - TermiMax Professional Serial Monitor

## Version

v1.0.0+1

---

**Last Updated:** January 2025
**Status:** Active Development
