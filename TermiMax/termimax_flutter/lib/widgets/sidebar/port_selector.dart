import 'package:flutter/material.dart';
import 'package:termimax_flutter/theme/theme.dart';
import 'package:termimax_flutter/src/rust/api/models.dart';

/// Port selector dropdown with refresh and connect button
class PortSelector extends StatefulWidget {
  final List<PortInfo> ports;
  final String? selectedPort;
  final bool isConnected;
  final bool isLoading;
  final VoidCallback onRefresh;
  final ValueChanged<String?> onPortChanged;
  final VoidCallback onConnect;

  const PortSelector({
    super.key,
    required this.ports,
    this.selectedPort,
    this.isConnected = false,
    this.isLoading = false,
    required this.onRefresh,
    required this.onPortChanged,
    required this.onConnect,
  });

  @override
  State<PortSelector> createState() => _PortSelectorState();
}

class _PortSelectorState extends State<PortSelector> with SingleTickerProviderStateMixin {
  bool _isOpen = false;

  late AnimationController _animController;
  late Animation<double> _expandAnim;

  @override
  void initState() {
    super.initState();
    _animController = AnimationController(
      duration: const Duration(milliseconds: 400),
      vsync: this,
    );
    _expandAnim = CurvedAnimation(
      parent: _animController,
      curve: Curves.easeOutCubic,
      reverseCurve: Curves.easeInCubic,
    );
  }

  @override
  void dispose() {
    _animController.dispose();
    super.dispose();
  }

  void _toggleDropdown() {
    if (widget.isConnected) return;

    if (_isOpen) {
      _animController.reverse().then((_) {
        if (mounted) setState(() => _isOpen = false);
      });
    } else {
      widget.onRefresh();
      setState(() => _isOpen = true);
      _animController.forward();
    }
  }

  void _selectPort(String portName) {
    widget.onPortChanged(portName);
    _closeDropdown();
  }

  void _closeDropdown() {
    if (_isOpen) {
      _animController.reverse().then((_) {
        if (mounted) setState(() => _isOpen = false);
      });
    }
  }

  void _handleConnect() {
    _closeDropdown();
    widget.onConnect();
  }

  PortInfo? _getSelectedPortInfo() {
    if (widget.selectedPort == null || widget.selectedPort!.isEmpty) {
      return null;
    }
    return widget.ports.where((p) => p.name == widget.selectedPort).firstOrNull;
  }

