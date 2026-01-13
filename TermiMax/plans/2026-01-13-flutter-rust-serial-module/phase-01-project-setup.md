# Phase 1: Project Setup & Rust Bridge

## Objective
Khởi tạo dự án Flutter + Rust với flutter_rust_bridge v2

---

## Tasks

### 1.1 Create Flutter Project với flutter_rust_bridge

```bash
# Cài đặt codegen tool
cargo install flutter_rust_bridge_codegen

# Tạo project mới
cd /Users/vienkmt/Dropbox/Project2025/TermiPro
flutter_rust_bridge_codegen create termipro_flutter

# Hoặc nếu muốn tạo Flutter project trước
flutter create termipro_flutter --platforms=macos,windows,linux
cd termipro_flutter
flutter_rust_bridge_codegen integrate
```

### 1.2 Configure Cargo.toml

```toml
# rust/Cargo.toml
[package]
name = "termipro_rust"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib", "staticlib"]

[dependencies]
flutter_rust_bridge = "2"
serialport = "4.3"
tokio = { version = "1", features = ["rt-multi-thread", "sync", "time", "macros"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
parking_lot = "0.12"
once_cell = "1.19"

[profile.release]
opt-level = 3
lto = true
codegen-units = 1
```

### 1.3 Configure pubspec.yaml

```yaml
# pubspec.yaml
name: termipro_flutter
description: Professional Serial Monitor

publish_to: 'none'

version: 1.0.0+1

environment:
  sdk: '>=3.0.0 <4.0.0'

dependencies:
  flutter:
    sdk: flutter
  flutter_rust_bridge: ^2.0.0
  flutter_riverpod: ^2.4.0
  riverpod_annotation: ^2.3.0
  google_fonts: ^6.1.0
  freezed_annotation: ^2.4.0

dev_dependencies:
  flutter_test:
    sdk: flutter
  flutter_lints: ^3.0.0
  freezed: ^2.4.0
  build_runner: ^2.4.0
  riverpod_generator: ^2.3.0

flutter:
  uses-material-design: true
```

### 1.4 macOS Entitlements

```xml
<!-- macos/Runner/Release.entitlements -->
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>com.apple.security.app-sandbox</key>
    <true/>
    <key>com.apple.security.device.usb</key>
    <true/>
    <key>com.apple.security.device.serial</key>
    <true/>
</dict>
</plist>
```

```xml
<!-- macos/Runner/DebugProfile.entitlements -->
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>com.apple.security.app-sandbox</key>
    <true/>
    <key>com.apple.security.device.usb</key>
    <true/>
    <key>com.apple.security.device.serial</key>
    <true/>
    <key>com.apple.security.cs.allow-jit</key>
    <true/>
    <key>com.apple.security.network.server</key>
    <true/>
</dict>
</plist>
```

### 1.5 Create Basic Rust Structure

```rust
// rust/src/lib.rs
mod api;

pub use api::*;
```

```rust
// rust/src/api/mod.rs
pub mod serial;
pub mod models;

pub use serial::*;
pub use models::*;
```

```rust
// rust/src/api/models.rs
use serde::{Deserialize, Serialize};

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
```

```rust
// rust/src/api/serial.rs
use super::models::*;

/// Simple test function to verify bridge works
pub fn greet(name: String) -> String {
    format!("Hello, {}! from Rust", name)
}

/// List available serial ports (placeholder)
pub fn list_serial_ports() -> Result<Vec<PortInfo>, String> {
    Ok(vec![])
}
```

### 1.6 Create Main Entry Point

```dart
// lib/main.dart
import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:termipro_flutter/src/rust/frb_generated.dart';

Future<void> main() async {
  WidgetsFlutterBinding.ensureInitialized();

  // Initialize Rust bridge - REQUIRED
  await RustLib.init();

  runApp(
    const ProviderScope(
      child: TermiProApp(),
    ),
  );
}

class TermiProApp extends StatelessWidget {
  const TermiProApp({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'TermiPro',
      debugShowCheckedModeBanner: false,
      theme: ThemeData(
        colorScheme: ColorScheme.fromSeed(
          seedColor: const Color(0xFF0EA5E9),
          brightness: Brightness.light,
        ),
        useMaterial3: true,
      ),
      home: const TestScreen(),
    );
  }
}

class TestScreen extends StatelessWidget {
  const TestScreen({super.key});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(title: const Text('TermiPro - Bridge Test')),
      body: Center(
        child: FutureBuilder<String>(
          future: greet(name: 'Flutter'),
          builder: (context, snapshot) {
            if (snapshot.hasData) {
              return Text(
                snapshot.data!,
                style: Theme.of(context).textTheme.headlineMedium,
              );
            }
            return const CircularProgressIndicator();
          },
        ),
      ),
    );
  }
}
```

### 1.7 Generate Bridge Code

```bash
cd termipro_flutter
flutter_rust_bridge_codegen generate
```

### 1.8 Test Build

```bash
# macOS
flutter run -d macos

# Windows
flutter run -d windows

# Linux
flutter run -d linux
```

---

## Verification Checklist

- [ ] Project created successfully
- [ ] Cargo.toml configured với all dependencies
- [ ] pubspec.yaml configured
- [ ] macOS entitlements set up
- [ ] Basic Rust structure created
- [ ] Bridge code generated without errors
- [ ] `greet()` function works from Flutter
- [ ] App runs on macOS
- [ ] App runs on Windows (nếu có)
- [ ] App runs on Linux (nếu có)

---

## Expected Output

Khi chạy app, màn hình hiển thị:
```
Hello, Flutter! from Rust
```

Điều này xác nhận flutter_rust_bridge hoạt động đúng.

---

## Troubleshooting

### Error: "RustLib not found"
- Run `flutter_rust_bridge_codegen generate` again
- Check `lib/src/rust/frb_generated.dart` exists

### Error: "serialport crate not found"
- Check Cargo.toml has `serialport = "4.3"`
- Run `cargo build` in `rust/` folder

### macOS: "USB access denied"
- Check entitlements file
- For development, may need to run outside sandbox

### Windows: "Cannot find MSVC"
- Install Visual Studio Build Tools
- Or install Visual Studio with C++ workload
