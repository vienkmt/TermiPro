# TermiPro - Project Overview & Product Development Requirements (PDR)

## Executive Summary

**TermiPro** is a professional desktop serial communication application for embedded systems developers, IoT engineers, and hardware developers. It provides a unified interface to communicate with microcontrollers and IoT devices using multiple protocols: Serial, TCP/IP, Modbus, and MQTT.

**Status**: Production Ready
**Version**: 1.0.0
**Release Date**: 2025
**Platform**: macOS, Windows, Linux

---

## 1. Project Definition

### 1.1 Vision
Create the industry-standard serial monitor for embedded systems with modern UI, multi-protocol support, and professional reliability.

### 1.2 Mission
Empower developers to efficiently communicate with embedded systems and IoT devices through a beautiful, intuitive desktop application.

### 1.3 Target Users
- Embedded systems engineers
- IoT application developers
- Firmware developers
- Electronics hobbyists
- Industrial automation technicians

### 1.4 Market Context
- **Problem**: Existing serial monitors (putty, minicom) have outdated UIs and limited features
- **Solution**: Modern desktop app with intuitive interface and multi-protocol support
- **Opportunity**: Growing IoT/embedded market with demand for professional tools

---

## 2. Core Features

### 2.1 Serial Port Communication
**Status**: Complete

#### Functional Requirements
- [x] Auto-detect USB serial ports
- [x] Filter results to show only valid USB devices
- [x] Support full serial configuration (baud, data bits, stop bits, parity)
- [x] Real-time connection status display
- [x] Auto-detect and remove disconnected ports

#### Technical Requirements
- Baud rates: 300 to 921,600 bps
- Data bits: 5, 6, 7, 8
- Stop bits: 1, 1.5, 2
- Parity: None, Odd, Even
- Flow control: DTR/RTS support
- Cross-platform support: macOS, Windows, Linux

#### Acceptance Criteria
- Serial data transmitted and received without corruption
- Port list refreshes when devices plugged/unplugged
- Connection persists until user disconnects
- Auto-reconnect on unexpected disconnect

### 2.2 TCP Communication
**Status**: Complete

#### TCP Client
- [x] Connect to remote TCP servers
- [x] Auto-reconnect with exponential backoff
- [x] Retry logic for failed sends
- [x] Multiple simultaneous connections (one per tab)

#### TCP Server
- [x] Listen on configurable port
- [x] Accept multiple client connections
- [x] Broadcast to all clients
- [x] Send to specific client
- [x] Disconnect individual clients
- [x] Echo mode for testing

#### Technical Requirements
- Port range: 1-65535
- Max clients per server: configurable
- Connection timeout: configurable
- Retry attempts: 3 with 500ms delay
- Reconnect attempts: 3 with 1s delay

#### Acceptance Criteria
- TCP data transmitted reliably
- Server accepts multiple connections without data loss
- Automatic reconnection succeeds when server comes back online
- Echo mode correctly reflects sent data

### 2.3 Modbus Protocol Support
**Status**: Complete

#### Modbus Master
- [x] Support RTU (serial) and TCP modes
- [x] Read coils and discrete inputs
- [x] Read input and holding registers
- [x] Write single and multiple registers
- [x] Support multiple simultaneous connections

#### Modbus Slave
- [x] Simulate Modbus device
- [x] Support RTU and TCP modes
- [x] Expose configurable registers
- [x] Accept read/write requests
- [x] Event logging for testing

#### Technical Requirements
- Function codes: 01, 02, 03, 04, 05, 06, 15, 16 (basic + multi-write)
- Response timeout: configurable (default 1000ms)
- Slave ID: 1-247
- Register count: 0-65535
- RTU encoding: CRC16 checksums

#### Acceptance Criteria
- Modbus reads/writes complete without timeout
- CRC validation prevents corrupted data
- Slave responds to all supported function codes
- Multi-register operations atomic

### 2.4 MQTT Support
**Status**: Complete

#### MQTT Client
- [x] Connect to MQTT brokers (version 3.1.1)
- [x] Publish messages with configurable QoS
- [x] Subscribe to topics
- [x] Support username/password authentication
- [x] Retain message support
- [x] Multiple subscriptions per connection

#### Technical Requirements
- MQTT version: 3.1.1
- QoS levels: 0, 1, 2
- Max topic length: 65535 bytes
- Keep-alive: configurable (default 60s)
- Connection timeout: 10s

#### Acceptance Criteria
- Messages published and received reliably
- Topic subscription filters work correctly
- QoS 2 ensures exactly-once delivery
- Authentication succeeds with correct credentials

### 2.5 Terminal Display
**Status**: Complete

#### Functional Requirements
- [x] Real-time data display
- [x] Distinguish TX (sent) from RX (received) data
- [x] Display mode: Text or Hex
- [x] Statistics: Byte count for TX and RX
- [x] Auto-scroll to latest message
- [x] Clear button to reset history
- [x] Search/filter (future)

#### Technical Requirements
- Terminal limit: 500 messages per tab
- Timestamp: millisecond precision
- Character encoding: UTF-8 with fallback to hex
- Color coding: Sent (yellow), Received (green)

