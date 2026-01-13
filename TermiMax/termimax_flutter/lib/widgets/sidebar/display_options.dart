import 'package:flutter/material.dart';
import 'package:macos_ui/macos_ui.dart' show MacosSwitch;
import 'package:termimax_flutter/theme/theme.dart';
import 'package:termimax_flutter/models/serial_config.dart';

/// Display options section
class DisplayOptionsCard extends StatelessWidget {
  final DisplayOptions options;
  final ValueChanged<DisplayOptions> onChanged;

  const DisplayOptionsCard({
    super.key,
    required this.options,
    required this.onChanged,
  });

  @override
  Widget build(BuildContext context) {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        Text('Display', style: AppTypography.sectionTitle),
        const SizedBox(height: 12),
        // Mode (3 items x 60 = 180)
        _buildRow(
          'Mode',
          _SegmentControl(
            options: DisplayOptions.displayModes,
            selected: options.displayMode,
            onChanged: (v) => onChanged(options.copyWith(displayMode: v)),
            itemWidth: 60,
          ),
        ),
        const SizedBox(height: 10),
        // Line End (4 items x 45 = 180)
        _buildRow(
          'Line End',
          _SegmentControl(
            options: DisplayOptions.lineEndingOptions,
            selected: options.lineEnding,
            onChanged: (v) => onChanged(options.copyWith(lineEnding: v)),
            itemWidth: 45,
          ),
        ),
        const SizedBox(height: 10),
        // Auto Scroll
        _buildRow(
          'Auto Scroll',
          MacosSwitch(
            value: options.autoScroll,
            onChanged: (v) => onChanged(options.copyWith(autoScroll: v)),
          ),
        ),
      ],
    );
  }

  Widget _buildRow(String label, Widget child) {
    return Row(
      children: [
        Text(label, style: TextStyle(fontSize: 11, color: AppColors.textSecondary)),
        const Spacer(),
        child,
      ],
    );
  }
}

/// Animated segment control
class _SegmentControl extends StatefulWidget {
  final List<String> options;
  final String selected;
  final ValueChanged<String> onChanged;
  final double itemWidth;

  const _SegmentControl({
    required this.options,
    required this.selected,
    required this.onChanged,
    required this.itemWidth,
  });

  @override
  State<_SegmentControl> createState() => _SegmentControlState();
}

class _SegmentControlState extends State<_SegmentControl> {
  String? _hoveredOption;

  @override
  Widget build(BuildContext context) {
    return Container(
      height: 28,
      decoration: BoxDecoration(
        color: AppColors.surfaceVariant,
        borderRadius: BorderRadius.circular(4),
      ),
      padding: const EdgeInsets.all(2),
      child: Row(
        mainAxisSize: MainAxisSize.min,
        children: widget.options.map((opt) {
          final isSelected = opt == widget.selected;
          final isHovered = opt == _hoveredOption && !isSelected;

          return GestureDetector(
            onTap: () => widget.onChanged(opt),
            child: MouseRegion(
              cursor: SystemMouseCursors.click,
              onEnter: (_) => setState(() => _hoveredOption = opt),
              onExit: (_) => setState(() => _hoveredOption = null),
              child: AnimatedContainer(
                duration: const Duration(milliseconds: 150),
                width: widget.itemWidth,
                height: 24,
                alignment: Alignment.center,
                decoration: BoxDecoration(
                  color: isSelected
                      ? AppColors.primary
                      : isHovered
                          ? AppColors.border
                          : Colors.transparent,
                  borderRadius: BorderRadius.circular(3),
                  boxShadow: isSelected
                      ? [BoxShadow(color: Colors.black.withOpacity(0.1), blurRadius: 2, offset: const Offset(0, 1))]
                      : [],
                ),
                child: Text(
                  opt,
                  style: TextStyle(
                    fontSize: 11,
                    fontWeight: isSelected ? FontWeight.w600 : FontWeight.w400,
                    color: isSelected ? Colors.white : AppColors.textSecondary,
                  ),
                ),
              ),
            ),
          );
        }).toList(),
      ),
    );
  }
}
