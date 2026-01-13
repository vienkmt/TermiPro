# Phase 1: Foundation Setup

**Status**: Pending
**Priority**: High
**Dependencies**: None

---

## Objective

Setup cơ sở cho việc chuyển từ Material sang macos_ui, bao gồm dependencies, entry point, và theme system.

---

## Tasks

### 1.1 Update pubspec.yaml

**File**: `pubspec.yaml`

**Changes**:
```yaml
dependencies:
  flutter:
    sdk: flutter

  # NEW: macOS UI packages
  macos_ui: ^2.2.0
  macos_window_utils: ^1.2.0

  # KEEP: Existing dependencies
  cupertino_icons: ^1.0.8
  termipro_rust:
    path: rust_builder
  flutter_rust_bridge: 2.11.1
  google_fonts: ^7.0.1  # Keep for JetBrains Mono terminal font
```

**Actions**:
1. Add `macos_ui: ^2.2.0`
2. Add `macos_window_utils: ^1.2.0`
3. Run `flutter pub get`
4. Verify no dependency conflicts

**Verification**:
```bash
cd termimax_flutter
flutter pub get
flutter pub outdated
```

---

### 1.2 Update main.dart

**File**: `lib/main.dart`

**Current Code**:
```dart
import 'package:flutter/material.dart';
import 'package:termimax_flutter/src/rust/frb_generated.dart';
import 'package:termimax_flutter/theme/theme.dart';
import 'package:termimax_flutter/screens/serial_screen.dart';

Future<void> main() async {
  WidgetsFlutterBinding.ensureInitialized();
  await RustLib.init();
  runApp(const MyApp());
}

class MyApp extends StatelessWidget {
  const MyApp({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'TermiMax',
      debugShowCheckedModeBanner: false,
      theme: AppTheme.light,
      home: const SerialScreen(),
    );
  }
}
```

**New Code**:
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

**Key Changes**:
1. `StatelessWidget` → `StatefulWidget` (for window setup)
2. `MaterialApp` → `MacosApp`
3. Add `_setupWindow()` for transparent titlebar
4. Support light/dark theme modes

---

### 1.3 Update app_colors.dart

**File**: `lib/theme/app_colors.dart`

**Updated Code**:
```dart
import 'package:flutter/material.dart';

/// App color constants - macOS native style
/// Aligned with Apple HIG color palette
class AppColors {
  AppColors._();

  // Primary - Sky Blue (softer)
  static const Color primary = Color(0xFF0A84FF);      // Apple Blue
  static const Color primaryLight = Color(0xFF5AC8FA); // Lighter variant
  static const Color primaryDark = Color(0xFF0071E3);  // Darker variant
  static const Color primarySurface = Color(0xFFE1F0FF);

  // Background & Surface (Apple Off-white)
  static const Color background = Color(0xFFF5F5F7);
  static const Color surface = Color(0xFFFFFFFF);
  static const Color surfaceVariant = Color(0xFFF0F0F2);
  static const Color cardBackground = Color(0xFFFFFFFF);

  // Text (Apple standard)
  static const Color textPrimary = Color(0xFF1D1D1F);
  static const Color textSecondary = Color(0xFF6E6E73);
  static const Color textTertiary = Color(0xFF8E8E93);
  static const Color textOnPrimary = Color(0xFFFFFFFF);

  // Border & Divider (softer)
  static const Color border = Color(0xFFE5E5EA);
  static const Color borderLight = Color(0xFFF0F0F2);
  static const Color divider = Color(0xFFE5E5EA);

  // Status - Success (Apple Green)
  static const Color success = Color(0xFF34C759);
  static const Color successLight = Color(0xFFD4EDDA);
  static const Color successDark = Color(0xFF28A745);

  // Status - Error (Apple Red)
  static const Color error = Color(0xFFFF3B30);
  static const Color errorLight = Color(0xFFF8D7DA);
  static const Color errorDark = Color(0xFFDC3545);

  // Status - Warning (Apple Orange)
  static const Color warning = Color(0xFFFF9500);
  static const Color warningLight = Color(0xFFFFF3CD);
  static const Color warningDark = Color(0xFFE68A00);

  // Terminal
  static const Color terminalBackground = Color(0xFFFAFAFC);
  static const Color terminalBorder = Color(0xFFE5E5EA);

  // TX/RX Badges
  static const Color txBadge = Color(0xFFFFF3CD);
  static const Color txText = Color(0xFF856404);
  static const Color rxBadge = Color(0xFFD4EDDA);
  static const Color rxText = Color(0xFF155724);

  // Connection status
  static const Color connected = Color(0xFF34C759);
  static const Color disconnected = Color(0xFF8E8E93);

  // Hover & Focus
  static const Color hoverOverlay = Color(0x08000000);
  static const Color focusRing = Color(0xFF0A84FF);

  // Shadows - macOS style (subtle)
  static const Color shadowLight = Color(0x08000000);
  static const Color shadowMedium = Color(0x10000000);

  // Helper for subtle shadows
  static List<BoxShadow> get subtleShadow => [
    BoxShadow(
      color: Colors.black.withOpacity(0.04),
      blurRadius: 8,
      offset: const Offset(0, 2),
    ),
  ];

  static List<BoxShadow> get cardShadow => [
    BoxShadow(
      color: Colors.black.withOpacity(0.06),
      blurRadius: 12,
      offset: const Offset(0, 4),
    ),
  ];
}
```

