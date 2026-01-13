# Plan: Refactor TermiMax UI theo macOS Native Style

**Date**: 2025-01-13
**Status**: Draft
**Complexity**: Hard
**Estimated Files**: 15+

---

## Executive Summary

Refactor toàn bộ giao diện TermiMax Flutter từ Material Design sang **macOS native style** sử dụng **macos_ui** package. Giữ nguyên logic, cấu trúc cơ bản, chỉ thay đổi UI components và styling.

### Key Decision: Sử dụng macos_ui thay vì custom Material

**Lý do:**
1. **Native Look & Feel**: macos_ui tuân thủ Apple HIG (Human Interface Guidelines)
2. **Active Maintenance**: Package stable, đang được maintain tích cực
3. **Component Coverage**: 30+ native widgets (Sidebar, ToolBar, PopupButton, Switch, etc.)
4. **Performance**: Negligible overhead, GPU-accelerated effects
5. **Future-proof**: Hỗ trợ Liquid Glass design trends (2025+)

---

## Current State Analysis

### Existing Structure
```
lib/
├── main.dart                 # MaterialApp entry point
├── screens/
│   └── serial_screen.dart    # Main screen (722 lines)
├── theme/
│   ├── app_colors.dart       # Sky Blue palette
│   ├── app_typography.dart   # Plus Jakarta Sans + JetBrains Mono
│   ├── app_theme.dart        # ThemeData config
│   └── theme.dart            # Barrel export
├── models/
│   └── serial_config.dart    # Config models
└── widgets/
    ├── common/
    │   └── custom_dropdown.dart  # Custom overlay dropdown
    ├── sidebar/
    │   ├── sidebar.dart          # Sidebar container (320px)
    │   ├── port_selector.dart    # Port dropdown + Connect button
    │   ├── config_card.dart      # Baud/Data/Stop/Parity config
    │   ├── signal_toggles.dart   # DTR/RTS toggles
    │   ├── display_options.dart  # Text/Hex/Chart mode
    │   └── auto_send_card.dart   # Auto send settings
    └── chart/
        └── realtime_chart.dart   # Chart component
```

### Current Issues
1. **Material AppBar**: Không phù hợp macOS, cần native TitleBar
2. **Heavy Borders**: Cards có border 1px dày, chưa tinh tế
3. **Material Switch**: Không giống macOS System Preferences toggle
4. **Custom Dropdowns**: Hoạt động tốt nhưng không native
5. **Spacing**: Chưa consistent (16px, 12px mixed)
6. **No Vibrancy**: Thiếu blur/transparency effects

---

## Target Design

### Visual Style
- **Design Language**: macOS HIG + Minimalism Swiss Style
- **Color Scheme**: Softer Sky Blue + macOS System Colors
- **Typography**: SF Pro (via system fonts) + JetBrains Mono (terminal)
- **Spacing**: 4px base unit (4, 8, 12, 16, 20, 24, 32)
- **Corners**: 10-12px radius (macOS standard)
- **Effects**: Subtle shadows, optional vibrancy

### Component Mapping

| Current (Material) | Target (macos_ui) |
|-------------------|-------------------|
| MaterialApp | MacosApp |
| Scaffold + AppBar | MacosWindow + TitleBar |
| Container sidebar | Sidebar widget |
| Custom dropdown | MacosPopupButton |
| Switch | MacosSwitch |
| ElevatedButton | PushButton |
| TextField | MacosTextField |
| Card with border | Container with BoxDecoration (subtle shadow) |

---

## Implementation Phases

### Phase 1: Foundation Setup
**Files**: `pubspec.yaml`, `main.dart`, `theme/`

1. **Add Dependencies**
   ```yaml
   dependencies:
     macos_ui: ^2.2.0
     macos_window_utils: ^1.2.0
   ```

2. **Update main.dart**
   - Replace `MaterialApp` với `MacosApp`
   - Setup `MacosThemeData.light()` / `.dark()`
   - Init window effects trong `initState()`

3. **Refactor Theme Files**
   - `app_colors.dart`: Add macOS system color integration
   - `app_typography.dart`: Use system fonts for headings, keep JetBrains Mono
   - `app_theme.dart`: Create `MacosThemeData` extension

