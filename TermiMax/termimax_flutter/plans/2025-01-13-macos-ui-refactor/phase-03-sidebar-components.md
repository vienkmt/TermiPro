# Phase 3: Sidebar Components

**Status**: Pending
**Priority**: High
**Dependencies**: Phase 2 complete

---

## Objective

Refactor tất cả sidebar components sử dụng macos_ui widgets thay vì Material/custom widgets.

---

## Tasks

### 3.1 Update port_selector.dart

**File**: `lib/widgets/sidebar/port_selector.dart`

**Changes**:
- Replace custom dropdown với `MacosPopupButton`
- Replace `ElevatedButton` với `PushButton`
- Remove overlay-based dropdown logic (MacosPopupButton handles this)

**Updated Code**:
```dart
import 'package:flutter/cupertino.dart';
import 'package:macos_ui/macos_ui.dart';
import 'package:termimax_flutter/theme/theme.dart';
import 'package:termimax_flutter/src/rust/api/models.dart';

class PortSelector extends StatelessWidget {
  final List<PortInfo> ports;
  final String? selectedPort;
  final bool isConnected;
  final bool isLoading;
  final VoidCallback onRefresh;
  final ValueChanged<String?> onPortChanged;
  final VoidCallback onConnect;

  const PortSelector({
    super.key,
    required this.ports,
    this.selectedPort,
    this.isConnected = false,
    this.isLoading = false,
    required this.onRefresh,
    required this.onPortChanged,
    required this.onConnect,
  });

  @override
  Widget build(BuildContext context) {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        // Label
        Text(
          'Serial Port',
          style: AppTypography.labelSmall.copyWith(
            color: AppColors.textSecondary,
          ),
        ),
        const SizedBox(height: 6),

        // Port dropdown
        Row(
          children: [
            Expanded(
              child: MacosPopupButton<String>(
                value: selectedPort,
                onChanged: isConnected ? null : (value) {
                  if (value != null) onPortChanged(value);
                },
                items: [
                  if (ports.isEmpty)
                    const MacosPopupMenuItem(
                      value: '',
                      child: Text('No ports found'),
                    )
                  else
                    ...ports.map((port) => MacosPopupMenuItem(
                      value: port.name,
                      child: Row(
                        children: [
                          const MacosIcon(
                            CupertinoIcons.device_phone_portrait,
                            size: 14,
                          ),
                          const SizedBox(width: 8),
                          Expanded(
                            child: Column(
                              crossAxisAlignment: CrossAxisAlignment.start,
                              mainAxisSize: MainAxisSize.min,
                              children: [
                                Text(port.portType),
                                if (port.product != null)
                                  Text(
                                    port.product!,
                                    style: AppTypography.bodySmall,
                                    overflow: TextOverflow.ellipsis,
                                  ),
                              ],
                            ),
                          ),
                        ],
                      ),
                    )),
                ],
              ),
            ),
            const SizedBox(width: 8),
            // Refresh button
            MacosIconButton(
              icon: isLoading
                  ? const SizedBox(
                      width: 16,
                      height: 16,
                      child: ProgressCircle(),
                    )
                  : const MacosIcon(
                      CupertinoIcons.refresh,
                      size: 16,
                    ),
              onPressed: isConnected || isLoading ? null : onRefresh,
            ),
          ],
        ),
        const SizedBox(height: 12),

        // Connect button
        SizedBox(
          width: double.infinity,
          child: PushButton(
            controlSize: ControlSize.large,
            color: isConnected
                ? MacosColors.systemRedColor
                : MacosColors.controlAccentColor,
            onPressed: selectedPort != null && selectedPort!.isNotEmpty
                ? onConnect
                : null,
            child: Row(
              mainAxisAlignment: MainAxisAlignment.center,
              children: [
                MacosIcon(
                  isConnected
                      ? CupertinoIcons.link_badge_minus
                      : CupertinoIcons.link,
                  size: 16,
                  color: MacosColors.white,
                ),
                const SizedBox(width: 8),
                Text(
                  isConnected ? 'Disconnect' : 'Connect',
                  style: const TextStyle(color: MacosColors.white),
                ),
              ],
            ),
          ),
        ),
      ],
    );
  }
}
```

---

### 3.2 Update config_card.dart

**File**: `lib/widgets/sidebar/config_card.dart`

**Changes**:
- Remove border, add subtle shadow
- Replace `CustomDropdown` với `MacosPopupButton`

