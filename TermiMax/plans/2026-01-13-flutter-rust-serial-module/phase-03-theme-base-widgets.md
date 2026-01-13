# Phase 3: Flutter Theme & Base Widgets

## Objective
Thiết lập theme và base components giống UI gốc (SerialTab.vue)

---

## Tasks

### 3.1 App Colors

```dart
// lib/theme/app_colors.dart
import 'package:flutter/material.dart';

class AppColors {
  // Brand Colors (Sky Blue accent)
  static const Color accent = Color(0xFF0EA5E9);
  static const Color accentSecondary = Color(0xFF6366F1);
  static const Color accentLight = Color(0xFFE0F2FE);

  // Semantic Colors
  static const Color danger = Color(0xFFEF4444);
  static const Color dangerLight = Color(0xFFFEE2E2);
  static const Color success = Color(0xFF22C55E);
  static const Color successLight = Color(0xFFDCFCE7);
  static const Color warning = Color(0xFFF59E0B);
  static const Color warningLight = Color(0xFFFEF3C7);

  // TX/RX Colors (from Vue CSS)
  static const Color txColor = Color(0xFFEA580C);  // Orange
  static const Color rxColor = Color(0xFF0EA5E9);  // Blue (same as accent)

  // Background Colors
  static const Color bgPrimary = Color(0xFFFFFFFF);
  static const Color bgSecondary = Color(0xFFF8FAFC);
  static const Color bgTertiary = Color(0xFFF1F5F9);
  static const Color bgHover = Color(0xFFE2E8F0);

  // Border Colors
  static const Color borderColor = Color(0xFFE2E8F0);
  static const Color borderFocus = Color(0xFF0EA5E9);

  // Text Colors
  static const Color textPrimary = Color(0xFF1E293B);
  static const Color textSecondary = Color(0xFF64748B);
  static const Color textTertiary = Color(0xFF94A3B8);

  // Shadows
  static List<BoxShadow> get shadowSm => [
    BoxShadow(
      color: Colors.black.withOpacity(0.05),
      blurRadius: 4,
      offset: const Offset(0, 1),
    ),
  ];

  static List<BoxShadow> get shadowMd => [
    BoxShadow(
      color: Colors.black.withOpacity(0.1),
      blurRadius: 8,
      offset: const Offset(0, 4),
    ),
  ];

  static List<BoxShadow> get shadowLg => [
    BoxShadow(
      color: Colors.black.withOpacity(0.15),
      blurRadius: 16,
      offset: const Offset(0, 8),
    ),
  ];
}
```

### 3.2 App Theme