### Phase 2: Layout Structure
**Files**: `serial_screen.dart`, `sidebar/sidebar.dart`

1. **Replace Scaffold with MacosWindow**
   ```dart
   MacosWindow(
     titleBar: TitleBar(
       title: Text('TermiMax'),
       backgroundColor: MacosColors.transparent,
     ),
     sidebar: Sidebar(
       minWidth: 280,
       maxWidth: 320,
       builder: (context, scrollController) => _buildSidebarContent(),
     ),
     children: [_buildMainContent()],
   )
   ```

2. **Window Effects Setup**
   ```dart
   @override
   void initState() {
     super.initState();
     _setupWindowEffects();
   }

   Future<void> _setupWindowEffects() async {
     await WindowManipulator.makeTitlebarTransparent();
     await WindowManipulator.enableFullSizeContentView();
   }
   ```

### Phase 3: Sidebar Components
**Files**: `port_selector.dart`, `config_card.dart`, `signal_toggles.dart`, `display_options.dart`, `auto_send_card.dart`

1. **Port Selector**
   - Replace custom overlay dropdown với `MacosPopupButton`
   - Style Connect/Disconnect button với `PushButton`

2. **Config Card**
   - Replace `CustomDropdown` với `MacosPopupButton`
   - Use subtle shadow thay vì border
   - Apply consistent 16px padding

3. **Signal Toggles**
   - Replace `Switch` với `MacosSwitch`
   - Simplified card style

4. **Display Options**
   - Replace toggle buttons với `MacosSegmentedControl`
   - Use `MacosSwitch` for auto scroll

5. **Auto Send Card**
   - Use `MacosTextField` with number formatter
   - Subtle stepper buttons

### Phase 4: Main Content Area
**Files**: `serial_screen.dart` (terminal section)

1. **Terminal Header**
   - Native styling với subtle separator
   - TX/RX badges với softer colors

2. **Terminal Lines**
   - Cleaner left border indicator
   - Consistent padding
   - JetBrains Mono preserved

3. **Send Panel**
   - `MacosSegmentedControl` for Text/Hex toggle
   - `MacosTextField` for input
   - `PushButton` for send action

### Phase 5: Polish & Effects
**Files**: All components

1. **Apply Consistent Spacing**
   - 16px horizontal padding
   - 12px vertical gaps
   - 8px component spacing

2. **Hover States**
   - Subtle background change
   - No scale transforms
   - 200ms transition duration

3. **Optional: Vibrancy Effects**
   - Sidebar background blur
   - Title bar transparency

---

## File Change Details

### pubspec.yaml
```yaml
dependencies:
  flutter:
    sdk: flutter
  macos_ui: ^2.2.0
  macos_window_utils: ^1.2.0
  google_fonts: ^7.0.1  # Keep for JetBrains Mono
  termipro_rust:
    path: rust_builder
  flutter_rust_bridge: 2.11.1
  cupertino_icons: ^1.0.8

environment:
  sdk: ^3.10.7
  # Note: Cần Flutter 3.35.0+ cho macos_ui 2.2.0+
  # Nếu Flutter < 3.35.0, pin macos_ui ^2.1.0
```

### lib/main.dart (New)
```dart
import 'package:flutter/foundation.dart';
import 'package:macos_ui/macos_ui.dart';
import 'package:macos_window_utils/macos_window_utils.dart';
import 'package:termimax_flutter/src/rust/frb_generated.dart';
import 'package:termimax_flutter/screens/serial_screen.dart';

Future<void> main() async {
  WidgetsFlutterBinding.ensureInitialized();
  await RustLib.init();
  runApp(const MyApp());
}

class MyApp extends StatefulWidget {
  const MyApp({super.key});

  @override
  State<MyApp> createState() => _MyAppState();
}

class _MyAppState extends State<MyApp> {
  @override
  void initState() {
    super.initState();
    _setupWindow();
  }

  Future<void> _setupWindow() async {
    if (!kIsWeb && defaultTargetPlatform == TargetPlatform.macOS) {
      await WindowManipulator.makeTitlebarTransparent();
      await WindowManipulator.enableFullSizeContentView();
    }
  }

  @override
  Widget build(BuildContext context) {
    return MacosApp(
      title: 'TermiMax',
      debugShowCheckedModeBanner: false,
      theme: MacosThemeData.light(),
      darkTheme: MacosThemeData.dark(),
      themeMode: ThemeMode.system,
      home: const SerialScreen(),
    );
  }
}
```

