# Phase 5: Polish & Testing

**Status**: Pending
**Priority**: Medium
**Dependencies**: Phase 4 complete

---

## Objective

Finalize styling, ensure consistency, và comprehensive testing.

---

## Tasks

### 5.1 Spacing Audit

**Ensure consistent spacing throughout**:

| Location | Horizontal | Vertical |
|----------|------------|----------|
| Sidebar padding | 16px | 16px |
| Card padding | 16px | 16px |
| Card gaps | - | 16px |
| Element gaps | 8-12px | 8-12px |
| Terminal margin | 16px | 16px |
| Send panel padding | 16px | 12px |

**Action**: Review all widgets and ensure consistent use of spacing values from the 4px base system.

---

### 5.2 Color Consistency Check

**Verify usage of AppColors throughout**:

- [ ] All text uses `AppColors.textPrimary/Secondary/Tertiary`
- [ ] All backgrounds use `AppColors.background/surface/surfaceVariant`
- [ ] All borders use `AppColors.border/divider`
- [ ] Status colors use `AppColors.success/error/warning`
- [ ] No hardcoded color values

---

### 5.3 Typography Consistency Check

**Verify usage of AppTypography throughout**:

- [ ] Section headers use `labelLarge`
- [ ] Labels use `labelSmall`
- [ ] Body text uses `bodyMedium` or `bodySmall`
- [ ] Terminal text uses `terminal`
- [ ] Badges use `badge`
- [ ] No inline TextStyle definitions

---

### 5.4 Icon Consistency Check

**All icons should be CupertinoIcons**:

| Component | Icon |
|-----------|------|
| Terminal | `CupertinoIcons.terminal` |
| Settings | `CupertinoIcons.gear` |
| Port/USB | `CupertinoIcons.device_phone_portrait` |
| Refresh | `CupertinoIcons.refresh` |
| Connect | `CupertinoIcons.link` |
| Disconnect | `CupertinoIcons.link_badge_minus` |
| Signal | `CupertinoIcons.antenna_radiowaves_left_right` |
| Display | `CupertinoIcons.display` |
| Timer | `CupertinoIcons.timer` |
| Send | `CupertinoIcons.paperplane_fill` |
| Clear/Trash | `CupertinoIcons.trash` |
| Help | `CupertinoIcons.question_circle` |
| Add | `CupertinoIcons.plus` |
| Remove | `CupertinoIcons.minus` |

---

### 5.5 Hover States Verification

**Ensure proper hover feedback**:

- [ ] All buttons have hover state
- [ ] Dropdown items have hover highlight
- [ ] Icon buttons have hover background
- [ ] No scale transforms on hover

---

### 5.6 Keyboard Navigation

**Verify keyboard accessibility**:

- [ ] Tab through all interactive elements
- [ ] Enter activates buttons
- [ ] Escape closes dropdowns
- [ ] Focus rings visible

---

### 5.7 Light/Dark Mode Testing

**Test both theme modes**:

- [ ] App respects system theme setting
- [ ] All colors adapt properly in dark mode
- [ ] Text remains readable in both modes
- [ ] Shadows visible but subtle in dark mode

---

### 5.8 Window Behavior Testing

**Verify macOS window behavior**:

- [ ] Transparent titlebar working
- [ ] Traffic lights positioned correctly
- [ ] Sidebar resizable (if enabled)
- [ ] Window resizing smooth
- [ ] Minimum window size respected

---

### 5.9 Functional Testing Checklist

| Feature | Status |
|---------|--------|
| List serial ports | |
| Select port | |
| Connect to port | |
| Disconnect from port | |
| Change baud rate | |
| Change data bits | |
| Change stop bits | |
| Change parity | |
| Toggle DTR | |
| Toggle RTS | |
| Switch display mode (Text/Hex/Chart) | |
| Toggle auto scroll | |
| Change line ending | |
| Send text message | |
| Send hex message | |
| Receive data | |
| Clear terminal | |
| Auto send (if implemented) | |

---

### 5.10 Performance Testing

**Check for performance issues**:

1. **Startup time**:
   - App should start trong < 2 giây

2. **Data reception**:
   - No frame drops khi receive 1000+ messages/giây
   - Terminal scrolling smooth

3. **Memory usage**:
   - Stable memory footprint
   - No memory leaks sau extended usage

**Profile command**:
```bash
flutter run -d macos --profile
# Open DevTools và check Performance tab
```

---

### 5.11 Code Cleanup

**Final cleanup tasks**:

1. **Remove unused imports**:
   ```bash
   flutter analyze
   ```

2. **Remove unused files**:
   - `lib/widgets/common/custom_dropdown.dart` (if not used)
   - `lib/theme/app_theme.dart` (if replaced by MacosThemeData)

3. **Format code**:
   ```bash
   dart format lib/
   ```

4. **Run linter**:
   ```bash
   flutter analyze
   ```

---

### 5.12 Documentation Update

**Update project documentation**:

1. **README.md** (if exists):
   - Update screenshots
   - Note macOS-native UI

2. **CHANGELOG.md** (if exists):
   - Document UI refactor

---

## Final Verification

### Build Test
```bash
cd termimax_flutter
flutter clean
flutter pub get
flutter build macos --release
```

### Run Test
```bash
# Development
flutter run -d macos

# Release
open build/macos/Build/Products/Release/termimax_flutter.app
```

---

## Rollback Plan

If critical issues found:

1. Git revert to pre-Phase 1 commit
2. Document issues found
3. Plan fixes before retry

---

## Success Criteria Checklist

- [ ] App looks native macOS
- [ ] All existing functionality works
- [ ] No performance regression
- [ ] Code is clean và maintainable
- [ ] Passes flutter analyze
- [ ] Light và dark mode work
- [ ] Window behavior correct

---

## Post-Implementation

After successful completion:

1. **Commit changes**:
   ```bash
   git add .
   git commit -m "refactor: migrate UI to macos_ui for native macOS look"
   ```

2. **Create release build**:
   ```bash
   flutter build macos --release
   ```

3. **Test release build** trên different macOS versions (nếu có access)

---

*Phase 5 Complete = Project Complete*
