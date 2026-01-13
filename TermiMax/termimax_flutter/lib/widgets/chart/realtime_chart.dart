import 'dart:ui' as ui;
import 'package:flutter/material.dart';
import 'package:termimax_flutter/theme/theme.dart';

/// Real-time line chart for visualizing serial data
/// Optimized for high-frequency data (100Hz+)
class RealtimeChart extends StatelessWidget {
  final List<double> data;
  final int maxPoints;
  final double? minY;
  final double? maxY;
  final Color lineColor;
  final double lineWidth;
  final bool showGrid;
  final bool showStats;

  const RealtimeChart({
    super.key,
    required this.data,
    this.maxPoints = 500,
    this.minY,
    this.maxY,
    this.lineColor = const Color(0xFF0EA5E9),
    this.lineWidth = 1.5,
    this.showGrid = true,
    this.showStats = true,
  });

  @override
  Widget build(BuildContext context) {
    // Calculate stats in single pass
    double min = 0, max = 100, avg = 50;
    if (data.isNotEmpty) {
      min = data[0];
      max = data[0];
      double sum = 0;
      for (final v in data) {
        if (v < min) min = v;
        if (v > max) max = v;
        sum += v;
      }
      avg = sum / data.length;
    }

    final computedMinY = minY ?? (min - (max - min) * 0.1);
    final computedMaxY = maxY ?? (max + (max - min) * 0.1);

    return Column(
      children: [
        // Stats header
        if (showStats)
          Container(
            padding: const EdgeInsets.symmetric(horizontal: 16, vertical: 8),
            decoration: BoxDecoration(
              border: Border(
                bottom: BorderSide(color: AppColors.border),
              ),
            ),
            child: Row(
              children: [
                Icon(Icons.show_chart, size: 18, color: AppColors.primary),
                const SizedBox(width: 8),
                Text('Real-time Chart', style: AppTypography.labelLarge),
                const Spacer(),
                _buildStatChip('Min', min.toStringAsFixed(1), AppColors.primary),
                const SizedBox(width: 8),
                _buildStatChip('Max', max.toStringAsFixed(1), AppColors.warning),
                const SizedBox(width: 8),
                _buildStatChip('Avg', avg.toStringAsFixed(1), AppColors.success),
                const SizedBox(width: 8),
                _buildStatChip('Pts', '${data.length}', AppColors.textSecondary),
              ],
            ),
          ),
        // Chart area
        Expanded(
          child: data.isEmpty
              ? _buildEmptyState()
              : ClipRect(
                  child: CustomPaint(
                    painter: _ChartPainter(
                      data: data,
                      minY: computedMinY,
                      maxY: computedMaxY,
                      lineColor: lineColor,
                      lineWidth: lineWidth,
                      showGrid: showGrid,
                    ),
                    size: Size.infinite,
                  ),
                ),
        ),
      ],
    );
  }

  Widget _buildStatChip(String label, String value, Color color) {
    return Container(
      padding: const EdgeInsets.symmetric(horizontal: 8, vertical: 4),
      decoration: BoxDecoration(
        color: color.withOpacity(0.1),
        borderRadius: BorderRadius.circular(4),
      ),
      child: Row(
        mainAxisSize: MainAxisSize.min,
        children: [
          Text(
            '$label: ',
            style: AppTypography.labelSmall.copyWith(color: AppColors.textSecondary),
          ),
          Text(
            value,
            style: AppTypography.stats.copyWith(color: color),
          ),
        ],
      ),
    );
  }

  Widget _buildEmptyState() {
    return Center(
      child: Column(
        mainAxisSize: MainAxisSize.min,
        children: [
          Icon(Icons.show_chart, size: 48, color: AppColors.textTertiary),
          const SizedBox(height: 16),
          Text(
            'No data to display',
            style: AppTypography.bodyMedium.copyWith(color: AppColors.textSecondary),
          ),
          const SizedBox(height: 4),
          Text(
            'Connect and receive numeric data to see the chart',
            style: AppTypography.bodySmall.copyWith(color: AppColors.textTertiary),
          ),
        ],
      ),
    );
  }
}

/// Chart painter - simple and reliable
class _ChartPainter extends CustomPainter {
  final List<double> data;
  final double minY;
  final double maxY;
  final Color lineColor;
  final double lineWidth;
  final bool showGrid;

  _ChartPainter({
    required this.data,
    required this.minY,
    required this.maxY,
    required this.lineColor,
    required this.lineWidth,
    required this.showGrid,
  });