---

### 1.4 Update app_typography.dart

**File**: `lib/theme/app_typography.dart`

**Updated Code**:
```dart
import 'package:flutter/material.dart';
import 'package:google_fonts/google_fonts.dart';
import 'app_colors.dart';

/// App typography - System font for UI, JetBrains Mono for terminal
/// macOS uses SF Pro as system font (accessed via system font family)
class AppTypography {
  AppTypography._();

  // Terminal font family
  static String get monoFontFamily => GoogleFonts.jetBrainsMono().fontFamily!;

  // Heading styles - Use system font (SF Pro on macOS)
  static TextStyle get h1 => const TextStyle(
        fontSize: 28,
        fontWeight: FontWeight.w700,
        color: AppColors.textPrimary,
        letterSpacing: -0.5,
        height: 1.2,
      );

  static TextStyle get h2 => const TextStyle(
        fontSize: 22,
        fontWeight: FontWeight.w600,
        color: AppColors.textPrimary,
        letterSpacing: -0.3,
        height: 1.3,
      );

  static TextStyle get h3 => const TextStyle(
        fontSize: 17,
        fontWeight: FontWeight.w600,
        color: AppColors.textPrimary,
        height: 1.35,
      );

  static TextStyle get h4 => const TextStyle(
        fontSize: 15,
        fontWeight: FontWeight.w600,
        color: AppColors.textPrimary,
        height: 1.4,
      );

  // Body styles
  static TextStyle get bodyLarge => const TextStyle(
        fontSize: 15,
        fontWeight: FontWeight.w400,
        color: AppColors.textPrimary,
        height: 1.5,
      );

  static TextStyle get bodyMedium => const TextStyle(
        fontSize: 13,
        fontWeight: FontWeight.w400,
        color: AppColors.textPrimary,
        height: 1.5,
      );

  static TextStyle get bodySmall => const TextStyle(
        fontSize: 11,
        fontWeight: FontWeight.w400,
        color: AppColors.textSecondary,
        height: 1.5,
      );

  // Label styles
  static TextStyle get labelLarge => const TextStyle(
        fontSize: 13,
        fontWeight: FontWeight.w600,
        color: AppColors.textPrimary,
        height: 1.4,
      );

  static TextStyle get labelMedium => const TextStyle(
        fontSize: 12,
        fontWeight: FontWeight.w500,
        color: AppColors.textPrimary,
        height: 1.4,
      );

  static TextStyle get labelSmall => const TextStyle(
        fontSize: 11,
        fontWeight: FontWeight.w500,
        color: AppColors.textSecondary,
        height: 1.4,
        letterSpacing: 0.2,
      );

  // Button text
  static TextStyle get button => const TextStyle(
        fontSize: 13,
        fontWeight: FontWeight.w600,
        color: AppColors.textOnPrimary,
        height: 1.2,
      );

  static TextStyle get buttonSmall => const TextStyle(
        fontSize: 11,
        fontWeight: FontWeight.w600,
        color: AppColors.textOnPrimary,
        height: 1.2,
      );

  // Terminal/Code styles - JetBrains Mono
  static TextStyle get terminal => GoogleFonts.jetBrainsMono(
        fontSize: 12,
        fontWeight: FontWeight.w400,
        color: AppColors.textPrimary,
        height: 1.5,
      );

  static TextStyle get terminalSmall => GoogleFonts.jetBrainsMono(
        fontSize: 11,
        fontWeight: FontWeight.w400,
        color: AppColors.textPrimary,
        height: 1.4,
      );

  static TextStyle get code => GoogleFonts.jetBrainsMono(
        fontSize: 12,
        fontWeight: FontWeight.w400,
        color: AppColors.textPrimary,
        height: 1.5,
      );

  // Input field text
  static TextStyle get input => const TextStyle(
        fontSize: 13,
        fontWeight: FontWeight.w400,
        color: AppColors.textPrimary,
        height: 1.4,
      );

  static TextStyle get inputHint => const TextStyle(
        fontSize: 13,
        fontWeight: FontWeight.w400,
        color: AppColors.textTertiary,
        height: 1.4,
      );

  // Badge text
  static TextStyle get badge => const TextStyle(
        fontSize: 10,
        fontWeight: FontWeight.w700,
        height: 1.2,
        letterSpacing: 0.2,
      );

  // Stats/Counter text
  static TextStyle get stats => GoogleFonts.jetBrainsMono(
        fontSize: 11,
        fontWeight: FontWeight.w600,
        color: AppColors.textSecondary,
        height: 1.2,
      );

  // Timestamp
  static TextStyle get timestamp => GoogleFonts.jetBrainsMono(
        fontSize: 10,
        fontWeight: FontWeight.w400,
        color: AppColors.textTertiary,
        height: 1.2,
      );
}
```

