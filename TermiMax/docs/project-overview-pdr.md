# TermiMax Flutter - Project Overview & Product Development Requirements

## Executive Summary

TermiMax Flutter is a professional-grade, cross-platform serial monitor application built with Flutter for the UI layer and Rust for high-performance serial I/O operations. It delivers a modern, responsive terminal interface for embedded systems developers and hardware engineers.

**Project Status:** v1.0.0 - Active Development
**Target Platforms:** macOS (primary), Windows, Linux, iOS, Android
**Technology Stack:** Flutter + Rust FFI + Material Design 3

## Product Vision

To provide embedded systems developers with a professional, intuitive, and performant serial communication tool that:
1. Delivers real-time data visualization and analysis
2. Supports advanced serial port configuration
3. Maintains responsive UI under high-frequency data streams
4. Offers multi-platform consistency
5. Exceeds traditional terminal monitor capabilities in usability

## Key Features Implemented

### 1. Serial Port Management

- **Port Discovery**: Platform-aware USB serial port detection
  - Windows: COM ports
  - macOS: /dev/tty.* devices
  - Linux: /dev/ttyUSB*, /dev/ttyACM*, /dev/ttyS*
  - Returns manufacturer and product information

- **Connection Management**:
  - Open/close ports with full error handling
  - Real-time connection status display
  - Graceful shutdown with 200ms thread synchronization
  - DTR/RTS control support

- **Port Configuration**:
  - Baud rates: 300 to 921,600 bps
  - Data bits: 5, 6, 7, 8
  - Stop bits: 1, 1.5 (mapped to 2), 2
  - Parity: None, Odd, Even
  - Full validation before connection

### 2. Data Transmission

- **Multiple Send Modes**:
  - Text mode: Direct string transmission
  - Hex mode: Parse hex strings (e.g., "48 65 6C 6C 6F" → "Hello")
  - Line ending options: None, CR (\r), LF (\n), CRLF (\r\n)

- **Performance Features**:
  - Configurable inter-byte delay (microseconds)
  - Send counter for tracking transmissions
  - Input retention (not cleared after send)
  - Quick clear button for input field

- **Auto-Send**:
  - Configurable interval: 50ms to 60,000ms
  - Per-byte delay support for device compatibility
  - Send counter tracking
  - Easy start/stop toggle

### 3. Data Reception & Display

- **Terminal Display**:
  - Real-time data rendering with TX/RX badges
  - Yellow badges for transmitted data
  - Green badges for received data
  - Separate TX/RX statistics counters
  - Configurable timestamp display

- **Multiple Display Modes**:
  - **Text Mode**: Human-readable character display
  - **Hex Mode**: Hexadecimal representation for binary data analysis
  - **Chart Mode**: Real-time data visualization (numeric value plotting)

- **Advanced Features**:
  - Line-based parsing (CR, LF, CRLF support)
  - Auto-scroll to latest message
  - Manual scroll control
  - Terminal clearing capability
  - Up to 500 chart data points

### 4. High-Performance Data Handling

- **Batching System**:
  - Accumulates incoming data during frame cycle
  - Single UI update per frame via `addPostFrameCallback`
  - Prevents UI jank from high-frequency data streams
  - Optimal for 100+ Hz serial data rates

- **Async Architecture**:
  - Rust: Multi-threaded tokio runtime
  - Flutter: Stream-based data subscription
  - Thread-safe communication via `StreamSink`

### 5. User Interface

- **Modern Design System**:
  - Material Design 3 implementation
  - Plus Jakarta Sans font (primary)
  - JetBrains Mono font (terminal output)
  - Consistent color palette with accessibility focus

- **Layout**:
  - Sidebar (320px): Port selection, configuration, display options
  - Main area: Terminal display with statistics
  - Footer: Input area with Send/Auto buttons
  - Responsive design for various window sizes

- **Theme**:
  - Light theme with sky blue accent
  - Proper contrast ratios for accessibility
  - Consistent spacing and component sizing

### 6. Platform Support

- **macOS**: Primary platform with full feature set
- **Windows**: Full support via Windows SDK
- **Linux**: Planned (Flutter + Rust support ready)
- **iOS**: Planned (Flutter support ready)
- **Android**: Planned (Flutter support ready)

## Planned Features (Roadmap)

### Near-term (v1.1.0)
- [ ] Session history and save/load functionality
- [ ] Data filtering and search in terminal
- [ ] Custom command templates
- [ ] Snapshot/screenshot export
- [ ] Settings persistence across sessions

### Mid-term (v1.2.0)
- [ ] Dark mode theme
- [ ] Multi-port simultaneous monitoring
- [ ] Data logging to file with rotation
- [ ] Pattern matching and notifications
- [ ] Custom color themes

### Long-term (v2.0.0)
- [ ] Protocol analyzers (Modbus, SLIP, etc.)
- [ ] Script recording and playback
- [ ] Graphical bitrate analyzer
- [ ] Network serial support (telnet)
- [ ] Plugin architecture for custom analyzers
- [ ] iOS and Android native support

## Technical Requirements

### Non-Functional Requirements

