import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:termimax_flutter/theme/theme.dart';
import 'package:termimax_flutter/models/serial_config.dart';

/// Auto send section
class AutoSendCard extends StatefulWidget {
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
  State<AutoSendCard> createState() => _AutoSendCardState();
}

class _AutoSendCardState extends State<AutoSendCard> {
  late TextEditingController _intervalController;
  final FocusNode _focusNode = FocusNode();
  bool _isFocused = false;

  @override
  void initState() {
    super.initState();
    _intervalController = TextEditingController(text: widget.settings.intervalMs.toString());
    _focusNode.addListener(() {
      setState(() => _isFocused = _focusNode.hasFocus);
    });
  }

  @override
  void didUpdateWidget(AutoSendCard oldWidget) {
    super.didUpdateWidget(oldWidget);
    // Only update text if not focused (avoid select all issue while typing)
    if (oldWidget.settings.intervalMs != widget.settings.intervalMs && !_focusNode.hasFocus) {
      _intervalController.text = widget.settings.intervalMs.toString();
    }
  }

  @override
  void dispose() {
    _intervalController.dispose();
    _focusNode.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    final enabled = !widget.settings.enabled;

    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        Row(
          children: [
            Text('Auto Send', style: AppTypography.sectionTitle),
            const Spacer(),
            if (widget.settings.enabled)
              Text('ON', style: TextStyle(fontSize: 10, fontWeight: FontWeight.w600, color: AppColors.success)),
          ],
        ),
        const SizedBox(height: 8),
        Row(
          children: [
            Text('Interval', style: TextStyle(fontSize: 11, color: AppColors.textSecondary)),
            const Spacer(),
            AnimatedContainer(
              duration: const Duration(milliseconds: 150),
              width: 70,
              decoration: BoxDecoration(
                borderRadius: BorderRadius.circular(4),
                boxShadow: _isFocused
                    ? [BoxShadow(color: AppColors.primary.withOpacity(0.2), blurRadius: 4, spreadRadius: 1)]
                    : [],
              ),
              child: TextField(
                controller: _intervalController,
                focusNode: _focusNode,
                enabled: enabled,
                keyboardType: TextInputType.number,
                inputFormatters: [FilteringTextInputFormatter.digitsOnly],
                style: TextStyle(fontSize: 12, color: AppColors.textPrimary),
                textAlign: TextAlign.center,
                decoration: InputDecoration(
                  isDense: true,
                  contentPadding: const EdgeInsets.symmetric(horizontal: 8, vertical: 8),
                  border: OutlineInputBorder(
                    borderRadius: BorderRadius.circular(4),
                    borderSide: BorderSide(color: AppColors.border),
                  ),
                  enabledBorder: OutlineInputBorder(
                    borderRadius: BorderRadius.circular(4),
                    borderSide: BorderSide(color: AppColors.border),
                  ),
                  focusedBorder: OutlineInputBorder(
                    borderRadius: BorderRadius.circular(4),
                    borderSide: BorderSide(color: AppColors.primary, width: 1.5),
                  ),
                  filled: true,
                  fillColor: AppColors.surface,
                ),
                onChanged: (text) {
                  final v = int.tryParse(text);
                  if (v != null) {
                    widget.onChanged(widget.settings.copyWith(intervalMs: v));
                  }
                },
              ),
            ),
            const SizedBox(width: 4),
            Text('ms', style: TextStyle(fontSize: 10, color: AppColors.textTertiary)),
          ],
        ),
        if (widget.settings.enabled) ...[
          const SizedBox(height: 6),
          Text(
            'Sent: ${widget.settings.sendCount}',
            style: AppTypography.bodySmall.copyWith(color: AppColors.textTertiary),
          ),
        ],
      ],
    );
  }
}