---

### 1.5 Update theme.dart barrel export

**File**: `lib/theme/theme.dart`

**Updated Code**:
```dart
// Barrel file for theme exports
export 'app_colors.dart';
export 'app_typography.dart';
// Note: app_theme.dart removed as we use MacosThemeData directly
```

---

### 1.6 Update macos/Podfile (if needed)

**File**: `macos/Podfile`

**Add/Update**:
```ruby
platform :osx, '10.14.6'

# ... existing code ...

post_install do |installer|
  installer.pods_project.targets.each do |target|
    flutter_additional_macos_build_settings(target)
    target.build_configurations.each do |config|
      config.build_settings['MACOSX_DEPLOYMENT_TARGET'] = '10.14.6'
    end
  end
end
```

---

## Verification Steps

1. **Build Check**:
   ```bash
   cd termimax_flutter
   flutter clean
   flutter pub get
   flutter build macos
   ```

2. **Run Check**:
   ```bash
   flutter run -d macos
   ```

3. **Visual Verification**:
   - App launches với MacosApp
   - Titlebar transparent
   - System theme mode working (light/dark)

---

## Rollback

If issues occur:
1. Revert `main.dart` to use `MaterialApp`
2. Remove macos_ui dependencies
3. Keep theme file changes (compatible với both)

---

## Next Phase

After Phase 1 complete, proceed to **Phase 2: Layout Structure** where we update `serial_screen.dart` to use `MacosWindow` and `Sidebar`.
