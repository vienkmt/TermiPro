# Phase 4: Main Content Area

**Status**: Pending
**Priority**: Medium
**Dependencies**: Phase 3 complete

---

## Objective

Polish terminal display và send panel với macOS native styling.

---

## Tasks

### 4.1 Terminal Area Improvements

**Location**: `serial_screen.dart` - `_buildTerminalArea()`

**Changes**:
- Subtle shadow thay vì border
- Cleaner header với native icons
- Smoother scrolling

**Code Update**:
```dart
Widget _buildTerminalArea() {
  return Container(
    margin: const EdgeInsets.all(16),
    decoration: BoxDecoration(
      color: AppColors.terminalBackground,
      borderRadius: BorderRadius.circular(10),
      boxShadow: AppColors.subtleShadow,
    ),
    clipBehavior: Clip.antiAlias,
    child: Column(
      children: [
        _buildTerminalHeader(),
        Container(
          height: 1,
          color: AppColors.divider.withOpacity(0.5),
        ),
        Expanded(
          child: _buildTerminalContent(),
        ),
      ],
    ),
  );
}

Widget _buildTerminalContent() {
  if (_displayOptions.chartMode) {
    return RealtimeChart(
      key: ValueKey(_rxCount),
      data: _chartData,
      maxPoints: _maxChartPoints,
      lineColor: AppColors.primary,
    );
  }

  if (_terminalEntries.isEmpty) {
    return _buildEmptyTerminal();
  }

  return Scrollbar(
    controller: _terminalScrollController,
    thumbVisibility: true,
    child: ListView.builder(
      controller: _terminalScrollController,
      padding: const EdgeInsets.all(12),
      itemCount: _terminalEntries.length,
      itemBuilder: (context, index) {
        return _buildTerminalLine(_terminalEntries[index]);
      },
    ),
  );
}
```

---

### 4.2 Terminal Header Refinement

**Code Update**:
```dart
Widget _buildTerminalHeader() {
  return Container(
    padding: const EdgeInsets.symmetric(horizontal: 16, vertical: 10),
    color: AppColors.surface,
    child: Row(
      children: [
        // Title
        Row(
          mainAxisSize: MainAxisSize.min,
          children: [
            const MacosIcon(
              CupertinoIcons.terminal,
              size: 14,
              color: AppColors.primary,
            ),
            const SizedBox(width: 8),
            Text(
              'Terminal',
              style: AppTypography.labelMedium,
            ),
          ],
        ),
        const Spacer(),

        // TX Counter
        _buildCounterBadge(
          label: 'TX',
          count: _txCount,
          backgroundColor: AppColors.txBadge,
          textColor: AppColors.txText,
        ),
        const SizedBox(width: 8),

        // RX Counter
        _buildCounterBadge(
          label: 'RX',
          count: _rxCount,
          backgroundColor: AppColors.rxBadge,
          textColor: AppColors.rxText,
        ),
        const SizedBox(width: 12),

        // Clear button
        MacosIconButton(
          icon: const MacosIcon(
            CupertinoIcons.trash,
            size: 14,
            color: AppColors.textSecondary,
          ),
          onPressed: _clearTerminal,
          padding: const EdgeInsets.all(4),
        ),
      ],
    ),
  );
}

Widget _buildCounterBadge({
  required String label,
  required int count,
  required Color backgroundColor,
  required Color textColor,
}) {
  return Container(
    padding: const EdgeInsets.symmetric(horizontal: 8, vertical: 3),
    decoration: BoxDecoration(
      color: backgroundColor,
      borderRadius: BorderRadius.circular(4),
    ),
    child: Text(
      '$label: $count',
      style: AppTypography.badge.copyWith(color: textColor),
    ),
  );
}
```

---

### 4.3 Terminal Line Styling

