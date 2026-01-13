import 'package:flutter/material.dart';
import 'package:termimax_flutter/theme/theme.dart';

/// Compact dropdown - professional style
class CustomDropdown<T> extends StatefulWidget {
  final T value;
  final List<DropdownItem<T>> items;
  final ValueChanged<T> onChanged;
  final String? label;
  final IconData? icon;
  final bool enabled;
  final double? width;

  const CustomDropdown({
    super.key,
    required this.value,
    required this.items,
    required this.onChanged,
    this.label,
    this.icon,
    this.enabled = true,
    this.width,
  });

  @override
  State<CustomDropdown<T>> createState() => _CustomDropdownState<T>();
}

class _CustomDropdownState<T> extends State<CustomDropdown<T>> {
  final LayerLink _layerLink = LayerLink();
  OverlayEntry? _overlayEntry;
  bool _isOpen = false;

  @override
  void dispose() {
    _removeOverlay();
    super.dispose();
  }

  void _toggleDropdown() {
    if (!widget.enabled) return;
    if (_isOpen) {
      _removeOverlay();
    } else {
      _showOverlay();
    }
  }

  void _showOverlay() {
    final renderBox = context.findRenderObject() as RenderBox;
    final size = renderBox.size;

    _overlayEntry = OverlayEntry(
      builder: (context) => Stack(
        children: [
          Positioned.fill(
            child: GestureDetector(
              onTap: _removeOverlay,
              behavior: HitTestBehavior.opaque,
              child: Container(color: Colors.transparent),
            ),
          ),
          Positioned(
            width: widget.width ?? size.width,
            child: CompositedTransformFollower(
              link: _layerLink,
              showWhenUnlinked: false,
              offset: Offset(0, size.height + 2),
              child: Material(
                elevation: 8,
                shadowColor: Colors.black.withOpacity(0.15),
                borderRadius: BorderRadius.circular(4),
                color: AppColors.surface,
                child: Container(
                  constraints: const BoxConstraints(maxHeight: 200),
                  child: ClipRRect(
                    borderRadius: BorderRadius.circular(4),
                    child: SingleChildScrollView(
                      child: Column(
                        mainAxisSize: MainAxisSize.min,
                        children: widget.items.map((item) {
                          final isSelected = item.value == widget.value;
                          return InkWell(
                            onTap: () {
                              widget.onChanged(item.value);
                              _removeOverlay();
                            },
                            child: Container(
                              width: double.infinity,
                              padding: const EdgeInsets.symmetric(
                                horizontal: 10,
                                vertical: 8,
                              ),
                              color: isSelected
                                  ? AppColors.primarySurface
                                  : Colors.transparent,
                              child: Text(
                                item.label,
                                style: AppTypography.bodySmall.copyWith(
                                  color: isSelected
                                      ? AppColors.primary
                                      : AppColors.textPrimary,
                                  fontWeight: isSelected
                                      ? FontWeight.w600
                                      : FontWeight.w400,
                                ),
                              ),
                            ),
                          );
                        }).toList(),
                      ),
                    ),
                  ),
                ),
              ),
            ),
          ),
        ],
      ),
    );

    Overlay.of(context).insert(_overlayEntry!);
    setState(() => _isOpen = true);
  }

  void _removeOverlay() {
    _overlayEntry?.remove();
    _overlayEntry = null;
    if (mounted) {
      setState(() => _isOpen = false);
    }
  }

  @override
  Widget build(BuildContext context) {
    final selectedItem = widget.items.firstWhere(
      (item) => item.value == widget.value,
      orElse: () => widget.items.first,
    );

    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      mainAxisSize: MainAxisSize.min,
      children: [
        if (widget.label != null) ...[
          Text(
            widget.label!,
            style: AppTypography.labelSmall.copyWith(
              color: AppColors.textTertiary,
              fontSize: 10,
            ),
          ),
          const SizedBox(height: 4),
        ],
        CompositedTransformTarget(
          link: _layerLink,
          child: GestureDetector(
            onTap: _toggleDropdown,
            child: MouseRegion(
              cursor: widget.enabled ? SystemMouseCursors.click : SystemMouseCursors.basic,
              child: Container(
                width: widget.width,
                padding: const EdgeInsets.symmetric(horizontal: 10, vertical: 7),
                decoration: BoxDecoration(
                  color: widget.enabled
                      ? AppColors.surface
                      : AppColors.surfaceVariant,
                  borderRadius: BorderRadius.circular(4),
                  border: Border.all(
                    color: _isOpen ? AppColors.primary : AppColors.border,
                    width: _isOpen ? 1.5 : 1,
                  ),
                ),
                child: Row(
                  children: [
                    Expanded(
                      child: Text(
                        selectedItem.label,
                        style: AppTypography.bodySmall.copyWith(
                          color: widget.enabled
                              ? AppColors.textPrimary
                              : AppColors.textTertiary,
                          fontWeight: FontWeight.w500,
                        ),
                      ),
                    ),
                    Icon(
                      _isOpen ? Icons.expand_less : Icons.expand_more,
                      size: 16,
                      color: widget.enabled
                          ? AppColors.textSecondary
                          : AppColors.textTertiary,
                    ),
                  ],
                ),
              ),
            ),
          ),
        ),
      ],
    );
  }
}

/// Dropdown item model
class DropdownItem<T> {
  final T value;
  final String label;
  final IconData? icon;

  const DropdownItem({
    required this.value,
    required this.label,
    this.icon,
  });
}