  @override
  void paint(Canvas canvas, Size size) {
    if (data.isEmpty) return;

    const padding = EdgeInsets.fromLTRB(50, 20, 20, 30);
    final chartRect = Rect.fromLTWH(
      padding.left,
      padding.top,
      size.width - padding.left - padding.right,
      size.height - padding.top - padding.bottom,
    );

    // Background
    canvas.drawRect(
      chartRect,
      Paint()..color = const Color(0xFFFAFAFA),
    );

    // Grid
    if (showGrid) {
      _drawGrid(canvas, chartRect);
    }

    // Y-axis labels
    _drawYAxisLabels(canvas, chartRect);

    // Line chart
    _drawLineChart(canvas, chartRect);

    // Border
    canvas.drawRect(
      chartRect,
      Paint()
        ..color = const Color(0xFFE2E8F0)
        ..style = PaintingStyle.stroke
        ..strokeWidth = 1,
    );
  }

  void _drawGrid(Canvas canvas, Rect rect) {
    final paint = Paint()..strokeWidth = 1;

    // Horizontal lines
    paint.color = const Color(0xFFE2E8F0);
    for (int i = 0; i <= 4; i++) {
      final y = rect.top + (rect.height / 4) * i;
      canvas.drawLine(Offset(rect.left, y), Offset(rect.right, y), paint);
    }

    // Vertical lines
    paint.color = const Color(0xFFF1F5F9);
    for (int i = 0; i <= 10; i++) {
      final x = rect.left + (rect.width / 10) * i;
      canvas.drawLine(Offset(x, rect.top), Offset(x, rect.bottom), paint);
    }
  }

  void _drawYAxisLabels(Canvas canvas, Rect rect) {
    final textStyle = ui.TextStyle(
      color: const Color(0xFF64748B),
      fontSize: 10,
      fontFamily: 'JetBrains Mono',
    );

    for (int i = 0; i <= 4; i++) {
      final value = maxY - (maxY - minY) * (i / 4);
      final y = rect.top + (rect.height / 4) * i;

      final builder = ui.ParagraphBuilder(ui.ParagraphStyle(textAlign: TextAlign.right))
        ..pushStyle(textStyle)
        ..addText(value.toStringAsFixed(1));

      final paragraph = builder.build()..layout(const ui.ParagraphConstraints(width: 40));
      canvas.drawParagraph(paragraph, Offset(rect.left - 45, y - 6));
    }
  }

  void _drawLineChart(Canvas canvas, Rect rect) {
    if (data.length < 2) return;

    final range = maxY - minY;
    if (range == 0) return;

    // Downsample if needed
    final maxDisplayPoints = rect.width.toInt().clamp(100, 600);
    final displayData = data.length > maxDisplayPoints
        ? _downsample(data, maxDisplayPoints)
        : data;

    final xStep = rect.width / (displayData.length - 1);

    // Build path
    final path = Path();
    final firstY = rect.bottom - ((displayData[0] - minY) / range) * rect.height;
    path.moveTo(rect.left, firstY.clamp(rect.top, rect.bottom));

    for (int i = 1; i < displayData.length; i++) {
      final x = rect.left + xStep * i;
      final y = rect.bottom - ((displayData[i] - minY) / range) * rect.height;
      path.lineTo(x, y.clamp(rect.top, rect.bottom));
    }

    // Gradient fill
    final fillPath = Path.from(path)
      ..lineTo(rect.right, rect.bottom)
      ..lineTo(rect.left, rect.bottom)
      ..close();

    canvas.drawPath(
      fillPath,
      Paint()
        ..shader = ui.Gradient.linear(
          Offset(rect.left, rect.top),
          Offset(rect.left, rect.bottom),
          [lineColor.withOpacity(0.2), lineColor.withOpacity(0.02)],
        ),
    );

    // Line
    canvas.drawPath(
      path,
      Paint()
        ..color = lineColor
        ..strokeWidth = lineWidth
        ..style = PaintingStyle.stroke
        ..strokeCap = StrokeCap.round
        ..strokeJoin = StrokeJoin.round,
    );
  }

  /// Simple downsampling - take every Nth point
  List<double> _downsample(List<double> data, int targetPoints) {
    final result = <double>[];
    final step = data.length / targetPoints;
    for (int i = 0; i < targetPoints; i++) {
      result.add(data[(i * step).floor().clamp(0, data.length - 1)]);
    }
    return result;
  }

  @override
  bool shouldRepaint(covariant _ChartPainter oldDelegate) => true;
}
