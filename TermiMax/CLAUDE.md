# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

**TermiMax** is a professional serial port monitoring application built with **Flutter + Rust**. It uses `flutter_rust_bridge` for FFI communication between Dart UI and Rust serial port backend.

- **Frontend**: Flutter (Dart 3.10.7+)
- **Backend**: Rust with `serialport` crate
- **Bridge**: flutter_rust_bridge 2.11.1
- **Platforms**: macOS (primary), Windows, Linux

## Build Commands

```bash
# Navigate to Flutter project
cd termimax_flutter

# Install dependencies
flutter pub get

# Run development
flutter run -d macos    # macOS
flutter run -d windows  # Windows
flutter run -d linux    # Linux

# Build production
flutter build macos --release
flutter build windows --release
flutter build linux --release

# Regenerate Rust FFI bindings (after modifying rust/src/api/)
flutter_rust_bridge_codegen generate
```

## Project Structure

```
TermiMax/
├── termimax_flutter/           # Main Flutter + Rust app
│   ├── lib/
│   │   ├── main.dart           # Entry point (RustLib.init())
│   │   ├── screens/
│   │   │   └── serial_screen.dart   # Main screen (721 LOC)
│   │   ├── models/
│   │   │   └── serial_config.dart   # Config models
│   │   ├── theme/              # Colors, typography, theme
│   │   ├── widgets/
│   │   │   ├── sidebar/        # Port selector, config, signals
│   │   │   ├── chart/          # Real-time chart
│   │   │   └── common/         # Custom dropdown
│   │   └── src/rust/           # Auto-generated FFI bindings
│   ├── rust/
│   │   └── src/
│   │       ├── lib.rs          # Entry
│   │       └── api/
│   │           ├── serial.rs   # Core serial logic (366 LOC)
│   │           ├── models.rs   # Data structures
│   │           └── state.rs    # Global state
│   ├── pubspec.yaml
│   └── flutter_rust_bridge.yaml
├── docs/                       # Documentation
├── plans/                      # Implementation plans
└── esp32_test/                 # Test firmware
```

## Architecture

### Data Flow
```
Flutter UI (StatefulWidget)
    ↓ (callback pattern)
lib/src/rust/api/serial.dart (generated)
    ↓ (FFI)
rust/src/api/serial.rs
    ↓
serialport crate → USB Serial Port
```

### Key Rust APIs (rust/src/api/serial.rs)

| Function | Purpose |
|----------|---------|
| `list_serial_ports()` | List USB ports (platform-aware) |
| `open_port(config)` | Open with baud, data bits, parity, etc. |
| `close_port(port_name)` | Graceful shutdown |
| `send_data(port, data, is_hex, byte_delay)` | TX with hex/text modes |
| `stream_serial_data(port, baud, sink)` | RX streaming via StreamSink |
| `is_port_open(port_name)` | Connection status check |

### State Management

- **Dart**: StatefulWidget with callback pattern (no Provider/Riverpod)
- **Rust**: Thread-safe global singleton
  - `OnceCell<Arc<SerialState>>` for lazy init
  - `parking_lot::Mutex` for port HashMap
  - `Arc<AtomicBool>` for per-port running flags

### Performance Patterns

1. **Batching**: High-frequency RX data batched via `addPostFrameCallback`
2. **Chunked TX**: 256-byte chunks with 500µs delay
3. **Terminal limit**: Max 500 entries (auto-trim oldest)
4. **Chart limit**: Max 500 data points

## Key Files

| File | Lines | Purpose |
|------|-------|---------|
| `lib/screens/serial_screen.dart` | 721 | Main UI + all state logic |
| `rust/src/api/serial.rs` | 366 | Core serial port operations |
| `lib/models/serial_config.dart` | 139 | Config + display option models |
| `lib/theme/app_theme.dart` | 273 | Material 3 theme config |

## Coding Conventions

### Dart
- **Naming**: camelCase vars, PascalCase classes, `_private` prefix
- **Models**: Immutable with `copyWith()` pattern
- **Widgets**: Stateless preferred, Stateful for local state
- **State updates**: Single `setState()` per logical change

### Rust
- **Naming**: snake_case, SCREAMING_CASE for constants
- **Error handling**: `Result<T, String>` for FFI functions
- **Thread safety**: Always use `Arc<Mutex<>>` for shared state
- **FFI exposure**: Public functions in `api/` module auto-exposed

## Dependencies

### Flutter (pubspec.yaml)
- `flutter_rust_bridge`: 2.11.1
- `google_fonts`: ^7.0.1
- `cupertino_icons`: ^1.0.8

### Rust (Cargo.toml)
- `flutter_rust_bridge`: =2.11.1
- `serialport`: 4.3
- `tokio`: 1 (rt-multi-thread, sync, time)
- `parking_lot`: 0.12
- `serde`, `serde_json`: 1

## Adding New Features

### New Rust API function:
1. Add function to `rust/src/api/serial.rs`
2. Export in `rust/src/api/mod.rs`
3. Run `flutter_rust_bridge_codegen generate`
4. Call from Dart via generated `lib/src/rust/api/serial.dart`

### New UI Widget:
1. Create in `lib/widgets/` appropriate subfolder
2. Use `AppColors` and `AppTypography` from theme/
3. Follow callback pattern for state communication