```dart
// lib/theme/app_theme.dart
import 'package:flutter/material.dart';
import 'package:google_fonts/google_fonts.dart';
import 'app_colors.dart';

class AppTheme {
  // Border Radius constants
  static const double radiusSm = 4.0;
  static const double radiusMd = 8.0;
  static const double radiusLg = 12.0;

  // Sidebar width
  static const double sidebarWidth = 320.0;

  // Font families
  static String get fontSans => GoogleFonts.plusJakartaSans().fontFamily!;
  static String get fontMono => GoogleFonts.jetBrainsMono().fontFamily!;

  static ThemeData get light {
    return ThemeData(
      useMaterial3: true,
      brightness: Brightness.light,
      colorScheme: ColorScheme.fromSeed(
        seedColor: AppColors.accent,
        brightness: Brightness.light,
        primary: AppColors.accent,
        secondary: AppColors.accentSecondary,
        surface: AppColors.bgPrimary,
        error: AppColors.danger,
      ),
      scaffoldBackgroundColor: AppColors.bgPrimary,
      fontFamily: fontSans,
      textTheme: TextTheme(
        // Headers
        headlineLarge: GoogleFonts.plusJakartaSans(
          fontSize: 24,
          fontWeight: FontWeight.w700,
          color: AppColors.textPrimary,
        ),
        headlineMedium: GoogleFonts.plusJakartaSans(
          fontSize: 20,
          fontWeight: FontWeight.w600,
          color: AppColors.textPrimary,
        ),
        // Body
        bodyLarge: GoogleFonts.plusJakartaSans(
          fontSize: 14,
          fontWeight: FontWeight.w500,
          color: AppColors.textPrimary,
        ),
        bodyMedium: GoogleFonts.plusJakartaSans(
          fontSize: 12,
          fontWeight: FontWeight.w500,
          color: AppColors.textSecondary,
        ),
        bodySmall: GoogleFonts.plusJakartaSans(
          fontSize: 11,
          fontWeight: FontWeight.w500,
          color: AppColors.textTertiary,
        ),
        // Labels
        labelLarge: GoogleFonts.plusJakartaSans(
          fontSize: 13,
          fontWeight: FontWeight.w600,
          color: AppColors.textPrimary,
        ),
        labelMedium: GoogleFonts.plusJakartaSans(
          fontSize: 11,
          fontWeight: FontWeight.w600,
          color: AppColors.textSecondary,
          letterSpacing: 0.5,
        ),
      ),
      dividerColor: AppColors.borderColor,
      dividerTheme: const DividerThemeData(
        color: AppColors.borderColor,
        thickness: 1,
        space: 1,
      ),
      inputDecorationTheme: InputDecorationTheme(
        filled: true,
        fillColor: AppColors.bgTertiary,
        contentPadding: const EdgeInsets.symmetric(horizontal: 12, vertical: 8),
        border: OutlineInputBorder(
          borderRadius: BorderRadius.circular(radiusMd),
          borderSide: const BorderSide(color: AppColors.borderColor),
        ),
        enabledBorder: OutlineInputBorder(
          borderRadius: BorderRadius.circular(radiusMd),
          borderSide: const BorderSide(color: AppColors.borderColor),
        ),
        focusedBorder: OutlineInputBorder(
          borderRadius: BorderRadius.circular(radiusMd),
          borderSide: const BorderSide(color: AppColors.accent, width: 2),
        ),
        hintStyle: GoogleFonts.plusJakartaSans(
          fontSize: 12,
          color: AppColors.textTertiary,
        ),
      ),
      cardTheme: CardTheme(
        color: AppColors.bgSecondary,
        elevation: 0,
        shape: RoundedRectangleBorder(
          borderRadius: BorderRadius.circular(radiusMd),
          side: const BorderSide(color: AppColors.borderColor),
        ),
        margin: EdgeInsets.zero,
      ),
    );
  }
}
```

### 3.3 Custom Dropdown Widget