**Updated Code**:
```dart
import 'package:flutter/cupertino.dart';
import 'package:macos_ui/macos_ui.dart';
import 'package:termimax_flutter/theme/theme.dart';
import 'package:termimax_flutter/models/serial_config.dart';

class ConfigCard extends StatelessWidget {
  final SerialConfigModel config;
  final bool enabled;
  final ValueChanged<SerialConfigModel> onChanged;

  const ConfigCard({
    super.key,
    required this.config,
    this.enabled = true,
    required this.onChanged,
  });

  @override
  Widget build(BuildContext context) {
    return Container(
      padding: const EdgeInsets.all(16),
      decoration: BoxDecoration(
        color: AppColors.surface,
        borderRadius: BorderRadius.circular(10),
        boxShadow: AppColors.subtleShadow,
      ),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          // Header
          Row(
            children: [
              const MacosIcon(
                CupertinoIcons.gear,
                size: 16,
                color: AppColors.primary,
              ),
              const SizedBox(width: 8),
              Text('Configuration', style: AppTypography.labelLarge),
            ],
          ),
          const SizedBox(height: 16),

          // Baud Rate
          _buildDropdown(
            label: 'Baud Rate',
            value: config.baudRate.toString(),
            enabled: enabled,
            items: SerialConfigModel.baudRates
                .map((rate) => rate.toString())
                .toList(),
            onChanged: (value) {
              final rate = int.tryParse(value ?? '');
              if (rate != null) {
                onChanged(config.copyWith(baudRate: rate));
              }
            },
          ),
          const SizedBox(height: 12),

          // Data Bits & Stop Bits row
          Row(
            children: [
              Expanded(
                child: _buildDropdown(
                  label: 'Data Bits',
                  value: config.dataBits.toString(),
                  enabled: enabled,
                  items: SerialConfigModel.dataBitsOptions
                      .map((bits) => bits.toString())
                      .toList(),
                  onChanged: (value) {
                    final bits = int.tryParse(value ?? '');
                    if (bits != null) {
                      onChanged(config.copyWith(dataBits: bits));
                    }
                  },
                ),
              ),
              const SizedBox(width: 12),
              Expanded(
                child: _buildDropdown(
                  label: 'Stop Bits',
                  value: config.stopBits,
                  enabled: enabled,
                  items: SerialConfigModel.stopBitsOptions,
                  onChanged: (value) {
                    if (value != null) {
                      onChanged(config.copyWith(stopBits: value));
                    }
                  },
                ),
              ),
            ],
          ),
          const SizedBox(height: 12),

          // Parity
          _buildDropdown(
            label: 'Parity',
            value: config.parity,
            enabled: enabled,
            items: SerialConfigModel.parityOptions,
            onChanged: (value) {
              if (value != null) {
                onChanged(config.copyWith(parity: value));
              }
            },
          ),
        ],
      ),
    );
  }

  Widget _buildDropdown({
    required String label,
    required String value,
    required bool enabled,
    required List<String> items,
    required ValueChanged<String?> onChanged,
  }) {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        Text(
          label,
          style: AppTypography.labelSmall.copyWith(
            color: AppColors.textSecondary,
          ),
        ),
        const SizedBox(height: 6),
        MacosPopupButton<String>(
          value: value,
          onChanged: enabled ? onChanged : null,
          items: items
              .map((item) => MacosPopupMenuItem(
                    value: item,
                    child: Text(item),
                  ))
              .toList(),
        ),
      ],
    );
  }
}
```

---

### 3.3 Update signal_toggles.dart

**File**: `lib/widgets/sidebar/signal_toggles.dart`

**Changes**:
- Replace `Switch` với `MacosSwitch`
- Remove ON/OFF status badges (switch is self-explanatory)
- Simplified styling

