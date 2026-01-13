# Phase 4-7: Summary Implementation Guide

## Phase 4: Sidebar UI Components

### Port Selector (`lib/widgets/sidebar/port_selector.dart`)

**Key Features**:
- Extended CustomDropdown cho port list
- Hiển thị `port_type` + `product/manufacturer`
- "In use" badge với màu warning
- Auto refresh khi click dropdown

**Structure**:
```dart
class PortSelector extends ConsumerWidget {
  // Uses portsProvider to get list
  // Uses selectedPortProvider for current selection
  // Triggers refresh via ref.refresh(portsProvider)
}
```

### Config Card (`lib/widgets/sidebar/config_card.dart`)

**Dropdowns**:
| Config | Options | Default |
|--------|---------|---------|
| Baud Rate | 9600, 19200, 57600, 115200, 460800, 921600 | 115200 |
| Data Bits | 5, 6, 7, 8 | 8 |
| Stop Bits | 1, 1.5, 2 | 1 |
| Parity | None, Odd, Even | None |

### Signal Toggles (`lib/widgets/sidebar/signal_toggles.dart`)

```dart
Row(
  children: [
    ToggleSwitch(value: dtr, label: 'DTR'),
    SizedBox(width: 16),
    ToggleSwitch(value: rts, label: 'RTS'),
    Spacer(),
    HelpButton(onPressed: showSignalHelpModal),
  ],
)
```

### Display Options (`lib/widgets/sidebar/display_options.dart`)

**Components**:
- Text/Hex toggle buttons (segmented control)
- Auto Scroll switch
- Line Ending selector (None, CR, LF, CRLF)

### Auto Send Card (`lib/widgets/sidebar/auto_send_card.dart`)

**Fields**:
- Interval input (50-60000 ms)
- Byte Delay input (0-10000 µs)
- Send count display với "ON" badge khi active

---

## Phase 5: Terminal Display & Send Panel

### Terminal Header (`lib/widgets/terminal/terminal_header.dart`)

```dart
Row(
  children: [
    Icon(Icons.terminal),
    Text('Terminal'),
    Spacer(),
    StatBadge(type: 'TX', count: txCount, color: txColor),
    StatBadge(type: 'RX', count: rxCount, color: rxColor),
    ClearButton(onPressed: clearTerminal),
  ],
)
```

### Terminal Display (`lib/widgets/terminal/terminal_display.dart`)

```dart
class TerminalDisplay extends ConsumerWidget {
  static const maxEntries = 500;

  @override
  Widget build(context, ref) {
    final entries = ref.watch(terminalDataProvider);
    final autoScroll = ref.watch(autoScrollProvider);
    final displayMode = ref.watch(displayModeProvider);

    return ListView.builder(
      controller: _scrollController,
      itemCount: entries.length,
      itemBuilder: (_, index) => TerminalLine(
        entry: entries[index],
        displayMode: displayMode,
      ),
    );
  }
}
```

### Terminal Line (`lib/widgets/terminal/terminal_line.dart`)

```dart
Container(
  decoration: BoxDecoration(
    color: AppColors.bgSecondary,
    borderRadius: BorderRadius.circular(2),
    border: Border(
      left: BorderSide(
        color: entry.type == 'tx' ? AppColors.txColor : AppColors.rxColor,
        width: 2,
      ),
    ),
  ),
  child: Row(
    children: [
      Text('[${entry.timestamp}]', style: timestampStyle),
      DirectionBadge(type: entry.type), // TX or RX
      Expanded(
        child: Text(
          displayMode == 'hex'
              ? formatHex(entry.data)
              : formatText(entry.data),
          style: monoStyle,
        ),
      ),
    ],
  ),
)
```

### Send Container (`lib/widgets/send_panel/send_container.dart`)

```dart
Row(
  children: [
    ToggleSwitch(value: sendAsHex, label: 'Hex', compact: true),
    Expanded(
      child: TextField(
        controller: inputController,
        decoration: InputDecoration(
          hintText: sendAsHex ? 'VD: 48 65 6C 6C 6F' : 'Nhập tin nhắn...',
          suffixIcon: ClearInputButton(),
        ),
        onSubmitted: (_) => sendMessage(),
      ),
    ),
    SendButton(onPressed: sendMessage),
    AutoSendButton(
      isRunning: autoSendEnabled,
      onStart: startAutoSend,
      onStop: stopAutoSend,
    ),
  ],
)
```

---

## Phase 6: State Management & Integration

### Providers Structure

