import 'package:flutter/material.dart';
import 'package:google_fonts/google_fonts.dart';
import 'app_colors.dart';

/// App typography - System font for UI, JetBrains Mono for terminal
/// macOS uses SF Pro as system font (accessed via default font family)
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

  // Section title - for sidebar sections
  static TextStyle get sectionTitle => const TextStyle(
        fontSize: 11,
        fontWeight: FontWeight.w600,
        color: AppColors.textSecondary,
        letterSpacing: 0.5,
        height: 1.2,
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
