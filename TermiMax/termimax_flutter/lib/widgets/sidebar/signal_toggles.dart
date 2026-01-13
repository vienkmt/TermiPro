import 'package:flutter/material.dart';
import 'package:macos_ui/macos_ui.dart' show MacosSwitch;
import 'package:termimax_flutter/theme/theme.dart';

/// Signal lines section - flat design
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
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        Text('Signals', style: AppTypography.sectionTitle),
        const SizedBox(height: 12),
        Row(
          children: [
            Expanded(child: _buildToggle('DTR', dtr, enabled ? onDtrChanged : null)),
            const SizedBox(width: 16),
            Expanded(child: _buildToggle('RTS', rts, enabled ? onRtsChanged : null)),
          ],
        ),
      ],
    );
  }

  Widget _buildToggle(String label, bool value, ValueChanged<bool>? onChanged) {
    return Row(
      children: [
        Text(
          label,
          style: AppTypography.bodySmall.copyWith(
            color: enabled ? AppColors.textSecondary : AppColors.textTertiary,
          ),
        ),
        const Spacer(),
        MacosSwitch(
          value: value,
          onChanged: onChanged,
        ),
      ],
    );
  }
}