**Updated Code**:
```dart
import 'package:flutter/cupertino.dart';
import 'package:macos_ui/macos_ui.dart';
import 'package:termimax_flutter/theme/theme.dart';

class SignalToggles extends StatelessWidget {
  final bool dtr;
  final bool rts;
  final bool enabled;
  final ValueChanged<bool> onDtrChanged;
  final ValueChanged<bool> onRtsChanged;

  const SignalToggles({
    super.key,
    required this.dtr,
    required this.rts,
    this.enabled = true,
    required this.onDtrChanged,
    required this.onRtsChanged,
  });

  @override
  Widget build(BuildContext context) {
    return Container(
      padding: const EdgeInsets.all(16),
      decoration: BoxDecoration(
        color: AppColors.surface,
        borderRadius: BorderRadius.circular(10),
        boxShadow: AppColors.subtleShadow,
      ),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          // Header
          Row(
            children: [
              const MacosIcon(
                CupertinoIcons.antenna_radiowaves_left_right,
                size: 16,
                color: AppColors.primary,
              ),
              const SizedBox(width: 8),
              Text('Signal Lines', style: AppTypography.labelLarge),
              const Spacer(),
              HelpButton(
                onPressed: () => _showHelpDialog(context),
              ),
            ],
          ),
          const SizedBox(height: 16),

          // DTR Toggle
          _buildToggleRow(
            label: 'DTR',
            description: 'Data Terminal Ready',
            value: dtr,
            onChanged: enabled ? onDtrChanged : null,
          ),
          const SizedBox(height: 12),

          // RTS Toggle
          _buildToggleRow(
            label: 'RTS',
            description: 'Request to Send',
            value: rts,
            onChanged: enabled ? onRtsChanged : null,
          ),
        ],
      ),
    );
  }

  Widget _buildToggleRow({
    required String label,
    required String description,
    required bool value,
    ValueChanged<bool>? onChanged,
  }) {
    return Row(
      children: [
        Expanded(
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              Text(
                label,
                style: AppTypography.labelMedium.copyWith(
                  color: enabled ? AppColors.textPrimary : AppColors.textTertiary,
                ),
              ),
              Text(
                description,
                style: AppTypography.bodySmall.copyWith(
                  color: AppColors.textTertiary,
                ),
              ),
            ],
          ),
        ),
        MacosSwitch(
          value: value,
          onChanged: onChanged,
        ),
      ],
    );
  }

  void _showHelpDialog(BuildContext context) {
    showMacosAlertDialog(
      context: context,
      builder: (context) => MacosAlertDialog(
        appIcon: const MacosIcon(
          CupertinoIcons.question_circle,
          size: 56,
          color: AppColors.primary,
        ),
        title: const Text('Signal Lines'),
        message: const Text(
          'DTR (Data Terminal Ready) - Indicates terminal is ready. '
          'Many devices use this for reset.\n\n'
          'RTS (Request to Send) - Used for hardware flow control.\n\n'
          'Arduino/ESP32: Keep DTR ON for auto-reset.',
        ),
        primaryButton: PushButton(
          controlSize: ControlSize.large,
          onPressed: () => Navigator.pop(context),
          child: const Text('Got it'),
        ),
      ),
    );
  }
}
```

---

### 3.4 Update display_options.dart

**File**: `lib/widgets/sidebar/display_options.dart`

**Changes**:
- Replace toggle buttons với `MacosSegmentedControl`
- Replace `Switch` với `MacosSwitch`
- Replace `CustomDropdown` với `MacosPopupButton`