### lib/theme/app_colors.dart (Updated)
```dart
import 'package:flutter/material.dart';
import 'package:macos_ui/macos_ui.dart';

class AppColors {
  AppColors._();

  // Primary - System Accent (respects user preference)
  static Color get primary => MacosColors.controlAccentColor;
  static const Color primaryFixed = Color(0xFF0EA5E9); // Fallback Sky Blue

  // Backgrounds (macOS style)
  static const Color background = Color(0xFFF5F5F7);  // Apple Off-white
  static const Color surface = Color(0xFFFFFFFF);
  static const Color surfaceVariant = Color(0xFFF0F0F2);

  // Text
  static const Color textPrimary = Color(0xFF1D1D1F);   // Apple Black
  static const Color textSecondary = Color(0xFF6E6E73); // Apple Gray
  static const Color textTertiary = Color(0xFF8E8E93);
  static const Color textOnPrimary = Color(0xFFFFFFFF);

  // Borders & Dividers (softer)
  static const Color border = Color(0xFFE5E5EA);
  static const Color divider = Color(0xFFE5E5EA);

  // Status colors (keep existing, slightly softer)
  static const Color success = Color(0xFF34C759);  // Apple Green
  static const Color error = Color(0xFFFF3B30);    // Apple Red
  static const Color warning = Color(0xFFFF9500);  // Apple Orange

  // Terminal
  static const Color terminalBackground = Color(0xFFFAFAFC);
  static const Color terminalBorder = Color(0xFFE5E5EA);

  // TX/RX
  static const Color txBadge = Color(0xFFFFF3CD);
  static const Color txText = Color(0xFF856404);
  static const Color rxBadge = Color(0xFFD4EDDA);
  static const Color rxText = Color(0xFF155724);

  // Shadows
  static List<BoxShadow> get subtleShadow => [
    BoxShadow(
      color: Colors.black.withOpacity(0.04),
      blurRadius: 8,
      offset: const Offset(0, 2),
    ),
  ];
}
```

### lib/theme/app_typography.dart (Updated)
```dart
import 'package:flutter/material.dart';
import 'package:google_fonts/google_fonts.dart';
import 'package:macos_ui/macos_ui.dart';
import 'app_colors.dart';

class AppTypography {
  AppTypography._();

  // System font for headings (SF Pro via system)
  static const String _systemFont = '.SF Pro Text';

  // Terminal font
  static String get monoFontFamily => GoogleFonts.jetBrainsMono().fontFamily!;

  // Headings (use system font)
  static TextStyle get h1 => const TextStyle(
    fontFamily: _systemFont,
    fontSize: 28,
    fontWeight: FontWeight.w700,
    color: AppColors.textPrimary,
    letterSpacing: -0.5,
  );

  static TextStyle get h2 => const TextStyle(
    fontFamily: _systemFont,
    fontSize: 22,
    fontWeight: FontWeight.w600,
    color: AppColors.textPrimary,
    letterSpacing: -0.3,
  );

  static TextStyle get h3 => const TextStyle(
    fontFamily: _systemFont,
    fontSize: 17,
    fontWeight: FontWeight.w600,
    color: AppColors.textPrimary,
  );

  static TextStyle get h4 => const TextStyle(
    fontFamily: _systemFont,
    fontSize: 15,
    fontWeight: FontWeight.w600,
    color: AppColors.textPrimary,
  );

  // Body
  static TextStyle get bodyLarge => const TextStyle(
    fontFamily: _systemFont,
    fontSize: 15,
    fontWeight: FontWeight.w400,
    color: AppColors.textPrimary,
  );

  static TextStyle get bodyMedium => const TextStyle(
    fontFamily: _systemFont,
    fontSize: 13,
    fontWeight: FontWeight.w400,
    color: AppColors.textPrimary,
  );

  static TextStyle get bodySmall => const TextStyle(
    fontFamily: _systemFont,
    fontSize: 11,
    fontWeight: FontWeight.w400,
    color: AppColors.textSecondary,
  );

  // Labels
  static TextStyle get labelLarge => const TextStyle(
    fontFamily: _systemFont,
    fontSize: 13,
    fontWeight: FontWeight.w600,
    color: AppColors.textPrimary,
  );

  static TextStyle get labelSmall => const TextStyle(
    fontFamily: _systemFont,
    fontSize: 11,
    fontWeight: FontWeight.w500,
    color: AppColors.textSecondary,
    letterSpacing: 0.2,
  );

  // Terminal - Keep JetBrains Mono
  static TextStyle get terminal => GoogleFonts.jetBrainsMono(
    fontSize: 12,
    fontWeight: FontWeight.w400,
    color: AppColors.textPrimary,
    height: 1.5,
  );

  static TextStyle get timestamp => GoogleFonts.jetBrainsMono(
    fontSize: 10,
    fontWeight: FontWeight.w400,
    color: AppColors.textTertiary,
  );

  // Badge
  static TextStyle get badge => const TextStyle(
    fontFamily: _systemFont,
    fontSize: 10,
    fontWeight: FontWeight.w700,
    letterSpacing: 0.2,
  );
}
```