#### Acceptance Criteria
- No data loss in display
- Terminal remains responsive with 500+ messages
- Display modes toggle without data loss
- Auto-scroll doesn't interfere with manual scrolling

### 2.6 Data Transmission
**Status**: Complete

#### Functional Requirements
- [x] Text mode: Send raw text
- [x] Hex mode: Send hex bytes (space-separated)
- [x] Line ending options: None, LF, CRLF
- [x] Preserve input after send
- [x] Clear input button
- [x] Auto-send with configurable interval

#### Technical Requirements
- Auto-send range: 50ms - 60,000ms
- Input buffer: no limit
- Parsing: space/newline separated hex values
- Line endings: configurable per tab

#### Acceptance Criteria
- Hex values parse correctly
- Auto-send maintains specified interval
- No data corruption on text/hex conversion

### 2.7 Multi-Tab Management
**Status**: Complete

#### Functional Requirements
- [x] Create new tabs for different connections
- [x] Switch between tabs
- [x] Close tabs with confirmation
- [x] Persistent configuration per tab
- [x] Max 8 simultaneous tabs
- [x] Tab naming

#### Technical Requirements
- Tab limit: 8 (system constraint)
- State persistence: In-memory only
- Tab isolation: No cross-tab interference

#### Acceptance Criteria
- Tabs switch without data loss
- Closing one tab doesn't affect others
- Configuration saved per tab during session
- Tab limit enforced gracefully

### 2.8 User Interface
**Status**: Complete

#### Functional Requirements
- [x] Modern, intuitive design
- [x] Light theme with sky blue accent
- [x] Responsive layout (responsive at 900x600 minimum)
- [x] Keyboard shortcuts for common actions
- [x] Status indicators for connection state
- [x] i18n support (Vietnamese & English)

#### Technical Requirements
- Minimum window: 900x600
- Default window: 1200x850
- Fonts: Plus Jakarta Sans (UI), JetBrains Mono (terminal)
- Offline operation: All fonts bundled
- Dark mode: Not required

#### Acceptance Criteria
- UI loads in under 2 seconds
- All text renders correctly in both languages
- Terminal font is monospace and legible

### 2.9 Application Updates
**Status**: Complete

#### Functional Requirements
- [x] Check for updates on startup
- [x] Notify user of new versions
- [x] Download and apply updates
- [x] Automatic relaunch after update

#### Technical Requirements
- Update check: Every app start
- Signature verification: Required
- Rollback support: Automatic on failure

#### Acceptance Criteria
- Update process transparent to user
- No data loss during update
- Automatic relaunch succeeds

---

## 3. Non-Functional Requirements

### 3.1 Performance
- **UI Responsiveness**: Under 100ms for user interactions
- **Terminal Display**: Handle 500+ messages without lag
- **Memory Usage**: Under 200MB for typical use
- **CPU Usage**: Under 5% idle
- **Event Latency**: Real-time (< 10ms) for data display

### 3.2 Reliability
- **MTBF** (Mean Time Between Failures): > 7 days continuous operation
- **Data Integrity**: No data loss on normal operation
- **Graceful Degradation**: Single tab failure doesn't crash app
- **Recovery**: Auto-reconnect on network/serial failures

### 3.3 Security
- **No Remote Execution**: Commands from external sources forbidden
- **No Data Logging**: Terminal history not saved to disk by default
- **Encrypted Transport**: Support TLS/SSL for TCP/MQTT
- **Input Validation**: Sanitize all command inputs

### 3.4 Usability
- **Learning Curve**: First serial connection < 2 minutes
- **Help System**: In-app help for all features
- **Error Messages**: User-friendly, actionable
- **Defaults**: Sensible defaults for all configurations

### 3.5 Maintainability
- **Code Quality**: Well-documented, modular design
- **Test Coverage**: > 80% for critical paths
- **Build Time**: < 5 minutes development build
- **Dependency Management**: Regular updates, minimal dependencies

### 3.6 Scalability
- **Multi-Protocol**: Support 6 protocols (complete)
- **Extensibility**: Plugin architecture (future)
- **Concurrent Operations**: No limit on simultaneous connections
- **Data Rate**: Support up to 921.6kbps

---

## 4. Technical Architecture

### 4.1 Tech Stack
- **Frontend**: Vue.js 3 + Vite
- **Backend**: Rust + Tokio async runtime
- **IPC**: Tauri v2 command/event system
- **Serial**: serialport crate v4.3
- **TCP**: tokio::net
- **MQTT**: rumqttc v0.24
- **Modbus**: Custom implementation in Rust
- **Build**: Cargo + npm

### 4.2 Communication Architecture

```
┌─────────────────────────────────────────────────────────┐
│                     Frontend (Vue.js)                   │
│         Components → tabStore → Async Events            │
└────────────────────┬────────────────────────────────────┘
                     │ Tauri IPC
                     ↓
┌─────────────────────────────────────────────────────────┐
│                  Backend (Rust)                         │
│    Commands → State Managers → Async Runtimes          │
│              ↓                                           │
│    Serial/TCP/Modbus/MQTT Events → Frontend Events     │
└─────────────────────────────────────────────────────────┘
```