```dart
// lib/widgets/common/custom_dropdown.dart
import 'package:flutter/material.dart';
import '../../theme/app_colors.dart';
import '../../theme/app_theme.dart';

class CustomDropdown<T> extends StatefulWidget {
  final String label;
  final Widget? icon;
  final T? value;
  final List<DropdownItem<T>> items;
  final ValueChanged<T>? onChanged;
  final bool enabled;
  final String? placeholder;

  const CustomDropdown({
    super.key,
    required this.label,
    this.icon,
    this.value,
    required this.items,
    this.onChanged,
    this.enabled = true,
    this.placeholder,
  });

  @override
  State<CustomDropdown<T>> createState() => _CustomDropdownState<T>();
}

class DropdownItem<T> {
  final T value;
  final String label;
  final String? subtitle;
  final bool enabled;

  const DropdownItem({
    required this.value,
    required this.label,
    this.subtitle,
    this.enabled = true,
  });
}

class _CustomDropdownState<T> extends State<CustomDropdown<T>>
    with SingleTickerProviderStateMixin {
  bool _isOpen = false;
  OverlayEntry? _overlayEntry;
  final LayerLink _layerLink = LayerLink();
  late AnimationController _animationController;
  late Animation<double> _animation;

  @override
  void initState() {
    super.initState();
    _animationController = AnimationController(
      duration: const Duration(milliseconds: 150),
      vsync: this,
    );
    _animation = CurvedAnimation(
      parent: _animationController,
      curve: Curves.easeOut,
    );
  }

  @override
  void dispose() {
    _removeOverlay();
    _animationController.dispose();
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
    _overlayEntry = _createOverlay();
    Overlay.of(context).insert(_overlayEntry!);
    _animationController.forward();
    setState(() => _isOpen = true);
  }

  void _removeOverlay() {
    _animationController.reverse().then((_) {
      _overlayEntry?.remove();
      _overlayEntry = null;
    });
    setState(() => _isOpen = false);
  }

  OverlayEntry _createOverlay() {
    final renderBox = context.findRenderObject() as RenderBox;
    final size = renderBox.size;

    return OverlayEntry(
      builder: (context) => Positioned(
        width: size.width,
        child: CompositedTransformFollower(
          link: _layerLink,
          offset: Offset(0, size.height + 4),
          child: FadeTransition(
            opacity: _animation,
            child: SlideTransition(
              position: Tween<Offset>(
                begin: const Offset(0, -0.1),
                end: Offset.zero,
              ).animate(_animation),
              child: Material(
                elevation: 8,
                borderRadius: BorderRadius.circular(AppTheme.radiusSm),
                child: Container(
                  constraints: const BoxConstraints(maxHeight: 200),
                  decoration: BoxDecoration(
                    color: AppColors.bgSecondary,
                    borderRadius: BorderRadius.circular(AppTheme.radiusSm),
                    border: Border.all(color: AppColors.borderColor),
                  ),
                  child: ListView.builder(
                    padding: EdgeInsets.zero,
                    shrinkWrap: true,
                    itemCount: widget.items.length,
                    itemBuilder: (context, index) {
                      final item = widget.items[index];
                      final isSelected = item.value == widget.value;

                      return InkWell(
                        onTap: item.enabled
                            ? () {
                                widget.onChanged?.call(item.value);
                                _removeOverlay();
                              }
                            : null,
                        child: Container(
                          padding: const EdgeInsets.symmetric(
                            horizontal: 12,
                            vertical: 8,
                          ),
                          decoration: BoxDecoration(
                            color: isSelected
                                ? AppColors.accentLight
                                : Colors.transparent,
                          ),
                          child: Row(
                            children: [
                              Expanded(
                                child: Column(
                                  crossAxisAlignment: CrossAxisAlignment.start,
                                  children: [
                                    Text(
                                      item.label,
                                      style: TextStyle(
                                        fontSize: 12,
                                        fontWeight: isSelected
                                            ? FontWeight.w600
                                            : FontWeight.w500,
                                        color: item.enabled
                                            ? (isSelected
                                                ? AppColors.accent
                                                : AppColors.textPrimary)
                                            : AppColors.textTertiary,
                                      ),
                                    ),
                                    if (item.subtitle != null)
                                      Text(
                                        item.subtitle!,
                                        style: TextStyle(
                                          fontSize: 10,
                                          color: isSelected
                                              ? AppColors.accent
                                              : AppColors.textTertiary,
                                        ),
                                      ),
                                  ],
                                ),
                              ),
                              if (isSelected)
                                const Icon(
                                  Icons.check,
                                  size: 14,
                                  color: AppColors.accent,
                                ),
                            ],
                          ),
                        ),
                      );
                    },
                  ),
                ),
              ),
            ),
          ),
        ),
      ),
    );
  }

  @override
  Widget build(BuildContext context) {
    final selectedItem = widget.items.cast<DropdownItem<T>?>().firstWhere(
          (item) => item?.value == widget.value,
          orElse: () => null,
        );

    return CompositedTransformTarget(
      link: _layerLink,
      child: GestureDetector(
        onTap: _toggleDropdown,
        child: AnimatedContainer(
          duration: const Duration(milliseconds: 200),
          padding: const EdgeInsets.symmetric(horizontal: 10, vertical: 8),
          decoration: BoxDecoration(
            color: _isOpen ? AppColors.bgSecondary : AppColors.bgTertiary,
            borderRadius: BorderRadius.circular(AppTheme.radiusSm),
            border: Border.all(
              color: _isOpen ? AppColors.accent : AppColors.borderColor,
            ),
            boxShadow: _isOpen
                ? [
                    BoxShadow(
                      color: AppColors.accent.withOpacity(0.2),
                      blurRadius: 4,
                      spreadRadius: 1,
                    ),
                  ]
                : null,
          ),
          child: Opacity(
            opacity: widget.enabled ? 1.0 : 0.6,
            child: Row(
              children: [
                if (widget.icon != null) ...[
                  widget.icon!,
                  const SizedBox(width: 6),
                ],
                Text(
                  widget.label,
                  style: const TextStyle(
                    fontSize: 11,
                    fontWeight: FontWeight.w500,
                    color: AppColors.textSecondary,
                  ),
                ),
                const Spacer(),
                Text(
                  selectedItem?.label ?? widget.placeholder ?? '',
                  style: TextStyle(
                    fontSize: 12,
                    fontWeight: FontWeight.w600,
                    color: selectedItem != null
                        ? AppColors.textPrimary
                        : AppColors.textTertiary,
                    fontFamily: AppTheme.fontMono,
                  ),
                ),
                const SizedBox(width: 6),
                AnimatedRotation(
                  duration: const Duration(milliseconds: 200),
                  turns: _isOpen ? 0.5 : 0,
                  child: const Icon(
                    Icons.keyboard_arrow_down,
                    size: 16,
                    color: AppColors.textSecondary,
                  ),
                ),
              ],
            ),
          ),
        ),
      ),
    );
  }
}
```