---

## Migration Strategy

### Approach: Gradual Replacement
1. Thêm dependencies mới song song với existing
2. Refactor từng component một
3. Test mỗi phase trước khi tiếp tục
4. Giữ nguyên logic/state management

### Testing Checkpoints
- [ ] Phase 1: App khởi động với MacosApp
- [ ] Phase 2: Layout hiển thị đúng với MacosWindow
- [ ] Phase 3: Sidebar components hoạt động
- [ ] Phase 4: Terminal hoạt động bình thường
- [ ] Phase 5: Kết nối serial vẫn work

### Rollback Plan
- Git commit sau mỗi phase
- Có thể rollback về Material nếu có vấn đề

---

## Risks & Mitigations

| Risk | Likelihood | Impact | Mitigation |
|------|------------|--------|------------|
| Flutter version incompatible | Medium | High | Pin macos_ui ^2.1.0 nếu < 3.35.0 |
| State management conflicts | Low | Medium | Không thay đổi state logic |
| Performance regression | Low | Low | Profile với DevTools sau mỗi phase |
| Missing features in macos_ui | Medium | Medium | Fallback to custom widgets nếu cần |

---

## Dependencies

### Required Packages
```yaml
macos_ui: ^2.2.0          # Core macOS UI widgets
macos_window_utils: ^1.2.0 # Window customization
```

### Optional Packages
```yaml
flutter_acrylic: ^1.1.4    # Advanced blur effects (if needed)
```

### Removed Dependencies
- Không cần remove, google_fonts vẫn dùng cho JetBrains Mono

---

## Success Criteria

1. **Visual Match**: UI trông native macOS, tuân thủ HIG
2. **Functionality**: Tất cả features hoạt động như cũ
3. **Performance**: Không có frame drops đáng kể
4. **Code Quality**: Clean, maintainable code
5. **Consistency**: Spacing và styling đồng nhất

---

## Appendix: Component Reference

### macos_ui Widgets Used

| Widget | Purpose |
|--------|---------|
| MacosApp | App entry point |
| MacosWindow | Window container với titlebar |
| TitleBar | Native title bar |
| Sidebar | Collapsible sidebar |
| MacosScaffold | Content layout |
| ContentArea | Main content region |
| MacosPopupButton | Dropdown menus |
| MacosSwitch | Toggle switches |
| MacosTextField | Text input |
| PushButton | Action buttons |
| MacosSegmentedControl | Tab/mode selection |
| MacosIcon | SF Symbols icons |

---

## Next Steps

1. **Review & Approve Plan** - Xác nhận approach
2. **Execute Phase 1** - Foundation setup
3. **Execute Phase 2** - Layout structure
4. **Execute Phase 3** - Sidebar components
5. **Execute Phase 4** - Main content
6. **Execute Phase 5** - Polish & testing
7. **Final Review** - QA và adjustments

---

*Plan created: 2025-01-13*
*Author: Claude (with ui-ux-pro-max & planning skills)*