### 4.3 State Management

**Frontend** (tabStore.js):
- Reactive tab collection
- Terminal data per tab
- Connection state per tab
- Auto-send configuration

**Backend** (lib.rs):
- SerialState: Port handles
- TcpState: Client/server handles
- ModbusState: Connection handles
- ModbusSlaveState: Server handles
- MqttState: Client handles

### 4.4 Runtime Architecture

**Async Runtimes**:
- TCP: 4 worker threads (handles many concurrent connections)
- Modbus: 2 worker threads
- Modbus Slave: 2 worker threads
- MQTT: 2 worker threads

---

## 5. Development Roadmap

### Phase 1: Foundation (Complete)
- [x] Serial port support
- [x] TCP client/server
- [x] Basic UI
- [x] Terminal display

### Phase 2: Enterprise Features (Complete)
- [x] Modbus master/slave
- [x] MQTT support
- [x] Multi-tab management
- [x] i18n support

### Phase 3: Professional Polish (Complete)
- [x] Auto-update system
- [x] Modern UI design
- [x] Cross-platform builds
- [x] Performance optimization

### Phase 4: Future Enhancements (Planned)
- [ ] WebSocket support
- [ ] CAN bus support
- [ ] Script automation (Lua/JavaScript)
- [ ] Data logging & persistence
- [ ] Plugin system
- [ ] Dark mode
- [ ] Terminal search/filter
- [ ] Custom protocols

---

## 6. Success Metrics

### Adoption
- Target: 10,000+ downloads in first year
- Measure: GitHub releases download count

### User Satisfaction
- Target: 4.5+ star rating on distribution platforms
- Measure: User reviews and ratings

### Performance
- Target: < 100MB memory usage
- Measure: Memory profiler on typical operations

### Stability
- Target: < 0.1% crash rate
- Measure: Crash reporting analytics

### Community
- Target: 100+ GitHub stars
- Measure: GitHub stars

---

## 7. Risk Management

### Risk: Serial Port Compatibility
- **Probability**: Medium
- **Impact**: Medium
- **Mitigation**: Use well-tested serialport crate, test on multiple platforms
- **Contingency**: Fall back to native OS serial APIs

### Risk: TCP Reconnection Complexity
- **Probability**: Low
- **Impact**: High
- **Mitigation**: Extensive testing, exponential backoff logic
- **Contingency**: Manual reconnect button

### Risk: MQTT Edge Cases
- **Probability**: Medium
- **Impact**: Medium
- **Mitigation**: Follow MQTT 3.1.1 spec closely, test with major brokers
- **Contingency**: Clear documentation of limitations

### Risk: Terminal Performance
- **Probability**: Low
- **Impact**: High
- **Mitigation**: Hard-coded 500 message limit, RAF batching
- **Contingency**: Implement message compression

### Risk: Platform-Specific Bugs
- **Probability**: Medium
- **Impact**: High
- **Mitigation**: Test on macOS, Windows, Linux
- **Contingency**: Community-driven bug fixes

---

## 8. Implementation Constraints

### Must-Have
- Cross-platform support (macOS, Windows, Linux)
- Real-time data display
- Multi-tab support
- Persistent connection state during session

### Should-Have
- Auto-reconnect logic
- MQTT support
- Modbus slave
- i18n support

### Nice-to-Have
- Dark mode
- Terminal search
- Data export
- Custom protocols

---

## 9. Version History

| Version | Date | Highlights |
|---------|------|-----------|
| 1.0.0 | 2025-12 | Initial release with 6 protocols |
| 0.9.0 | 2025-11 | MQTT and Modbus Slave added |
| 0.8.0 | 2025-10 | TCP Server support |
| 0.5.0 | 2025-09 | TCP Client support |
| 0.1.0 | 2025-08 | Serial + basic UI |

---

## 10. Compliance & Standards

- **MQTT**: Compliant with MQTT 3.1.1 specification
- **Modbus**: Compliant with Modbus RTU/TCP specifications
- **Serial**: Compliant with RS-232 standards
- **Accessibility**: WCAG 2.1 Level A (partial)
- **Security**: No security standards compliance required (personal tool)

---

## 11. Support & Maintenance

### Documentation
- User guides for each protocol
- API documentation for contributors
- Troubleshooting guides

### Community Support
- GitHub Issues for bug reports
- GitHub Discussions for questions
- Contributing guidelines

### Maintenance
- Critical bug fixes: within 48 hours
- Feature requests: evaluated quarterly
- Dependency updates: monthly
- Platform updates: within 30 days of OS release

---

## 12. Conclusion

TermiPro is a professionally engineered serial communication application that meets the needs of modern embedded systems developers. With comprehensive protocol support, modern UI, and reliable operation, it sets a new standard for desktop serial monitors.

**Approval Status**: Ready for Production

**Last Updated**: 2025-12-29
**PDR Version**: 1.0
