import 'package:flutter/material.dart';
import 'package:termimax_flutter/theme/theme.dart';
import 'package:termimax_flutter/models/serial_config.dart';
import 'package:termimax_flutter/src/rust/api/models.dart';
import 'port_selector.dart';
import 'config_card.dart';
import 'signal_toggles.dart';
import 'display_options.dart';
import 'auto_send_card.dart';

/// Sidebar widget containing all configuration panels
class Sidebar extends StatelessWidget {
  final List<PortInfo> ports;
  final String? selectedPort;
  final bool isConnected;
  final bool isLoadingPorts;
  final SerialConfigModel config;
  final DisplayOptions displayOptions;
  final AutoSendSettings autoSendSettings;
  final VoidCallback onRefreshPorts;
  final ValueChanged<String?> onPortChanged;
  final VoidCallback onConnect;
  final ValueChanged<SerialConfigModel> onConfigChanged;
  final ValueChanged<DisplayOptions> onDisplayOptionsChanged;
  final ValueChanged<AutoSendSettings> onAutoSendSettingsChanged;

  const Sidebar({
    super.key,
    required this.ports,
    this.selectedPort,
    this.isConnected = false,
    this.isLoadingPorts = false,
    required this.config,
    required this.displayOptions,
    required this.autoSendSettings,
    required this.onRefreshPorts,
    required this.onPortChanged,
    required this.onConnect,
    required this.onConfigChanged,
    required this.onDisplayOptionsChanged,
    required this.onAutoSendSettingsChanged,
  });

  @override
  Widget build(BuildContext context) {
    return Container(
      width: 320,
      height: double.infinity,
      decoration: BoxDecoration(
        color: AppColors.background,
        border: Border(
          right: BorderSide(color: AppColors.border),
        ),
      ),
      child: SingleChildScrollView(
        padding: const EdgeInsets.all(16),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            // Port Selection
            PortSelector(
              ports: ports,
              selectedPort: selectedPort,
              isConnected: isConnected,
              isLoading: isLoadingPorts,
              onRefresh: onRefreshPorts,
              onPortChanged: onPortChanged,
              onConnect: onConnect,
            ),
            const SizedBox(height: 16),
            // Configuration
            ConfigCard(
              config: config,
              enabled: !isConnected,
              onChanged: onConfigChanged,
            ),
            const SizedBox(height: 16),
            // Signal Lines
            SignalToggles(
              dtr: config.dtr,
              rts: config.rts,
              enabled: !isConnected,
              onDtrChanged: (value) {
                onConfigChanged(config.copyWith(dtr: value));
              },
              onRtsChanged: (value) {
                onConfigChanged(config.copyWith(rts: value));
              },
            ),
            const SizedBox(height: 16),
            // Display Options
            DisplayOptionsCard(
              options: displayOptions,
              onChanged: onDisplayOptionsChanged,
            ),
            const SizedBox(height: 16),
            // Auto Send
            AutoSendCard(
              settings: autoSendSettings,
              isConnected: isConnected,
              onChanged: onAutoSendSettingsChanged,
            ),
          ],
        ),
      ),
    );
  }
}
