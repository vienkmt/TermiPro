import 'package:flutter/material.dart';

/// Ultra Minimal color palette
class AppColors {
  AppColors._();

  // Primary - muted blue
  static const Color primary = Color(0xFF3B82F6);
  static const Color primaryMuted = Color(0xFF93C5FD);
  static const Color primarySurface = Color(0xFFEFF6FF);
  static const Color primaryLight = Color(0xFFBFDBFE);
  static const Color primaryDark = Color(0xFF1D4ED8);

  // Backgrounds - clean whites/grays
  static const Color background = Color(0xFFFFFFFF);
  static const Color surface = Color(0xFFFFFFFF);
  static const Color surfaceVariant = Color(0xFFF8FAFC);
  static const Color sidebarBackground = Color(0xFFEFF1F5);
  static const Color sidebarBorder = Color(0xFFE2E8F0);
  static const Color cardBackground = Color(0xFFFFFFFF);

  // Text on primary
  static const Color textOnPrimary = Color(0xFFFFFFFF);

  // Text - high contrast
  static const Color textPrimary = Color(0xFF0F172A);
  static const Color textSecondary = Color(0xFF64748B);
  static const Color textTertiary = Color(0xFF94A3B8);
  static const Color textMuted = Color(0xFFCBD5E1);

  // Borders - very subtle
  static const Color border = Color(0xFFE2E8F0);
  static const Color borderLight = Color(0xFFF1F5F9);
  static const Color divider = Color(0xFFF1F5F9);

  // Status
  static const Color success = Color(0xFF22C55E);
  static const Color successMuted = Color(0xFF86EFAC);
  static const Color successLight = Color(0xFFDCFCE7);
  static const Color successDark = Color(0xFF166534);
  static const Color error = Color(0xFFEF4444);
  static const Color warning = Color(0xFFF59E0B);

  // TX/RX - subtle
  static const Color txText = Color(0xFFD97706);
  static const Color rxText = Color(0xFF16A34A);

  // Interactions
  static const Color hover = Color(0xFFF8FAFC);
  static const Color active = Color(0xFFF1F5F9);
  static const Color hoverOverlay = Color(0x0A000000);

  // Subtle shadows
  static List<BoxShadow> get subtleShadow => [
    BoxShadow(
      color: Colors.black.withOpacity(0.04),
      blurRadius: 4,
      offset: const Offset(0, 1),
    ),
  ];
  static List<BoxShadow> get cardShadow => [
    BoxShadow(
      color: Colors.black.withOpacity(0.06),
      blurRadius: 8,
      offset: const Offset(0, 2),
    ),
  ];
}