**Updated Code**:
```dart
import 'package:flutter/cupertino.dart';
import 'package:macos_ui/macos_ui.dart';
import 'package:termimax_flutter/theme/theme.dart';
import 'package:termimax_flutter/models/serial_config.dart';

class DisplayOptionsCard extends StatefulWidget {
  final DisplayOptions options;
  final ValueChanged<DisplayOptions> onChanged;

  const DisplayOptionsCard({
    super.key,
    required this.options,
    required this.onChanged,
  });

  @override
  State<DisplayOptionsCard> createState() => _DisplayOptionsCardState();
}

class _DisplayOptionsCardState extends State<DisplayOptionsCard> {
  late MacosTabController _tabController;

  @override
  void initState() {
    super.initState();
    _tabController = MacosTabController(
      initialIndex: _getModeIndex(widget.options.displayMode),
      length: 3,
    );
  }

  @override
  void didUpdateWidget(DisplayOptionsCard oldWidget) {
    super.didUpdateWidget(oldWidget);
    if (oldWidget.options.displayMode != widget.options.displayMode) {
      _tabController.index = _getModeIndex(widget.options.displayMode);
    }
  }

  int _getModeIndex(String mode) {
    switch (mode) {
      case 'Text': return 0;
      case 'Hex': return 1;
      case 'Chart': return 2;
      default: return 0;
    }
  }

  String _getMode(int index) {
    switch (index) {
      case 0: return 'Text';
      case 1: return 'Hex';
      case 2: return 'Chart';
      default: return 'Text';
    }
  }

  @override
  void dispose() {
    _tabController.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return Container(
      padding: const EdgeInsets.all(16),
      decoration: BoxDecoration(
        color: AppColors.surface,
        borderRadius: BorderRadius.circular(10),
        boxShadow: AppColors.subtleShadow,
      ),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          // Header
          Row(
            children: [
              const MacosIcon(
                CupertinoIcons.display,
                size: 16,
                color: AppColors.primary,
              ),
              const SizedBox(width: 8),
              Text('Display', style: AppTypography.labelLarge),
            ],
          ),
          const SizedBox(height: 16),

          // Display Mode
          Text(
            'Display Mode',
            style: AppTypography.labelSmall.copyWith(
              color: AppColors.textSecondary,
            ),
          ),
          const SizedBox(height: 8),
          SizedBox(
            width: double.infinity,
            child: MacosSegmentedControl(
              controller: _tabController,
              tabs: const [
                MacosTab(label: 'Text'),
                MacosTab(label: 'Hex'),
                MacosTab(label: 'Chart'),
              ],
              onChanged: (index) {
                widget.onChanged(
                  widget.options.copyWith(displayMode: _getMode(index)),
                );
              },
            ),
          ),
          const SizedBox(height: 16),

          // Auto Scroll
          Row(
            children: [
              Expanded(
                child: Text(
                  'Auto Scroll',
                  style: AppTypography.bodyMedium,
                ),
              ),
              MacosSwitch(
                value: widget.options.autoScroll,
                onChanged: (value) {
                  widget.onChanged(
                    widget.options.copyWith(autoScroll: value),
                  );
                },
              ),
            ],
          ),
          const SizedBox(height: 12),

          // Line Ending
          Text(
            'Line Ending',
            style: AppTypography.labelSmall.copyWith(
              color: AppColors.textSecondary,
            ),
          ),
          const SizedBox(height: 6),
          MacosPopupButton<String>(
            value: widget.options.lineEnding,
            onChanged: (value) {
              if (value != null) {
                widget.onChanged(
                  widget.options.copyWith(lineEnding: value),
                );
              }
            },
            items: DisplayOptions.lineEndingOptions
                .map((ending) => MacosPopupMenuItem(
                      value: ending,
                      child: Text(ending),
                    ))
                .toList(),
          ),
        ],
      ),
    );
  }
}
```

---

### 3.5 Update auto_send_card.dart

**File**: `lib/widgets/sidebar/auto_send_card.dart`

**Changes**:
- Replace `TextField` với `MacosTextField`
- Use `MacosIcon` cho icons
- Simplified stepper buttons

