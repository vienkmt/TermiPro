# Phase 2: Layout Structure

**Status**: Pending
**Priority**: High
**Dependencies**: Phase 1 complete

---

## Objective

Chuyển đổi layout structure từ Material Scaffold sang MacosWindow với native Sidebar.

---

## Tasks

### 2.1 Update serial_screen.dart - Layout Structure

**File**: `lib/screens/serial_screen.dart`

**Major Changes**:

1. Import macos_ui
2. Replace Scaffold với MacosWindow
3. Replace Row layout với MacosWindow sidebar + children

**Current Structure**:
```dart
Scaffold(
  appBar: _buildAppBar(),
  body: Row(
    children: [
      Sidebar(...),           // Custom sidebar widget
      Expanded(
        child: Column(
          children: [
            Expanded(child: _buildTerminalArea()),
            _buildSendPanel(),
          ],
        ),
      ),
    ],
  ),
)
```

**New Structure**:
```dart
MacosWindow(
  titleBar: TitleBar(
    title: Text('TermiMax'),
    backgroundColor: MacosColors.transparent,
  ),
  sidebar: Sidebar(
    minWidth: 280,
    maxWidth: 320,
    builder: (context, scrollController) => _buildSidebarContent(scrollController),
  ),
  children: [
    MacosScaffold(
      children: [
        ContentArea(
          builder: (context, scrollController) => Column(
            children: [
              Expanded(child: _buildTerminalArea()),
              _buildSendPanel(),
            ],
          ),
        ),
      ],
    ),
  ],
)
```

---

### 2.2 Updated serial_screen.dart Code