```dart
// lib/providers/serial_provider.dart

// Config providers
final selectedPortProvider = StateProvider<String?>((ref) => null);
final baudRateProvider = StateProvider<int>((ref) => 115200);
final dataBitsProvider = StateProvider<int>((ref) => 8);
final stopBitsProvider = StateProvider<String>((ref) => '1');
final parityProvider = StateProvider<String>((ref) => 'none');
final dtrProvider = StateProvider<bool>((ref) => false);
final rtsProvider = StateProvider<bool>((ref) => false);

// Connection state
enum ConnectionState { disconnected, connecting, connected, error }
final connectionStateProvider = StateProvider<ConnectionState>(
  (ref) => ConnectionState.disconnected,
);

// Available ports
final portsProvider = FutureProvider<List<PortInfo>>((ref) async {
  return await listSerialPorts();
});

// Serial data stream
final serialStreamProvider = StreamProvider.autoDispose<SerialData>((ref) {
  final port = ref.watch(selectedPortProvider);
  final baud = ref.watch(baudRateProvider);

  if (port == null) {
    throw Exception('No port selected');
  }

  return streamSerialData(portName: port, baudRate: baud);
});
```

```dart
// lib/providers/terminal_provider.dart

class TerminalEntry {
  final String type; // 'tx' or 'rx'
  final List<int> data;
  final String timestamp;

  TerminalEntry({required this.type, required this.data, required this.timestamp});
}

class TerminalNotifier extends StateNotifier<List<TerminalEntry>> {
  static const maxEntries = 500;

  TerminalNotifier() : super([]);

  void addTx(List<int> data) {
    _addEntry('tx', data);
  }

  void addRx(List<int> data) {
    _addEntry('rx', data);
  }

  void _addEntry(String type, List<int> data) {
    final entry = TerminalEntry(
      type: type,
      data: data,
      timestamp: _formatTime(DateTime.now()),
    );

    state = [...state, entry];

    // Trim old entries
    if (state.length > maxEntries) {
      state = state.sublist(state.length - maxEntries);
    }
  }

  void clear() {
    state = [];
  }
}

final terminalProvider = StateNotifierProvider<TerminalNotifier, List<TerminalEntry>>(
  (ref) => TerminalNotifier(),
);

// Statistics
final txCountProvider = Provider<int>((ref) {
  return ref.watch(terminalProvider).where((e) => e.type == 'tx').length;
});

final rxCountProvider = Provider<int>((ref) {
  return ref.watch(terminalProvider).where((e) => e.type == 'rx').length;
});
```

```dart
// lib/providers/auto_send_provider.dart

class AutoSendState {
  final bool enabled;
  final int interval;
  final int byteDelay;
  final int count;
  final String? message;

  AutoSendState({
    this.enabled = false,
    this.interval = 1000,
    this.byteDelay = 0,
    this.count = 0,
    this.message,
  });
}

class AutoSendNotifier extends StateNotifier<AutoSendState> {
  Timer? _timer;

  AutoSendNotifier() : super(AutoSendState());

  void start(String message, int interval, int byteDelay) {
    _timer?.cancel();
    state = AutoSendState(
      enabled: true,
      interval: interval,
      byteDelay: byteDelay,
      count: 0,
      message: message,
    );

    _timer = Timer.periodic(Duration(milliseconds: interval), (_) {
      // Send message via Rust
      // Increment count
      state = state.copyWith(count: state.count + 1);
    });
  }

  void stop() {
    _timer?.cancel();
    state = state.copyWith(enabled: false);
  }
}
```

### Serial Service

```dart
// lib/services/serial_service.dart

class SerialService {
  Future<void> connect(SerialConfig config) async {
    await openPort(config: config);
  }

  Future<void> disconnect(String portName) async {
    await closePort(portName: portName);
  }

  Future<void> send(String portName, String data, bool isHex, int? byteDelay) async {
    await sendData(
      portName: portName,
      data: data,
      isHex: isHex,
      byteDelayUs: byteDelay,
    );
  }

  Stream<SerialData> stream(String portName, int baudRate) {
    return streamSerialData(portName: portName, baudRate: baudRate);
  }
}

final serialServiceProvider = Provider((ref) => SerialService());
```

---

## Phase 7: Polish & Platform Testing

### Signal Help Modal

```dart
void showSignalHelpModal(BuildContext context) {
  showDialog(
    context: context,
    builder: (context) => Dialog(
      child: Container(
        width: 460,
        padding: EdgeInsets.all(16),
        child: Column(
          mainAxisSize: MainAxisSize.min,
          children: [
            // Header
            Row(
              children: [
                Text('Tín hiệu điều khiển', style: headerStyle),
                Spacer(),
                IconButton(icon: Icon(Icons.close), onPressed: Navigator.pop),
              ],
            ),
            Divider(),
            // DTR Section
            SignalInfoCard(
              name: 'DTR',
              fullName: 'Data Terminal Ready',
              meaning: 'Máy tính sẵn sàng giao tiếp',
              description: 'Báo hiệu thiết bị đầu cuối sẵn sàng...',
              cases: [
                ('Arduino reset khi connect', true),
                ('Không muốn reset board', false),
                ('ESP32 download mode', true),
              ],
            ),
            // RTS Section
            SignalInfoCard(
              name: 'RTS',
              fullName: 'Request To Send',
              meaning: 'Máy tính muốn gửi dữ liệu',
              description: 'Điều khiển hướng truyền...',
              cases: [...],
            ),
            // Summary
            Container(
              color: AppColors.successLight,
              padding: EdgeInsets.all(10),
              child: Row(
                children: [
                  Icon(Icons.check_circle, color: AppColors.success),
                  Text('Khuyến cáo: Hầu hết trường hợp để mặc định'),
                ],
              ),
            ),
          ],
        ),
      ),
    ),
  );
}
```