1. **Performance**:
   - Support data rates up to 921,600 bps
   - Maintain 60 FPS UI responsiveness with 100+ Hz data streams
   - Memory footprint < 150MB
   - Cold start time < 2 seconds

2. **Reliability**:
   - Handle port disconnection gracefully
   - Recover from serial errors without app crash
   - Preserve UI state on reconnection
   - Proper error messages for user guidance

3. **Compatibility**:
   - Work with all standard USB serial devices (FTDI, Prolific, CP210x, etc.)
   - Support various line ending conventions
   - Handle binary data without corruption
   - Platform-consistent behavior across macOS/Windows/Linux

4. **Usability**:
   - Intuitive port selection and configuration
   - Clear connection status indication
   - Meaningful error messages
   - Responsive UI with visual feedback

## Acceptance Criteria

### For Release v1.0.0

- [ ] All listed features implemented and tested
- [ ] Handles serial data rates from 300 to 921,600 bps
- [ ] Maintains UI responsiveness with high-frequency data
- [ ] No crashes on port errors or disconnection
- [ ] Clear error messages for common issues
- [ ] Documentation complete and accurate
- [ ] Works on macOS and Windows
- [ ] Builds without warnings in release mode

### For Each Feature

1. **Feature Implementation**: Code complete with no compiler warnings
2. **Testing**: Integration tests pass, manual testing complete
3. **Documentation**: API documented, user-facing features explained
4. **Error Handling**: All error paths handled gracefully
5. **Performance**: Meets non-functional requirements

## Architecture Decisions

### 1. FFI-Based Rust Backend

**Decision**: Use flutter_rust_bridge for serial I/O
**Rationale**:
- Native Rust performance for serial operations
- Thread safety via parking_lot and once_cell
- Cross-platform support without platform-specific code in Dart
- Future-proof for adding network serial support

### 2. Single-Screen Stateful Architecture

**Decision**: Main UI in single StatefulWidget
**Rationale**:
- Simplified state management (no Provider/Riverpod)
- Easier to reason about data flow
- Lower complexity for initial release
- Can refactor to multi-screen if needed

### 3. Callback Pattern State Management

**Decision**: Use callbacks instead of state management libraries
**Rationale**:
- Lightweight and transparent
- Easier debugging and performance profiling
- Explicit data flow without hidden subscriptions
- Sufficient for single-screen application

### 4. Data Batching System

**Decision**: Accumulate data and batch UI updates per frame
**Rationale**:
- Prevents UI jank with high-frequency serial data
- Smooth scrolling and display
- Optimal frame timing
- Reduces render overhead

## Dependencies & Tooling

### Flutter/Dart
- flutter_rust_bridge 2.11.1 (FFI code generation)
- google_fonts 7.0.1 (font management)
- Material Design 3 integration

### Rust
- serialport 4.3 (serial port I/O)
- tokio 1 (async runtime)
- parking_lot 0.12 (synchronization primitives)
- once_cell 1.19 (global state container)
- serde/serde_json (serialization)

## Testing Strategy

### Unit Tests
- Rust serial API functions
- Data parsing (hex, text, line endings)
- Configuration validation

### Integration Tests
- Full flow: port listing → connection → data transmission
- Display mode switching
- Error recovery

### Manual Testing
- Real devices with various serial configurations
- High-frequency data stress test
- Port connection/disconnection scenarios

## Success Metrics

| Metric | Target | Current |
|--------|--------|---------|
| Data rate support | 921,600 bps | Implemented |
| UI responsiveness | 60 FPS @ 100Hz data | Achieved via batching |
| Memory usage | < 150MB | < 100MB observed |
| Platform support | macOS, Windows, Linux | macOS, Windows complete |
| Feature completeness | 100% (v1.0) | 100% implemented |

## Constraints & Dependencies

### Technical Constraints
1. Flutter SDK 3.10.7+ required
2. Rust 1.70+ with cargo required
3. Platform-specific build tools (Xcode, MSVC, build-essential)
4. 5ms serial port read timeout (system-level constraint)

### Dependencies
1. serialport crate for low-level serial I/O
2. tokio for async operations
3. Flutter framework for UI
4. flutter_rust_bridge for FFI code generation

### Known Limitations
1. Serial port 1.5 stop bits mapped to 2 (serialport crate limitation)
2. No network serial support in v1.0 (planned for v2.0)
3. Single port monitoring (multi-port planned)
4. No native iOS/Android in v1.0 (planned)

## Version History

| Version | Date | Status | Changes |
|---------|------|--------|---------|
| 1.0.0 | Jan 2025 | Active | Initial release with core features |
| 1.0.0 | Jan 2025 | In Dev | Documentation, bug fixes |

## References

- [Flutter Documentation](https://flutter.dev)
- [flutter_rust_bridge Guide](https://cjycode.com/flutter_rust_bridge/)
- [Rust Serial Port Crate](https://docs.rs/serialport/)
- [Material Design 3 Specification](https://m3.material.io/)

---

**Document Version**: 1.0
**Last Updated**: January 13, 2025
**Authored by**: Documentation Team