### 3.4 Toggle Switch Widget

```dart
// lib/widgets/common/toggle_switch.dart
import 'package:flutter/material.dart';
import '../../theme/app_colors.dart';

class ToggleSwitch extends StatelessWidget {
  final bool value;
  final ValueChanged<bool>? onChanged;
  final String? label;
  final bool enabled;
  final bool compact;

  const ToggleSwitch({
    super.key,
    required this.value,
    this.onChanged,
    this.label,
    this.enabled = true,
    this.compact = false,
  });

  @override
  Widget build(BuildContext context) {
    final trackWidth = compact ? 26.0 : 36.0;
    final trackHeight = compact ? 14.0 : 20.0;
    final thumbSize = compact ? 10.0 : 16.0;

    return GestureDetector(
      onTap: enabled ? () => onChanged?.call(!value) : null,
      child: Opacity(
        opacity: enabled ? 1.0 : 0.5,
        child: Row(
          mainAxisSize: MainAxisSize.min,
          children: [
            AnimatedContainer(
              duration: const Duration(milliseconds: 200),
              width: trackWidth,
              height: trackHeight,
              decoration: BoxDecoration(
                color: value ? AppColors.accent : AppColors.bgTertiary,
                borderRadius: BorderRadius.circular(trackHeight / 2),
              ),
              child: AnimatedAlign(
                duration: const Duration(milliseconds: 200),
                curve: Curves.easeInOut,
                alignment:
                    value ? Alignment.centerRight : Alignment.centerLeft,
                child: Container(
                  width: thumbSize,
                  height: thumbSize,
                  margin: const EdgeInsets.all(2),
                  decoration: BoxDecoration(
                    color: Colors.white,
                    shape: BoxShape.circle,
                    boxShadow: AppColors.shadowSm,
                  ),
                ),
              ),
            ),
            if (label != null) ...[
              SizedBox(width: compact ? 4 : 8),
              Text(
                label!,
                style: TextStyle(
                  fontSize: compact ? 10 : 11,
                  fontWeight: FontWeight.w500,
                  color: AppColors.textSecondary,
                ),
              ),
            ],
          ],
        ),
      ),
    );
  }
}
```

### 3.5 Connect Button Widget