### Keyboard Shortcuts

```dart
// In SerialScreen
KeyboardListener(
  autofocus: true,
  onKeyEvent: (event) {
    if (event is KeyDownEvent) {
      if (event.logicalKey == LogicalKeyboardKey.enter) {
        sendMessage();
      }
    }
  },
  child: ...,
)
```

### Platform Testing Checklist

**macOS**:
- [ ] USB entitlements configured
- [ ] Code signing for distribution
- [ ] Test với USB-to-Serial adapter
- [ ] Test với Arduino/ESP32

**Windows**:
- [ ] COM port access works
- [ ] MSVC runtime included
- [ ] Test COM1, COM3, etc.

**Linux**:
- [ ] Document udev rules
- [ ] Test /dev/ttyUSB0
- [ ] GTK theming works

---

## Main Screen Layout

```dart
// lib/screens/serial_screen.dart

class SerialScreen extends ConsumerWidget {
  @override
  Widget build(BuildContext context, WidgetRef ref) {
    return Scaffold(
      body: Row(
        children: [
          // Sidebar (320px)
          Container(
            width: 320,
            color: AppColors.bgSecondary,
            padding: EdgeInsets.all(14),
            child: Column(
              children: [
                ConfigCard(
                  title: 'Serial Port',
                  icon: Icons.settings_input_component,
                  child: PortSelector(),
                ),
                SizedBox(height: 10),
                ConfigCard(
                  title: 'Configuration',
                  icon: Icons.tune,
                  child: Column(
                    children: [
                      BaudRateDropdown(),
                      DataBitsDropdown(),
                      StopBitsDropdown(),
                      ParityDropdown(),
                      SignalToggles(),
                    ],
                  ),
                ),
                SizedBox(height: 10),
                ConnectButton(),
                Spacer(),
                ConfigCard(
                  title: 'Display',
                  icon: Icons.monitor,
                  child: DisplayOptions(),
                ),
                SizedBox(height: 10),
                ConfigCard(
                  title: 'Auto Send',
                  icon: Icons.timer,
                  child: AutoSendCard(),
                ),
              ],
            ),
          ),
          // Divider
          VerticalDivider(width: 1),
          // Main content
          Expanded(
            child: Column(
              children: [
                TerminalHeader(),
                Expanded(child: TerminalDisplay()),
                SendContainer(),
              ],
            ),
          ),
        ],
      ),
    );
  }
}
```

---

## File List Summary

```
lib/
├── main.dart
├── app.dart
├── theme/
│   ├── app_colors.dart
│   └── app_theme.dart
├── models/
│   ├── serial_config.dart
│   ├── port_info.dart
│   └── terminal_entry.dart
├── providers/
│   ├── serial_provider.dart
│   ├── ports_provider.dart
│   ├── terminal_provider.dart
│   └── auto_send_provider.dart
├── services/
│   └── serial_service.dart
├── screens/
│   └── serial_screen.dart
└── widgets/
    ├── sidebar/
    │   ├── port_selector.dart
    │   ├── config_dropdowns.dart
    │   ├── signal_toggles.dart
    │   ├── display_options.dart
    │   └── auto_send_card.dart
    ├── terminal/
    │   ├── terminal_display.dart
    │   ├── terminal_line.dart
    │   ├── terminal_header.dart
    │   └── terminal_empty.dart
    ├── send_panel/
    │   ├── send_container.dart
    │   └── send_buttons.dart
    └── common/
        ├── custom_dropdown.dart
        ├── toggle_switch.dart
        ├── connect_button.dart
        ├── config_card.dart
        └── signal_help_modal.dart
```

---

## Implementation Order

1. **Phase 1**: Project setup, flutter_rust_bridge init
2. **Phase 2**: Rust API (critical path)
3. **Phase 3**: Theme + base widgets
4. **Phase 4**: Sidebar components
5. **Phase 5**: Terminal + send panel
6. **Phase 6**: Providers + integration
7. **Phase 7**: Polish + testing

**Recommended parallel tracks**:
- Track A: Rust API (Phase 2) → Integration (Phase 6)
- Track B: Theme (Phase 3) → UI Components (Phase 4, 5)

---

## Success Metrics

| Feature | Priority | Status |
|---------|----------|--------|
| List ports | P0 | - |
| Connect/Disconnect | P0 | - |
| Send text | P0 | - |
| Send hex | P0 | - |
| Receive stream | P0 | - |
| Terminal display | P0 | - |
| TX/RX badges | P1 | - |
| Auto send | P1 | - |
| DTR/RTS control | P1 | - |
| Line ending | P1 | - |
| Display mode toggle | P2 | - |
| Auto scroll | P2 | - |
| Statistics | P2 | - |
| Clear terminal | P2 | - |
| Signal help modal | P3 | - |