**Code Update**:
```dart
Widget _buildTerminalLine(TerminalEntry entry) {
  final isTx = entry.type == EntryType.tx;
  final dataText = _displayOptions.hexMode
      ? entry.data
          .map((b) => b.toRadixString(16).padLeft(2, '0').toUpperCase())
          .join(' ')
      : String.fromCharCodes(entry.data)
          .replaceAll('\r', '')
          .replaceAll('\n', '');

  return Container(
    margin: const EdgeInsets.only(bottom: 3),
    child: Row(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        // Left indicator
        Container(
          width: 3,
          height: 20,
          margin: const EdgeInsets.only(right: 8),
          decoration: BoxDecoration(
            color: isTx ? AppColors.warning : AppColors.primary,
            borderRadius: BorderRadius.circular(1.5),
          ),
        ),

        // Timestamp
        SizedBox(
          width: 60,
          child: Text(
            _formatTime(entry.timestamp),
            style: AppTypography.timestamp,
          ),
        ),

        // Badge
        Container(
          width: 28,
          margin: const EdgeInsets.only(right: 8),
          padding: const EdgeInsets.symmetric(horizontal: 4, vertical: 1),
          decoration: BoxDecoration(
            color: isTx ? AppColors.txBadge : AppColors.rxBadge,
            borderRadius: BorderRadius.circular(3),
          ),
          child: Text(
            isTx ? 'TX' : 'RX',
            style: AppTypography.badge.copyWith(
              color: isTx ? AppColors.txText : AppColors.rxText,
              fontSize: 9,
            ),
            textAlign: TextAlign.center,
          ),
        ),

        // Data
        Expanded(
          child: SelectableText(
            dataText,
            style: AppTypography.terminal,
          ),
        ),
      ],
    ),
  );
}
```

---

### 4.4 Empty Terminal State

**Code Update**:
```dart
Widget _buildEmptyTerminal() {
  return Center(
    child: Column(
      mainAxisSize: MainAxisSize.min,
      children: [
        MacosIcon(
          CupertinoIcons.terminal,
          size: 40,
          color: AppColors.textTertiary.withOpacity(0.5),
        ),
        const SizedBox(height: 12),
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
              : 'Connect to start',
          style: AppTypography.bodySmall.copyWith(
            color: AppColors.textTertiary,
          ),
        ),
      ],
    ),
  );
}
```

---

### 4.5 Send Panel Refinement

**Code Update**:
```dart
Widget _buildSendPanel() {
  return Container(
    padding: const EdgeInsets.symmetric(horizontal: 16, vertical: 12),
    decoration: BoxDecoration(
      color: AppColors.surface,
      border: Border(
        top: BorderSide(
          color: AppColors.divider.withOpacity(0.5),
        ),
      ),
    ),
    child: Row(
      children: [
        // Mode toggle
        MacosSegmentedControl(
          controller: MacosTabController(
            initialIndex: _displayOptions.hexMode ? 1 : 0,
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
          child: SizedBox(
            height: 28,
            child: MacosTextField(
              controller: _sendController,
              enabled: _isConnected,
              placeholder: _displayOptions.hexMode
                  ? 'Hex: 48 65 6C 6C 6F'
                  : 'Enter message...',
              style: AppTypography.terminal.copyWith(fontSize: 12),
              padding: const EdgeInsets.symmetric(horizontal: 10, vertical: 6),
              clearButtonMode: OverlayVisibilityMode.editing,
              onSubmitted: (_) => _sendData(),
            ),
          ),
        ),
        const SizedBox(width: 12),

        // Send button
        PushButton(
          controlSize: ControlSize.regular,
          onPressed: _isConnected && _sendController.text.isNotEmpty
              ? _sendData
              : null,
          child: const Row(
            mainAxisSize: MainAxisSize.min,
            children: [
              MacosIcon(
                CupertinoIcons.paperplane_fill,
                size: 12,
                color: MacosColors.white,
              ),
              SizedBox(width: 6),
              Text('Send'),
            ],
          ),
        ),
      ],
    ),
  );
}
```

---

## Verification Steps

1. **Visual Check**:
   - Terminal area có subtle shadow
   - Header clean với proper spacing
   - TX/RX counters visible và readable
   - Terminal lines aligned và consistent
   - Empty state centered và subtle
   - Send panel compact và functional

2. **Functionality Check**:
   - Clear button works
   - Auto scroll works
   - Text/Hex toggle works
   - Send button works
   - Enter key submits

3. **Performance Check**:
   - Smooth scrolling với nhiều entries
   - No jank khi receive data nhanh

---

## Next Phase

Proceed to **Phase 5: Polish & Testing** để finalize styling và comprehensive testing.
