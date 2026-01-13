import 'package:flutter/material.dart';
import 'package:termimax_flutter/theme/theme.dart';
import 'package:termimax_flutter/models/serial_config.dart';
import 'package:termimax_flutter/widgets/common/custom_dropdown.dart';

/// Configuration section - flat design
class ConfigCard extends StatelessWidget {
  final SerialConfigModel config;
  final bool enabled;
  final ValueChanged<SerialConfigModel> onChanged;

  const ConfigCard({
    super.key,
    required this.config,
    this.enabled = true,
    required this.onChanged,
  });

  @override
  Widget build(BuildContext context) {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        // Section title
        Text(
          'Configuration',
          style: AppTypography.sectionTitle,
        ),
        const SizedBox(height: 12),
        // Row 1: Baud + Data
        Row(
          children: [
            Expanded(
              child: _buildField('Baud', config.baudRate, SerialConfigModel.baudRates,
                  (v) => onChanged(config.copyWith(baudRate: v))),
            ),
            const SizedBox(width: 8),
            Expanded(
              child: _buildField('Data', config.dataBits, SerialConfigModel.dataBitsOptions,
                  (v) => onChanged(config.copyWith(dataBits: v))),
            ),
          ],
        ),
        const SizedBox(height: 8),
        // Row 2: Stop + Parity
        Row(
          children: [
            Expanded(
              child: _buildFieldString('Stop', config.stopBits, SerialConfigModel.stopBitsOptions,
                  (v) => onChanged(config.copyWith(stopBits: v))),
            ),
            const SizedBox(width: 8),
            Expanded(
              child: _buildFieldString('Parity', config.parity, SerialConfigModel.parityOptions,
                  (v) => onChanged(config.copyWith(parity: v))),
            ),
          ],
        ),
      ],
    );
  }

  Widget _buildField<T>(String label, T value, List<T> options, ValueChanged<T> onChanged) {
    return CustomDropdown<T>(
      label: label,
      value: value,
      enabled: enabled,
      items: options.map((v) => DropdownItem(value: v, label: v.toString())).toList(),
      onChanged: onChanged,
    );
  }

  Widget _buildFieldString(String label, String value, List<String> options, ValueChanged<String> onChanged) {
    return CustomDropdown<String>(
      label: label,
      value: value,
      enabled: enabled,
      items: options.map((v) => DropdownItem(value: v, label: v)).toList(),
      onChanged: onChanged,
    );
  }
}