```dart
// lib/widgets/common/connect_button.dart
import 'package:flutter/material.dart';
import '../../theme/app_colors.dart';
import '../../theme/app_theme.dart';

class ConnectButton extends StatelessWidget {
  final bool isConnected;
  final VoidCallback? onPressed;
  final bool enabled;

  const ConnectButton({
    super.key,
    required this.isConnected,
    this.onPressed,
    this.enabled = true,
  });

  @override
  Widget build(BuildContext context) {
    return AnimatedContainer(
      duration: const Duration(milliseconds: 300),
      width: double.infinity,
      height: 40,
      decoration: BoxDecoration(
        gradient: LinearGradient(
          colors: isConnected
              ? [AppColors.danger, const Color(0xFFF87171)]
              : [AppColors.accent, AppColors.accentSecondary],
          begin: Alignment.topLeft,
          end: Alignment.bottomRight,
        ),
        borderRadius: BorderRadius.circular(AppTheme.radiusLg),
        boxShadow: [
          BoxShadow(
            color: (isConnected ? AppColors.danger : AppColors.accent)
                .withOpacity(0.35),
            blurRadius: 14,
            offset: const Offset(0, 4),
          ),
        ],
      ),
      child: Material(
        color: Colors.transparent,
        child: InkWell(
          onTap: enabled ? onPressed : null,
          borderRadius: BorderRadius.circular(AppTheme.radiusLg),
          child: Opacity(
            opacity: enabled ? 1.0 : 0.5,
            child: Row(
              mainAxisAlignment: MainAxisAlignment.center,
              children: [
                Icon(
                  isConnected ? Icons.close : Icons.arrow_forward,
                  color: Colors.white,
                  size: 18,
                ),
                const SizedBox(width: 8),
                Text(
                  isConnected ? 'Ngắt kết nối' : 'Kết nối',
                  style: const TextStyle(
                    color: Colors.white,
                    fontSize: 13,
                    fontWeight: FontWeight.w600,
                  ),
                ),
              ],
            ),
          ),
        ),
      ),
    );
  }
}
```

### 3.6 Config Card Widget

```dart
// lib/widgets/common/config_card.dart
import 'package:flutter/material.dart';
import '../../theme/app_colors.dart';
import '../../theme/app_theme.dart';

class ConfigCard extends StatelessWidget {
  final String title;
  final IconData icon;
  final Widget child;
  final Widget? trailing;

  const ConfigCard({
    super.key,
    required this.title,
    required this.icon,
    required this.child,
    this.trailing,
  });

  @override
  Widget build(BuildContext context) {
    return Container(
      decoration: BoxDecoration(
        color: AppColors.bgSecondary,
        borderRadius: BorderRadius.circular(AppTheme.radiusMd),
        border: Border.all(color: AppColors.borderColor),
        boxShadow: AppColors.shadowSm,
      ),
      padding: const EdgeInsets.all(12),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Row(
            children: [
              Icon(
                icon,
                size: 14,
                color: AppColors.accent,
              ),
              const SizedBox(width: 8),
              Text(
                title.toUpperCase(),
                style: const TextStyle(
                  fontSize: 11,
                  fontWeight: FontWeight.w600,
                  color: AppColors.textPrimary,
                  letterSpacing: 0.5,
                ),
              ),
              if (trailing != null) ...[
                const Spacer(),
                trailing!,
              ],
            ],
          ),
          const SizedBox(height: 10),
          child,
        ],
      ),
    );
  }
}
```

---

## Verification Checklist

- [ ] AppColors matches Vue CSS variables
- [ ] AppTheme correctly applies fonts and colors
- [ ] CustomDropdown animates smoothly (slide + fade)
- [ ] ToggleSwitch works in both regular and compact modes
- [ ] ConnectButton shows correct gradient for states
- [ ] ConfigCard matches original card styling
- [ ] All widgets respect enabled/disabled states
- [ ] Shadows and borders match original design

---

## Testing

```dart
// Test widget in isolation
void main() {
  runApp(MaterialApp(
    theme: AppTheme.light,
    home: Scaffold(
      body: Center(
        child: Column(
          mainAxisSize: MainAxisSize.min,
          children: [
            CustomDropdown<int>(
              label: 'Baud Rate',
              icon: Icon(Icons.bolt, size: 12),
              value: 115200,
              items: [
                DropdownItem(value: 9600, label: '9.600'),
                DropdownItem(value: 115200, label: '115.200'),
              ],
              onChanged: (v) => print('Selected: $v'),
            ),
            SizedBox(height: 16),
            ToggleSwitch(
              value: true,
              label: 'DTR',
              onChanged: (v) => print('Toggle: $v'),
            ),
            SizedBox(height: 16),
            SizedBox(
              width: 200,
              child: ConnectButton(
                isConnected: false,
                onPressed: () => print('Connect'),
              ),
            ),
          ],
        ),
      ),
    ),
  ));
}
```
