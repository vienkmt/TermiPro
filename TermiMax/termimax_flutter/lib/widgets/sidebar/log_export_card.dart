import 'package:flutter/material.dart';
import 'package:termimax_flutter/models/log_export_settings.dart';
import 'package:termimax_flutter/theme/theme.dart';
import 'package:macos_ui/macos_ui.dart' show MacosSwitch;

class LogExportCard extends StatelessWidget {
  final LogExportSettings settings;
  final ValueChanged<bool> onChanged;
  final VoidCallback? onOpen;
  final VoidCallback? onExportExcel;
  final bool isConnected;

  const LogExportCard({
    super.key,
    required this.settings,
    required this.onChanged,
    this.onOpen,
    this.onExportExcel,
    this.isConnected = false,
  });

  @override
  Widget build(BuildContext context) {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        Row(
          children: [
            Text('Export Logs', style: AppTypography.sectionTitle),
            const Spacer(),
            SizedBox(
              height: 20,
              child: MacosSwitch(
                value: settings.enabled,
                onChanged: onChanged,
              ),
            ),
          ],
        ),
        if (settings.enabled && settings.filePath != null) ...[
          const SizedBox(height: 12),
          Container(
            padding: const EdgeInsets.all(8),
            decoration: BoxDecoration(
              color: AppColors.background,
              borderRadius: BorderRadius.circular(4),
              border: Border.all(color: AppColors.border),
            ),
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                Row(
                  children: [
                    Icon(Icons.description_outlined, 
                      size: 14, 
                      color: AppColors.textSecondary
                    ),
                    const SizedBox(width: 6),
                    Expanded(
                      child: Text(
                        _getFileName(settings.filePath!),
                        style: TextStyle(
                          fontSize: 11,
                          color: AppColors.textPrimary,
                          fontWeight: FontWeight.w500,
                        ),
                        maxLines: 1,
                        overflow: TextOverflow.ellipsis,
                      ),
                    ),
                  ],
                ),
                const SizedBox(height: 4),
                Padding(
                  padding: const EdgeInsets.only(left: 20),
                  child: Row(
                    children: [
                      Text(
                        _formatFileSize(settings.fileSize),
                        style: TextStyle(
                          fontSize: 10, 
                          color: AppColors.textTertiary
                        ),
                      ),
                      const SizedBox(width: 8),
                      // Small, subtle open button
                      MouseRegion(
                        cursor: SystemMouseCursors.click,
                        child: GestureDetector(
                          onTap: onOpen,
                          child: Container(
                            padding: const EdgeInsets.symmetric(horizontal: 6, vertical: 2),
                            decoration: BoxDecoration(
                              color: AppColors.surfaceVariant, // or a subtle grey
                              borderRadius: BorderRadius.circular(3),
                              border: Border.all(color: AppColors.border, width: 0.5),
                            ),
                            child: Row(
                              mainAxisSize: MainAxisSize.min,
                              children: [
                                Icon(Icons.open_in_new, size: 8, color: AppColors.textSecondary),
                                const SizedBox(width: 4),
                                Text(
                                  'Open',
                                  style: TextStyle(fontSize: 9, color: AppColors.textSecondary),
                                ),
                              ],
                            ),
                          ),
                        ),
                      ),
                      const SizedBox(width: 6),
                      // Export Excel button
                       Opacity(
                        opacity: isConnected ? 0.5 : 1.0,
                        child: MouseRegion(
                          cursor: isConnected ? SystemMouseCursors.forbidden : SystemMouseCursors.click,
                          child: GestureDetector(
                            onTap: isConnected ? null : onExportExcel,
                            child: Container(
                              padding: const EdgeInsets.symmetric(horizontal: 6, vertical: 2),
                              decoration: BoxDecoration(
                                color: AppColors.success.withOpacity(0.1),
                                borderRadius: BorderRadius.circular(3),
                                border: Border.all(color: AppColors.success.withOpacity(0.3), width: 0.5),
                              ),
                              child: Row(
                                mainAxisSize: MainAxisSize.min,
                                children: [
                                  Icon(Icons.table_chart_outlined, size: 8, color: AppColors.success),
                                  const SizedBox(width: 4),
                                  Text(
                                    'To Excel',
                                    style: TextStyle(fontSize: 9, color: AppColors.success),
                                  ),
                                ],
                              ),
                            ),
                          ),
                        ),
                      ),
                    ],
                  ),
                ),
              ],
            ),
          ),
        ],
      ],
    );
  }

  String _getFileName(String path) {
    return path.split('/').last;
  }

  String _formatFileSize(int bytes) {
    if (bytes < 1024) return '$bytes B';
    if (bytes < 1024 * 1024) return '${(bytes / 1024).toStringAsFixed(1)} KB';
    return '${(bytes / (1024 * 1024)).toStringAsFixed(2)} MB';
  }
}