```dart
import 'dart:async';
import 'package:flutter/material.dart';
import 'package:macos_ui/macos_ui.dart';
import 'package:termimax_flutter/theme/theme.dart';
import 'package:termimax_flutter/models/serial_config.dart';
import 'package:termimax_flutter/widgets/sidebar/sidebar_content.dart';
import 'package:termimax_flutter/widgets/chart/realtime_chart.dart';
import 'package:termimax_flutter/src/rust/api/models.dart';
import 'package:termimax_flutter/src/rust/api/serial.dart';

/// Main serial monitor screen with sidebar and terminal
class SerialScreen extends StatefulWidget {
  const SerialScreen({super.key});

  @override
  State<SerialScreen> createState() => _SerialScreenState();
}

class _SerialScreenState extends State<SerialScreen> {
  // State - UNCHANGED
  List<PortInfo> _ports = [];
  String? _selectedPort;
  bool _isConnected = false;
  bool _isLoadingPorts = false;
  SerialConfigModel _config = const SerialConfigModel();
  DisplayOptions _displayOptions = const DisplayOptions();
  AutoSendSettings _autoSendSettings = const AutoSendSettings();

  // Terminal data - UNCHANGED
  final List<TerminalEntry> _terminalEntries = [];
  int _txCount = 0;
  int _rxCount = 0;
  final List<int> _rxBuffer = [];
  final List<double> _chartData = [];
  static const int _maxChartPoints = 500;
  final List<List<int>> _pendingRxData = [];
  bool _updateScheduled = false;

  // Stream subscription - UNCHANGED
  StreamSubscription<SerialData>? _dataSubscription;

  // Controllers - UNCHANGED
  final TextEditingController _sendController = TextEditingController();
  final ScrollController _terminalScrollController = ScrollController();

  @override
  void initState() {
    super.initState();
    _refreshPorts();
  }

  @override
  void dispose() {
    _dataSubscription?.cancel();
    _sendController.dispose();
    _terminalScrollController.dispose();
    super.dispose();
  }

  // ==================== BUSINESS LOGIC (UNCHANGED) ====================
  // _refreshPorts, _toggleConnection, _connect, _startDataStream,
  // _handleReceivedData, _flushPendingData, _getDelimiterBytes,
  // _findDelimiter, _autoScroll, _handleDisconnect, _disconnect,
  // _sendData, _parseHexString, _clearTerminal, _showError
  // ... (keep all existing methods unchanged)

  Future<void> _refreshPorts() async { /* ... unchanged ... */ }
  Future<void> _toggleConnection() async { /* ... unchanged ... */ }
  Future<void> _connect() async { /* ... unchanged ... */ }
  void _startDataStream() { /* ... unchanged ... */ }
  void _handleReceivedData(List<int> data) { /* ... unchanged ... */ }
  void _flushPendingData() { /* ... unchanged ... */ }
  List<int> _getDelimiterBytes() { /* ... unchanged ... */ }
  int _findDelimiter(List<int> buffer, List<int> delimiter, int start) { /* ... unchanged ... */ }
  void _autoScroll() { /* ... unchanged ... */ }
  void _handleDisconnect() { /* ... unchanged ... */ }
  Future<void> _disconnect() async { /* ... unchanged ... */ }
  Future<void> _sendData() async { /* ... unchanged ... */ }
  List<int> _parseHexString(String hex) { /* ... unchanged ... */ }
  void _clearTerminal() { /* ... unchanged ... */ }
  void _showError(String message) { /* ... unchanged ... */ }

  // ==================== UI BUILD METHODS (UPDATED) ====================

  @override
  Widget build(BuildContext context) {
    return MacosWindow(
      titleBar: _buildTitleBar(),
      sidebar: Sidebar(
        minWidth: 280,
        maxWidth: 320,
        bottom: _buildConnectionStatus(),
        builder: (context, scrollController) => _buildSidebarContent(scrollController),
      ),
      child: MacosScaffold(
        children: [
          ContentArea(
            builder: (context, scrollController) => Column(
              children: [
                Expanded(child: _buildTerminalArea()),
                _buildSendPanel(),
              ],
            ),
          ),
        ],
      ),
    );
  }

  TitleBar _buildTitleBar() {
    return TitleBar(
      title: Row(
        mainAxisSize: MainAxisSize.min,
        children: [
          const MacosIcon(
            CupertinoIcons.terminal,
            size: 18,
            color: MacosColors.systemBlueColor,
          ),
          const SizedBox(width: 8),
          Text(
            'TermiMax',
            style: MacosTheme.of(context).typography.headline,
          ),
        ],
      ),
      backgroundColor: MacosColors.transparent,
    );
  }

  Widget _buildConnectionStatus() {
    return Padding(
      padding: const EdgeInsets.all(16),
      child: Container(
        padding: const EdgeInsets.symmetric(horizontal: 12, vertical: 8),
        decoration: BoxDecoration(
          color: _isConnected
              ? AppColors.successLight
              : AppColors.surfaceVariant,
          borderRadius: BorderRadius.circular(8),
        ),
        child: Row(
          mainAxisSize: MainAxisSize.min,
          children: [
            Container(
              width: 8,
              height: 8,
              decoration: BoxDecoration(
                color: _isConnected ? AppColors.success : AppColors.textTertiary,
                shape: BoxShape.circle,
              ),
            ),
            const SizedBox(width: 8),
            Text(
              _isConnected ? 'Connected' : 'Disconnected',
              style: AppTypography.labelSmall.copyWith(
                color: _isConnected ? AppColors.successDark : AppColors.textSecondary,
              ),
            ),
          ],
        ),
      ),
    );
  }

  Widget _buildSidebarContent(ScrollController scrollController) {
    return SingleChildScrollView(
      controller: scrollController,
      padding: const EdgeInsets.all(16),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          // Port Selection
          PortSelector(
            ports: _ports,
            selectedPort: _selectedPort,
            isConnected: _isConnected,
            isLoading: _isLoadingPorts,
            onRefresh: _refreshPorts,
            onPortChanged: (port) => setState(() => _selectedPort = port),
            onConnect: _toggleConnection,
          ),
          const SizedBox(height: 16),
          // Configuration
          ConfigCard(
            config: _config,
            enabled: !_isConnected,
            onChanged: (config) => setState(() => _config = config),
          ),
          const SizedBox(height: 16),
          // Signal Lines
          SignalToggles(
            dtr: _config.dtr,
            rts: _config.rts,
            enabled: !_isConnected,
            onDtrChanged: (value) {
              setState(() => _config = _config.copyWith(dtr: value));
            },
            onRtsChanged: (value) {
              setState(() => _config = _config.copyWith(rts: value));
            },
          ),
          const SizedBox(height: 16),
          // Display Options
          DisplayOptionsCard(
            options: _displayOptions,
            onChanged: (options) => setState(() => _displayOptions = options),
          ),
          const SizedBox(height: 16),
          // Auto Send
          AutoSendCard(
            settings: _autoSendSettings,
            isConnected: _isConnected,
            onChanged: (settings) => setState(() => _autoSendSettings = settings),
          ),
        ],
      ),
    );
  }

  Widget _buildTerminalArea() {
    return Container(
      margin: const EdgeInsets.all(16),
      decoration: BoxDecoration(
        color: AppColors.terminalBackground,
        borderRadius: BorderRadius.circular(10),
        boxShadow: AppColors.subtleShadow,
      ),
      child: Column(
        children: [
          _buildTerminalHeader(),
          const Divider(height: 1, color: AppColors.divider),
          Expanded(
            child: _displayOptions.chartMode
                ? RealtimeChart(
                    key: ValueKey(_rxCount),
                    data: _chartData,
                    maxPoints: _maxChartPoints,
                    lineColor: AppColors.primary,
                  )
                : _terminalEntries.isEmpty
                    ? _buildEmptyTerminal()
                    : ListView.builder(
                        controller: _terminalScrollController,
                        padding: const EdgeInsets.all(12),
                        itemCount: _terminalEntries.length,
                        itemBuilder: (context, index) {
                          return _buildTerminalLine(_terminalEntries[index]);
                        },
                      ),
          ),
        ],
      ),
    );
  }

  Widget _buildTerminalHeader() {
    return Padding(
      padding: const EdgeInsets.symmetric(horizontal: 16, vertical: 12),
      child: Row(
        children: [
          const MacosIcon(
            CupertinoIcons.terminal,
            size: 16,
            color: AppColors.primary,
          ),
          const SizedBox(width: 8),
          Text('Terminal', style: AppTypography.labelLarge),
          const Spacer(),
          // TX count
          Container(
            padding: const EdgeInsets.symmetric(horizontal: 8, vertical: 4),
            decoration: BoxDecoration(
              color: AppColors.txBadge,
              borderRadius: BorderRadius.circular(4),
            ),
            child: Text(
              'TX: $_txCount',
              style: AppTypography.badge.copyWith(color: AppColors.txText),
            ),
          ),
          const SizedBox(width: 8),
          // RX count
          Container(
            padding: const EdgeInsets.symmetric(horizontal: 8, vertical: 4),
            decoration: BoxDecoration(
              color: AppColors.rxBadge,
              borderRadius: BorderRadius.circular(4),
            ),
            child: Text(
              'RX: $_rxCount',
              style: AppTypography.badge.copyWith(color: AppColors.rxText),
            ),
          ),
          const SizedBox(width: 12),
          // Clear button
          MacosIconButton(
            icon: const MacosIcon(
              CupertinoIcons.trash,
              size: 16,
              color: AppColors.textSecondary,
            ),
            onPressed: _clearTerminal,
          ),
        ],
      ),
    );
  }

  Widget _buildEmptyTerminal() {
    return Center(
      child: Column(
        mainAxisSize: MainAxisSize.min,
        children: [
          MacosIcon(
            CupertinoIcons.terminal,
            size: 48,
            color: AppColors.textTertiary,
          ),
          const SizedBox(height: 16),
          Text(
            'No data yet',
            style: AppTypography.bodyMedium.copyWith(
              color: AppColors.textSecondary,
            ),
          ),
          const SizedBox(height: 4),
          Text(
            _isConnected
                ? 'Waiting for data...'
                : 'Connect to a serial port to start',
            style: AppTypography.bodySmall.copyWith(
              color: AppColors.textTertiary,
            ),
          ),
        ],
      ),
    );
  }

  Widget _buildTerminalLine(TerminalEntry entry) {
    final isTx = entry.type == EntryType.tx;
    final dataText = _displayOptions.hexMode
        ? entry.data.map((b) => b.toRadixString(16).padLeft(2, '0').toUpperCase()).join(' ')
        : String.fromCharCodes(entry.data).replaceAll('\r', '').replaceAll('\n', '');

    return Container(
      margin: const EdgeInsets.only(bottom: 4),
      padding: const EdgeInsets.all(8),
      decoration: BoxDecoration(
        border: Border(
          left: BorderSide(
            width: 3,
            color: isTx ? AppColors.warning : AppColors.primary,
          ),
        ),
        color: AppColors.surface,
        borderRadius: const BorderRadius.only(
          topRight: Radius.circular(4),
          bottomRight: Radius.circular(4),
        ),
      ),
      child: Row(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Text(
            '[${_formatTime(entry.timestamp)}]',
            style: AppTypography.timestamp,
          ),
          const SizedBox(width: 8),
          Container(
            padding: const EdgeInsets.symmetric(horizontal: 6, vertical: 2),
            decoration: BoxDecoration(
              color: isTx ? AppColors.txBadge : AppColors.rxBadge,
              borderRadius: BorderRadius.circular(4),
            ),
            child: Text(
              isTx ? 'TX' : 'RX',
              style: AppTypography.badge.copyWith(
                color: isTx ? AppColors.txText : AppColors.rxText,
              ),
            ),
          ),
          const SizedBox(width: 8),
          Expanded(
            child: Text(
              dataText,
              style: AppTypography.terminal,
            ),
          ),
        ],
      ),
    );
  }

  String _formatTime(DateTime time) {
    return '${time.hour.toString().padLeft(2, '0')}:'
           '${time.minute.toString().padLeft(2, '0')}:'
           '${time.second.toString().padLeft(2, '0')}';
  }

  Widget _buildSendPanel() {
    return Container(
      padding: const EdgeInsets.all(16),
      decoration: BoxDecoration(
        color: AppColors.surface,
        border: Border(
          top: BorderSide(color: AppColors.divider),
        ),
      ),
      child: Row(
        children: [
          // Mode toggle
          MacosSegmentedControl(
            controller: MacosTabController(
              initialIndex: _displayOptions.displayMode == 'Text' ? 0 : 1,
              length: 2,
            ),
            tabs: const [
              MacosTab(label: 'Text'),
              MacosTab(label: 'Hex'),
            ],
            onChanged: (index) {
              setState(() {
                _displayOptions = _displayOptions.copyWith(
                  displayMode: index == 0 ? 'Text' : 'Hex',
                );
              });
            },
          ),
          const SizedBox(width: 12),
          // Input field
          Expanded(
            child: MacosTextField(
              controller: _sendController,
              enabled: _isConnected,
              placeholder: _displayOptions.hexMode
                  ? 'Enter hex (e.g., 48 65 6C 6C 6F)'
                  : 'Enter message...',
              style: AppTypography.terminal,
              clearButtonMode: OverlayVisibilityMode.editing,
              onSubmitted: (_) => _sendData(),
            ),
          ),
          const SizedBox(width: 12),
          // Send button
          PushButton(
            controlSize: ControlSize.regular,
            onPressed: _isConnected && _sendController.text.isNotEmpty
                ? _sendData
                : null,
            child: Row(
              mainAxisSize: MainAxisSize.min,
              children: [
                const MacosIcon(
                  CupertinoIcons.paperplane_fill,
                  size: 14,
                ),
                const SizedBox(width: 6),
                const Text('Send'),
              ],
            ),
          ),
        ],
      ),
    );
  }
}

// Terminal entry model (unchanged)
class TerminalEntry {
  final EntryType type;
  final List<int> data;
  final DateTime timestamp;

  TerminalEntry({
    required this.type,
    required this.data,
    required this.timestamp,
  });
}

enum EntryType { tx, rx }
```

---

### 2.3 Update Sidebar Widget

**Note**: Rename `sidebar.dart` → `sidebar_content.dart` hoặc remove vì content giờ build trong `serial_screen.dart`

**Option A**: Remove `lib/widgets/sidebar/sidebar.dart` (sidebar content moved to serial_screen)

**Option B**: Keep as SidebarContent widget (recommended for cleaner code)

---

## Verification Steps

1. **Build Check**:
   ```bash
   flutter build macos
   ```

2. **Visual Verification**:
   - MacosWindow với transparent titlebar
   - Native Sidebar hiển thị đúng
   - Content area layout đúng
   - Connection status ở bottom sidebar

3. **Functionality Check**:
   - Sidebar scrollable
   - Port selection dropdown works
   - Connect/Disconnect works
   - Terminal display works

---

## Next Phase

Proceed to **Phase 3: Sidebar Components** để update individual sidebar widgets với macos_ui components.