  @override
  Widget build(BuildContext context) {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        Text('Port', style: AppTypography.sectionTitle),
        const SizedBox(height: 8),

        // Port dropdown container
        Container(
          decoration: BoxDecoration(
            borderRadius: BorderRadius.circular(6),
            border: Border.all(
              color: _isOpen ? AppColors.primary : AppColors.border,
              width: 1,
            ),
            boxShadow: _isOpen
                ? [BoxShadow(color: AppColors.primary.withOpacity(0.1), blurRadius: 4, spreadRadius: 1)]
                : [],
          ),
          child: ClipRRect(
            borderRadius: BorderRadius.circular(5),
            child: Column(
              mainAxisSize: MainAxisSize.min,
              children: [
              // Input field
              GestureDetector(
                onTap: _toggleDropdown,
                child: MouseRegion(
                  cursor: widget.isConnected ? SystemMouseCursors.basic : SystemMouseCursors.click,
                  child: Container(
                    padding: const EdgeInsets.symmetric(horizontal: 12, vertical: 10),
                    decoration: BoxDecoration(
                      color: widget.isConnected ? AppColors.surfaceVariant : AppColors.surface,
                      borderRadius: _isOpen
                          ? const BorderRadius.vertical(top: Radius.circular(5))
                          : BorderRadius.circular(5),
                    ),
                    child: Row(
                      children: [
                        Icon(
                          Icons.usb,
                          size: 18,
                          color: _isOpen ? AppColors.primary : AppColors.textSecondary,
                        ),
                        const SizedBox(width: 10),
                        Expanded(child: _buildPortInfo()),
                        if (widget.isLoading)
                          const SizedBox(
                            width: 16,
                            height: 16,
                            child: CircularProgressIndicator(strokeWidth: 2),
                          )
                        else
                          AnimatedRotation(
                            duration: const Duration(milliseconds: 400),
                            curve: Curves.easeOutCubic,
                            turns: _isOpen ? 0.5 : 0,
                            child: Icon(
                              Icons.keyboard_arrow_down,
                              size: 18,
                              color: _isOpen ? AppColors.primary : AppColors.textSecondary,
                            ),
                          ),
                      ],
                    ),
                  ),
                ),
              ),

              // Dropdown list - animated
              SizeTransition(
                sizeFactor: _expandAnim,
                axisAlignment: -1,
                child: _buildDropdownList(),
              ),
            ],
            ),
          ),
        ),

        const SizedBox(height: 12),

        // Connect button
        SizedBox(
          width: double.infinity,
          child: ElevatedButton(
            onPressed: widget.selectedPort != null && widget.selectedPort!.isNotEmpty
                ? _handleConnect
                : null,
            style: ElevatedButton.styleFrom(
              backgroundColor: widget.isConnected ? AppColors.error : AppColors.primary,
              foregroundColor: Colors.white,
              padding: const EdgeInsets.symmetric(vertical: 18),
              shape: RoundedRectangleBorder(
                borderRadius: BorderRadius.circular(6),
              ),
              elevation: 0,
            ).copyWith(
              overlayColor: WidgetStateProperty.all(Colors.white.withOpacity(0.1)),
            ),
            child: Row(
              mainAxisAlignment: MainAxisAlignment.center,
              children: [
                Icon(widget.isConnected ? Icons.link_off : Icons.link, size: 16),
                const SizedBox(width: 6),
                Text(
                  widget.isConnected ? 'Disconnect' : 'Connect',
                  style: const TextStyle(fontSize: 12, fontWeight: FontWeight.w600),
                ),
              ],
            ),
          ),
        ),
      ],
    );
  }

  Widget _buildPortInfo() {
    final port = _getSelectedPortInfo();
    final hasPort = widget.selectedPort != null && widget.selectedPort!.isNotEmpty;
    final textColor = hasPort ? AppColors.textPrimary : AppColors.textTertiary;

    if (port != null && port.product != null && port.product!.isNotEmpty) {
      return Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        mainAxisSize: MainAxisSize.min,
        children: [
          Text(
            port.portType,
            style: TextStyle(fontSize: 12, fontWeight: FontWeight.w500, color: textColor),
          ),
          const SizedBox(height: 1),
          Text(
            port.product!,
            style: TextStyle(fontSize: 10, color: AppColors.textTertiary),
            maxLines: 1,
            overflow: TextOverflow.ellipsis,
          ),
        ],
      );
    }

    return Text(
      hasPort ? (port?.portType ?? widget.selectedPort!.split('/').last) : 'Select port...',
      style: TextStyle(fontSize: 12, fontWeight: FontWeight.w500, color: textColor),
    );
  }

  Widget _buildDropdownList() {
    if (!_isOpen) return const SizedBox.shrink();

    return Container(
      constraints: const BoxConstraints(maxHeight: 200),
      decoration: BoxDecoration(
        color: AppColors.surface,
        border: Border(top: BorderSide(color: AppColors.border)),
      ),
      child: widget.isLoading
          ? const Padding(
              padding: EdgeInsets.all(16),
              child: Center(
                child: SizedBox(
                  width: 18,
                  height: 18,
                  child: CircularProgressIndicator(strokeWidth: 2),
                ),
              ),
            )
          : widget.ports.isEmpty
              ? Padding(
                  padding: const EdgeInsets.all(16),
                  child: Center(
                    child: Text(
                      'No ports found',
                      style: TextStyle(fontSize: 11, color: AppColors.textTertiary),
                    ),
                  ),
                )
              : SingleChildScrollView(
                  child: Column(
                    mainAxisSize: MainAxisSize.min,
                    children: widget.ports.map((port) {
                      final isSelected = port.name == widget.selectedPort;
                      return _PortItem(
                        port: port,
                        isSelected: isSelected,
                        onTap: () => _selectPort(port.name),
                      );
                    }).toList(),
                  ),
                ),
    );
  }
}

/// Port list item with hover effect
class _PortItem extends StatefulWidget {
  final PortInfo port;
  final bool isSelected;
  final VoidCallback onTap;

  const _PortItem({
    required this.port,
    required this.isSelected,
    required this.onTap,
  });

  @override
  State<_PortItem> createState() => _PortItemState();
}

class _PortItemState extends State<_PortItem> {
  @override
  Widget build(BuildContext context) {
    return Material(
      color: widget.isSelected ? AppColors.primarySurface : Colors.transparent,
      child: InkWell(
        onTap: widget.onTap,
        hoverColor: AppColors.primary.withOpacity(0.06),
        splashColor: AppColors.primary.withOpacity(0.1),
        child: Padding(
          padding: const EdgeInsets.symmetric(horizontal: 12, vertical: 10),
          child: Row(
            children: [
              Icon(
                Icons.usb,
                size: 16,
                color: widget.isSelected ? AppColors.primary : AppColors.textSecondary,
              ),
              const SizedBox(width: 10),
              Expanded(
                child: Column(
                  crossAxisAlignment: CrossAxisAlignment.start,
                  children: [
                    Text(
                      widget.port.portType,
                      style: TextStyle(
                        fontSize: 12,
                        fontWeight: widget.isSelected ? FontWeight.w600 : FontWeight.w500,
                        color: widget.isSelected ? AppColors.primary : AppColors.textPrimary,
                      ),
                    ),
                    if (widget.port.product != null) ...[
                      const SizedBox(height: 1),
                      Text(
                        widget.port.product!,
                        style: TextStyle(fontSize: 10, color: AppColors.textTertiary),
                        maxLines: 1,
                        overflow: TextOverflow.ellipsis,
                      ),
                    ],
                  ],
                ),
              ),
              if (widget.isSelected)
                Icon(Icons.check, size: 14, color: AppColors.primary),
            ],
          ),
        ),
      ),
    );
  }
}