**Updated Code**:
```dart
import 'package:flutter/cupertino.dart';
import 'package:flutter/services.dart';
import 'package:macos_ui/macos_ui.dart';
import 'package:termimax_flutter/theme/theme.dart';
import 'package:termimax_flutter/models/serial_config.dart';

class AutoSendCard extends StatelessWidget {
  final AutoSendSettings settings;
  final bool isConnected;
  final ValueChanged<AutoSendSettings> onChanged;

  const AutoSendCard({
    super.key,
    required this.settings,
    required this.isConnected,
    required this.onChanged,
  });

  @override
  Widget build(BuildContext context) {
    return Container(
      padding: const EdgeInsets.all(16),
      decoration: BoxDecoration(
        color: AppColors.surface,
        borderRadius: BorderRadius.circular(10),
        boxShadow: AppColors.subtleShadow,
      ),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          // Header
          Row(
            children: [
              const MacosIcon(
                CupertinoIcons.timer,
                size: 16,
                color: AppColors.primary,
              ),
              const SizedBox(width: 8),
              Text('Auto Send', style: AppTypography.labelLarge),
              const Spacer(),
              if (settings.enabled)
                Container(
                  padding: const EdgeInsets.symmetric(horizontal: 8, vertical: 3),
                  decoration: BoxDecoration(
                    color: AppColors.successLight,
                    borderRadius: BorderRadius.circular(4),
                  ),
                  child: Text(
                    'ACTIVE',
                    style: AppTypography.badge.copyWith(
                      color: AppColors.successDark,
                    ),
                  ),
                ),
            ],
          ),
          const SizedBox(height: 16),

          // Interval
          _buildNumberInput(
            context,
            label: 'Interval (ms)',
            value: settings.intervalMs,
            min: 50,
            max: 60000,
            enabled: !settings.enabled,
            onChanged: (value) {
              onChanged(settings.copyWith(intervalMs: value));
            },
          ),
          const SizedBox(height: 12),

          // Byte Delay
          _buildNumberInput(
            context,
            label: 'Byte Delay (µs)',
            value: settings.byteDelayUs,
            min: 0,
            max: 10000,
            enabled: !settings.enabled,
            onChanged: (value) {
              onChanged(settings.copyWith(byteDelayUs: value));
            },
          ),

          if (settings.enabled) ...[
            const SizedBox(height: 12),
            // Send count
            Container(
              padding: const EdgeInsets.all(12),
              decoration: BoxDecoration(
                color: AppColors.surfaceVariant,
                borderRadius: BorderRadius.circular(8),
              ),
              child: Row(
                children: [
                  const MacosIcon(
                    CupertinoIcons.paperplane,
                    size: 14,
                    color: AppColors.textSecondary,
                  ),
                  const SizedBox(width: 8),
                  Text(
                    'Messages sent:',
                    style: AppTypography.bodySmall,
                  ),
                  const Spacer(),
                  Text(
                    '${settings.sendCount}',
                    style: AppTypography.stats.copyWith(
                      color: AppColors.primary,
                    ),
                  ),
                ],
              ),
            ),
          ],
        ],
      ),
    );
  }

  Widget _buildNumberInput(
    BuildContext context, {
    required String label,
    required int value,
    required int min,
    required int max,
    required bool enabled,
    required ValueChanged<int> onChanged,
  }) {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        Text(
          label,
          style: AppTypography.labelSmall.copyWith(
            color: AppColors.textSecondary,
          ),
        ),
        const SizedBox(height: 6),
        Row(
          children: [
            Expanded(
              child: MacosTextField(
                controller: TextEditingController(text: value.toString()),
                enabled: enabled,
                inputFormatters: [
                  FilteringTextInputFormatter.digitsOnly,
                ],
                style: AppTypography.terminal,
                onChanged: (text) {
                  final parsed = int.tryParse(text);
                  if (parsed != null && parsed >= min && parsed <= max) {
                    onChanged(parsed);
                  }
                },
              ),
            ),
            const SizedBox(width: 8),
            Column(
              children: [
                _buildStepButton(
                  icon: CupertinoIcons.plus,
                  enabled: enabled && value < max,
                  onTap: () {
                    final step = _getStep(value);
                    onChanged((value + step).clamp(min, max));
                  },
                ),
                const SizedBox(height: 2),
                _buildStepButton(
                  icon: CupertinoIcons.minus,
                  enabled: enabled && value > min,
                  onTap: () {
                    final step = _getStep(value);
                    onChanged((value - step).clamp(min, max));
                  },
                ),
              ],
            ),
          ],
        ),
      ],
    );
  }

  int _getStep(int currentValue) {
    if (currentValue >= 1000) return 100;
    if (currentValue >= 100) return 50;
    return 10;
  }

  Widget _buildStepButton({
    required IconData icon,
    required bool enabled,
    required VoidCallback onTap,
  }) {
    return GestureDetector(
      onTap: enabled ? onTap : null,
      child: Container(
        width: 24,
        height: 18,
        decoration: BoxDecoration(
          color: enabled ? AppColors.surfaceVariant : AppColors.border,
          borderRadius: BorderRadius.circular(4),
        ),
        child: Center(
          child: MacosIcon(
            icon,
            size: 12,
            color: enabled ? AppColors.textSecondary : AppColors.textTertiary,
          ),
        ),
      ),
    );
  }
}
```

---

### 3.6 Remove/Update custom_dropdown.dart

**File**: `lib/widgets/common/custom_dropdown.dart`

**Action**: File này có thể bị **deprecated** vì đã sử dụng `MacosPopupButton`.

**Options**:
1. **Remove file** nếu không còn được sử dụng
2. **Keep as backup** nhưng không import
3. **Refactor** thành wrapper cho MacosPopupButton (không cần thiết)

**Recommendation**: Remove file sau khi verify tất cả components đã migrate.

---

## Verification Steps

1. **Build Check**:
   ```bash
   flutter build macos
   ```

2. **Visual Verification**:
   - Port selector dropdown native style
   - Config dropdowns native style
   - Switches là MacosSwitch
   - Cards có subtle shadows thay vì borders
   - Icons là CupertinoIcons

3. **Functionality Check**:
   - Port selection works
   - Config changes work
   - DTR/RTS toggles work
   - Display mode toggles work
   - Auto send settings work

---

## Next Phase

Proceed to **Phase 4: Main Content Area** để polish terminal display và send panel.
