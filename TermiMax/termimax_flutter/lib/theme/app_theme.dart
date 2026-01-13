import 'package:flutter/material.dart';
import 'package:google_fonts/google_fonts.dart';
import 'app_colors.dart';
import 'app_typography.dart';

/// Main app theme configuration
class AppTheme {
  AppTheme._();

  static ThemeData get light => ThemeData(
        useMaterial3: true,
        brightness: Brightness.light,

        // Color scheme
        colorScheme: ColorScheme.light(
          primary: AppColors.primary,
          onPrimary: AppColors.textOnPrimary,
          primaryContainer: AppColors.primarySurface,
          onPrimaryContainer: AppColors.primaryDark,
          secondary: AppColors.primaryLight,
          onSecondary: AppColors.textOnPrimary,
          surface: AppColors.surface,
          onSurface: AppColors.textPrimary,
          error: AppColors.error,
          onError: AppColors.textOnPrimary,
          outline: AppColors.border,
          outlineVariant: AppColors.borderLight,
        ),

        // Scaffold
        scaffoldBackgroundColor: AppColors.background,

        // AppBar
        appBarTheme: AppBarTheme(
          elevation: 0,
          scrolledUnderElevation: 1,
          backgroundColor: AppColors.surface,
          foregroundColor: AppColors.textPrimary,
          surfaceTintColor: Colors.transparent,
          titleTextStyle: AppTypography.h4,
          iconTheme: const IconThemeData(
            color: AppColors.textSecondary,
            size: 22,
          ),
        ),

        // Card
        cardTheme: CardThemeData(
          elevation: 0,
          color: AppColors.cardBackground,
          surfaceTintColor: Colors.transparent,
          shape: RoundedRectangleBorder(
            borderRadius: BorderRadius.circular(12),
            side: const BorderSide(color: AppColors.border, width: 1),
          ),
          margin: EdgeInsets.zero,
        ),

        // Divider
        dividerTheme: const DividerThemeData(
          color: AppColors.divider,
          thickness: 1,
          space: 1,
        ),

        // Text theme
        textTheme: TextTheme(
          displayLarge: AppTypography.h1,
          displayMedium: AppTypography.h2,
          displaySmall: AppTypography.h3,
          headlineLarge: AppTypography.h1,
          headlineMedium: AppTypography.h2,
          headlineSmall: AppTypography.h3,
          titleLarge: AppTypography.h4,
          titleMedium: AppTypography.labelLarge,
          titleSmall: AppTypography.labelMedium,
          bodyLarge: AppTypography.bodyLarge,
          bodyMedium: AppTypography.bodyMedium,
          bodySmall: AppTypography.bodySmall,
          labelLarge: AppTypography.labelLarge,
          labelMedium: AppTypography.labelMedium,
          labelSmall: AppTypography.labelSmall,
        ),

        // Input decoration
        inputDecorationTheme: InputDecorationTheme(
          filled: true,
          fillColor: AppColors.surface,
          contentPadding: const EdgeInsets.symmetric(
            horizontal: 16,
            vertical: 12,
          ),
          border: OutlineInputBorder(
            borderRadius: BorderRadius.circular(8),
            borderSide: const BorderSide(color: AppColors.border),
          ),
          enabledBorder: OutlineInputBorder(
            borderRadius: BorderRadius.circular(8),
            borderSide: const BorderSide(color: AppColors.border),
          ),
          focusedBorder: OutlineInputBorder(
            borderRadius: BorderRadius.circular(8),
            borderSide: const BorderSide(color: AppColors.primary, width: 2),
          ),
          errorBorder: OutlineInputBorder(
            borderRadius: BorderRadius.circular(8),
            borderSide: const BorderSide(color: AppColors.error),
          ),
          hintStyle: AppTypography.inputHint,
          labelStyle: AppTypography.labelMedium,
        ),

        // Elevated button
        elevatedButtonTheme: ElevatedButtonThemeData(
          style: ElevatedButton.styleFrom(
            elevation: 0,
            backgroundColor: AppColors.primary,
            foregroundColor: AppColors.textOnPrimary,
            disabledBackgroundColor: AppColors.border,
            disabledForegroundColor: AppColors.textTertiary,
            padding: const EdgeInsets.symmetric(horizontal: 20, vertical: 12),
            shape: RoundedRectangleBorder(
              borderRadius: BorderRadius.circular(8),
            ),
            textStyle: AppTypography.button,
          ),
        ),

        // Outlined button
        outlinedButtonTheme: OutlinedButtonThemeData(
          style: OutlinedButton.styleFrom(
            foregroundColor: AppColors.primary,
            side: const BorderSide(color: AppColors.primary),
            padding: const EdgeInsets.symmetric(horizontal: 20, vertical: 12),
            shape: RoundedRectangleBorder(
              borderRadius: BorderRadius.circular(8),
            ),
            textStyle: AppTypography.button.copyWith(color: AppColors.primary),
          ),
        ),

        // Text button
        textButtonTheme: TextButtonThemeData(
          style: TextButton.styleFrom(
            foregroundColor: AppColors.primary,
            padding: const EdgeInsets.symmetric(horizontal: 16, vertical: 8),
            shape: RoundedRectangleBorder(
              borderRadius: BorderRadius.circular(8),
            ),
            textStyle: AppTypography.button.copyWith(color: AppColors.primary),
          ),
        ),

        // Icon button
        iconButtonTheme: IconButtonThemeData(
          style: IconButton.styleFrom(
            foregroundColor: AppColors.textSecondary,
            hoverColor: AppColors.hoverOverlay,
            shape: RoundedRectangleBorder(
              borderRadius: BorderRadius.circular(8),
            ),
          ),
        ),

        // Chip
        chipTheme: ChipThemeData(
          backgroundColor: AppColors.surfaceVariant,
          selectedColor: AppColors.primarySurface,
          labelStyle: AppTypography.labelSmall,
          padding: const EdgeInsets.symmetric(horizontal: 8, vertical: 4),
          shape: RoundedRectangleBorder(
            borderRadius: BorderRadius.circular(6),
          ),
          side: BorderSide.none,
        ),

        // Switch
        switchTheme: SwitchThemeData(
          thumbColor: WidgetStateProperty.resolveWith((states) {
            if (states.contains(WidgetState.selected)) {
              return AppColors.textOnPrimary;
            }
            return AppColors.textTertiary;
          }),
          trackColor: WidgetStateProperty.resolveWith((states) {
            if (states.contains(WidgetState.selected)) {
              return AppColors.primary;
            }
            return AppColors.border;
          }),
          trackOutlineColor: WidgetStateProperty.all(Colors.transparent),
        ),

        // Checkbox
        checkboxTheme: CheckboxThemeData(
          fillColor: WidgetStateProperty.resolveWith((states) {
            if (states.contains(WidgetState.selected)) {
              return AppColors.primary;
            }
            return Colors.transparent;
          }),
          checkColor: WidgetStateProperty.all(AppColors.textOnPrimary),
          side: const BorderSide(color: AppColors.border, width: 2),
          shape: RoundedRectangleBorder(
            borderRadius: BorderRadius.circular(4),
          ),
        ),

        // Dropdown
        dropdownMenuTheme: DropdownMenuThemeData(
          inputDecorationTheme: InputDecorationTheme(
            filled: true,
            fillColor: AppColors.surface,
            contentPadding: const EdgeInsets.symmetric(
              horizontal: 16,
              vertical: 12,
            ),
            border: OutlineInputBorder(
              borderRadius: BorderRadius.circular(8),
              borderSide: const BorderSide(color: AppColors.border),
            ),
          ),
          menuStyle: MenuStyle(
            backgroundColor: WidgetStateProperty.all(AppColors.surface),
            elevation: WidgetStateProperty.all(4),
            shape: WidgetStateProperty.all(
              RoundedRectangleBorder(
                borderRadius: BorderRadius.circular(8),
                side: const BorderSide(color: AppColors.border),
              ),
            ),
          ),
        ),

        // Tooltip
        tooltipTheme: TooltipThemeData(
          decoration: BoxDecoration(
            color: AppColors.textPrimary,
            borderRadius: BorderRadius.circular(6),
          ),
          textStyle: AppTypography.bodySmall.copyWith(
            color: AppColors.textOnPrimary,
          ),
          padding: const EdgeInsets.symmetric(horizontal: 12, vertical: 8),
        ),

        // Snackbar
        snackBarTheme: SnackBarThemeData(
          backgroundColor: AppColors.textPrimary,
          contentTextStyle: AppTypography.bodyMedium.copyWith(
            color: AppColors.textOnPrimary,
          ),
          shape: RoundedRectangleBorder(
            borderRadius: BorderRadius.circular(8),
          ),
          behavior: SnackBarBehavior.floating,
        ),

        // Progress indicator
        progressIndicatorTheme: const ProgressIndicatorThemeData(
          color: AppColors.primary,
          linearTrackColor: AppColors.primarySurface,
          circularTrackColor: AppColors.primarySurface,
        ),

        // Scrollbar
        scrollbarTheme: ScrollbarThemeData(
          thumbColor: WidgetStateProperty.all(AppColors.border),
          radius: const Radius.circular(4),
          thickness: WidgetStateProperty.all(6),
        ),
      );
}
