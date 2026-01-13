import 'dart:async';
import 'package:flutter/cupertino.dart';
import 'package:flutter/material.dart';
import 'package:macos_ui/macos_ui.dart' show MacosSwitch, MacosTooltip, MacosIcon, MacosIconButton, PushButton, ControlSize;
import 'package:termimax_flutter/theme/theme.dart';
import 'package:termimax_flutter/models/serial_config.dart';
import 'package:termimax_flutter/widgets/sidebar/port_selector.dart';
import 'package:termimax_flutter/widgets/sidebar/config_card.dart';
import 'package:termimax_flutter/widgets/sidebar/signal_toggles.dart';
import 'package:termimax_flutter/widgets/sidebar/display_options.dart';
import 'package:termimax_flutter/widgets/sidebar/auto_send_card.dart';
import 'package:termimax_flutter/widgets/chart/realtime_chart.dart';
import 'dart:io';
import 'package:file_picker/file_picker.dart';
import 'package:excel/excel.dart' as excel;
import 'package:termimax_flutter/models/log_export_settings.dart';
import 'package:termimax_flutter/widgets/sidebar/log_export_card.dart';
import 'package:termimax_flutter/src/rust/api/models.dart';
import 'package:termimax_flutter/src/rust/api/serial.dart';

/// Main serial monitor screen with sidebar and terminal
class SerialScreen extends StatefulWidget {
  const SerialScreen({super.key});

  @override
  State<SerialScreen> createState() => _SerialScreenState();
}

class _SerialScreenState extends State<SerialScreen> {
  // State
  List<PortInfo> _ports = [];
  String? _selectedPort;
  bool _isConnected = false;
  bool _isLoadingPorts = false;
  SerialConfigModel _config = const SerialConfigModel();
  DisplayOptions _displayOptions = const DisplayOptions();
  AutoSendSettings _autoSendSettings = const AutoSendSettings();
  String _txLineEnding = 'LF'; // Separate line ending for TX

  // Terminal data
  final List<TerminalEntry> _terminalEntries = [];
  int _txCount = 0;
  int _rxCount = 0;

  // Buffer for incomplete lines (line-based parsing)
  final List<int> _rxBuffer = [];

  // Chart data (stores numeric values for visualization)
  final List<double> _chartData = [];
  static const int _maxChartPoints = 500;

  // Batching: accumulate data and flush periodically
  final List<List<int>> _pendingRxData = [];
  Timer? _flushTimer;

  // Stream subscription
  StreamSubscription<SerialData>? _dataSubscription;

  // Auto send timer
  Timer? _autoSendTimer;

  // Log Export
  LogExportSettings _logExportSettings = const LogExportSettings();
  final StringBuffer _logBuffer = StringBuffer();
  Timer? _logExportTimer;

  // Controllers
  final TextEditingController _sendController = TextEditingController();
  final ScrollController _terminalScrollController = ScrollController();
  final ScrollController _sidebarScrollController = ScrollController();

  @override
  void initState() {
    super.initState();
    _refreshPorts();
  }

  @override
  void dispose() {
    _dataSubscription?.cancel();
    _autoSendTimer?.cancel();
    _logExportTimer?.cancel();
    _flushTimer?.cancel();
    _sendController.dispose();
    _terminalScrollController.dispose();
    _sidebarScrollController.dispose();
    super.dispose();
  }

  // ==================== BUSINESS LOGIC ====================

  Future<void> _refreshPorts() async {
    setState(() => _isLoadingPorts = true);
    try {
      final ports = await listSerialPorts();
      setState(() {
        _ports = ports;
        _isLoadingPorts = false;
      });
    } catch (e) {
      setState(() => _isLoadingPorts = false);
      _showError('Failed to list ports: $e');
    }
  }

  Future<void> _toggleConnection() async {
    if (_isConnected) {
      await _disconnect();
    } else {
      await _connect();
    }
  }

  Future<void> _connect() async {
    if (_selectedPort == null) return;

    try {
      // Clear old data before connecting
      _clearTerminal();

      final rustConfig = SerialConfig(
        portName: _selectedPort!,
        baudRate: _config.baudRate,
        dataBits: _config.dataBits,
        stopBits: _config.stopBits,
        parity: _config.parity,
        dtr: _config.dtr,
        rts: _config.rts,
      );

      await openPort(config: rustConfig);
      setState(() => _isConnected = true);
      
      // Log connection event
      if (_logExportSettings.enabled) {
        _logBuffer.write('\r\n========================================\r\n');
        _logBuffer.write('CONNECTED: ${_selectedPort!} at ${DateTime.now().toString()}\r\n');
        _logBuffer.write('========================================\r\n');
      }
      
      _startDataStream();
    } catch (e) {
      final error = e.toString();
      if (error.contains('BUSY:')) {
        _showError('Port is busy. Close other applications using this port.');
      } else {
        _showError('Connection failed: $error');
      }
    }
  }

  void _startDataStream() {
    _dataSubscription?.cancel();
    _dataSubscription = streamSerialData(
      portName: _selectedPort!,
      baudRate: _config.baudRate,
    ).listen(
      (data) {
        if (data.data.isEmpty) {
          _handleDisconnect();
        } else {
          _handleReceivedData(data.data);
        }
      },
      onError: (e) {
        _handleDisconnect();
      },
    );
  }

  void _handleReceivedData(List<int> data) {
    _pendingRxData.add(List.from(data));

    // Use Timer instead of addPostFrameCallback for cancellable scheduling
    _flushTimer ??= Timer(const Duration(milliseconds: 16), () {
      _flushTimer = null;
      if (_pendingRxData.isNotEmpty && mounted) {
        _flushPendingData();
      }
    });
  }

  void _flushPendingData() {
    final List<TerminalEntry> newEntries = [];

    for (final data in _pendingRxData) {
      if (_displayOptions.lineEnding == 'None') {
        newEntries.add(TerminalEntry(
          type: EntryType.rx,
          data: data,
          timestamp: DateTime.now(),
        ));
      } else {
        _rxBuffer.addAll(data);
      }
    }

    if (_displayOptions.lineEnding != 'None') {
      final delimiter = _getDelimiterBytes();
      int searchStart = 0;

      while (true) {
        final delimIndex = _findDelimiter(_rxBuffer, delimiter, searchStart);
        if (delimIndex == -1) break;

        if (delimIndex > searchStart) {
          newEntries.add(TerminalEntry(
            type: EntryType.rx,
            data: _rxBuffer.sublist(searchStart, delimIndex),
            timestamp: DateTime.now(),
          ));
        }
        searchStart = delimIndex + delimiter.length;
      }

      if (searchStart > 0) {
        _rxBuffer.removeRange(0, searchStart);
      }
    }

    _pendingRxData.clear();

    if (newEntries.isNotEmpty) {
      setState(() {
        _rxCount += newEntries.length;
        _terminalEntries.addAll(newEntries);

        while (_terminalEntries.length > 500) {
          _terminalEntries.removeAt(0);
        }

        for (final entry in newEntries) {
          if (entry.type == EntryType.rx) {
            // Log to file if enabled
            _appendToLogBuffer(entry.timestamp, 'RX', entry.data);

            final text = String.fromCharCodes(entry.data).trim();
            final value = double.tryParse(text);
            if (value != null) {
              _chartData.add(value);
              while (_chartData.length > _maxChartPoints) {
                _chartData.removeAt(0);
              }
            }
          }
        }
      });

      // Only scroll if autoScroll is enabled
      if (_displayOptions.autoScroll) {
        _autoScroll();
      }
    }
  }

  List<int> _getDelimiterBytes() {
    switch (_displayOptions.lineEnding) {
      case 'CR':
        return [13];
      case 'LF':
        return [10];
      case 'CRLF':
        return [13, 10];
      default:
        return [10];
    }
  }

  int _findDelimiter(List<int> buffer, List<int> delimiter, int start) {
    for (int i = start; i <= buffer.length - delimiter.length; i++) {
      bool match = true;
      for (int j = 0; j < delimiter.length; j++) {
        if (buffer[i + j] != delimiter[j]) {
          match = false;
          break;
        }
      }
      if (match) return i;
    }
    return -1;
  }

  void _autoScroll() {
    WidgetsBinding.instance.addPostFrameCallback((_) {
      // Check autoScroll at execution time, not at schedule time
      if (_displayOptions.autoScroll && _terminalScrollController.hasClients) {
        _terminalScrollController.jumpTo(
          _terminalScrollController.position.maxScrollExtent,
        );
      }
    });
  }

  void _handleDisconnect() {
    _stopAutoSend();
    _flushTimer?.cancel();
    _flushTimer = null;
    _rxBuffer.clear();
    _pendingRxData.clear();
    
    // Log disconnect event
    if (_logExportSettings.enabled && _isConnected) {
        _logBuffer.write('\r\n========================================\r\n');
        _logBuffer.write('DISCONNECTED (Error/Remote) at ${DateTime.now().toString()}\r\n');
        _logBuffer.write('========================================\r\n');
    }
    
    setState(() => _isConnected = false);
    _showError('Port disconnected');
  }

  Future<void> _disconnect() async {
    try {
      _stopAutoSend();
      _dataSubscription?.cancel();
      _dataSubscription = null;
      await closePort(portName: _selectedPort!);
      _rxBuffer.clear();
      
      // Log disconnect event
      if (_logExportSettings.enabled) {
        _logBuffer.write('\r\n========================================\r\n');
        _logBuffer.write('DISCONNECTED (User action) at ${DateTime.now().toString()}\r\n');
        _logBuffer.write('========================================\r\n');
      }
      
      setState(() => _isConnected = false);
    } catch (e) {
      _showError('Disconnect failed: $e');
    }
  }

  String get _txLineEndingChars {
    switch (_txLineEnding) {
      case 'CR':
        return '\r';
      case 'LF':
        return '\n';
      case 'CRLF':
        return '\r\n';
      default:
        return '';
    }
  }

  Future<void> _sendData() async {
    if (!_isConnected || _sendController.text.isEmpty) return;

    final text = _sendController.text + _txLineEndingChars;

    try {
      await sendData(
        portName: _selectedPort!,
        data: text,
        isHex: _displayOptions.hexMode,
        byteDelayUs: _autoSendSettings.byteDelayUs > 0
            ? BigInt.from(_autoSendSettings.byteDelayUs)
            : null,
      );

      setState(() {
        _txCount++;
        _terminalEntries.add(TerminalEntry(
          type: EntryType.tx,
          data: _displayOptions.hexMode
              ? _parseHexString(_sendController.text)
              : text.codeUnits,
          timestamp: DateTime.now(),
        ));
        
        // Log TX
        final txData = _displayOptions.hexMode
            ? _parseHexString(_sendController.text)
            : text.codeUnits;
        _appendToLogBuffer(DateTime.now(), 'TX', txData);
      });

      // Scroll to bottom after sending
      if (_displayOptions.autoScroll) {
        _autoScroll();
      }
    } catch (e) {
      _showError('Send failed: $e');
    }
  }

  List<int> _parseHexString(String hex) {
    final cleanHex = hex.replaceAll(RegExp(r'[^0-9A-Fa-f]'), '');
    final bytes = <int>[];
    for (var i = 0; i < cleanHex.length; i += 2) {
      if (i + 1 < cleanHex.length) {
        bytes.add(int.parse(cleanHex.substring(i, i + 2), radix: 16));
      }
    }
    return bytes;
  }

  void _toggleAutoSend() {
    if (_autoSendSettings.enabled) {
      _stopAutoSend();
    } else {
      _startAutoSend();
    }
  }

  void _startAutoSend() {
    if (!_isConnected || _sendController.text.isEmpty) return;

    setState(() {
      _autoSendSettings = _autoSendSettings.copyWith(enabled: true, sendCount: 0);
    });

    _autoSendTimer = Timer.periodic(
      Duration(milliseconds: _autoSendSettings.intervalMs),
      (_) {
        if (_isConnected && _sendController.text.isNotEmpty) {
          _sendData();
          setState(() {
            _autoSendSettings = _autoSendSettings.copyWith(
              sendCount: _autoSendSettings.sendCount + 1,
            );
          });
        } else {
          _stopAutoSend();
        }
      },
    );
  }

  void _stopAutoSend() {
    _autoSendTimer?.cancel();
    _autoSendTimer = null;
    setState(() {
      _autoSendSettings = _autoSendSettings.copyWith(enabled: false);
    });
  }

  void _clearTerminal() {
    // Cancel any pending flush - this allows new data to be shown immediately
    _flushTimer?.cancel();
    _flushTimer = null;
    _rxBuffer.clear();
    _pendingRxData.clear();
    setState(() {
      _terminalEntries.clear();
      _chartData.clear();
      _txCount = 0;
      _rxCount = 0;
    });
  }

  void _showError(String message) {
    ScaffoldMessenger.of(context).showSnackBar(
      SnackBar(
        content: Text(message),
        backgroundColor: AppColors.error,
        behavior: SnackBarBehavior.floating,
      ),
    );
  }

  // ==================== LOG EXPORT ====================

  Future<void> _toggleLogExport(bool enabled) async {
    if (enabled) {
      if (_isConnected && _selectedPort != null) {
        // If connected, we can start immediately if we have a file path
        // But usually we want to ask for path first
      }

      String? outputFile = await FilePicker.platform.saveFile(
        dialogTitle: 'Select location to save logs',
        fileName: 'termi_log_${DateTime.now().millisecondsSinceEpoch}.txt',
      );

      if (outputFile != null) {
        setState(() {
          _logExportSettings = _logExportSettings.copyWith(
            enabled: true,
            filePath: outputFile,
            fileSize: 0,
          );
        });

        // Initial dump of existing logs
        final sb = StringBuffer();
        for (final entry in _terminalEntries) {
           final typeStr = entry.type == EntryType.tx ? 'TX' : 'RX';
           final t = entry.timestamp;
           final date = '${t.year}-${t.month.toString().padLeft(2, '0')}-${t.day.toString().padLeft(2, '0')}';
           final time = '${t.hour.toString().padLeft(2, '0')}:${t.minute.toString().padLeft(2, '0')}:${t.second.toString().padLeft(2, '0')}';
           final timestamp = '$date $time';
           final dataStr = _displayOptions.hexMode
               ? entry.data.map((b) => b.toRadixString(16).padLeft(2, '0')).join(' ')
               : String.fromCharCodes(entry.data).replaceAll('\r', '').replaceAll('\n', '');
           sb.write('$timestamp\t$typeStr\t$dataStr\r\n');
        }

        try {
          final file = File(outputFile);
          await file.writeAsString(sb.toString());
          final length = await file.length();
          
          setState(() {
            _logExportSettings = _logExportSettings.copyWith(fileSize: length);
          });
          
          _startLogTimer();
        } catch (e) {
          _showError('Failed to write log file: $e');
          setState(() {
             _logExportSettings = _logExportSettings.copyWith(enabled: false);
          });
        }
      }
    } else {
      _stopLogExport();
    }
  }

  void _stopLogExport() {
    _logExportTimer?.cancel();
    _logExportTimer = null;
    
    // Flush remaining
    if (_logBuffer.isNotEmpty) {
      _flushLogBuffer();
    }
    
    setState(() {
      _logExportSettings = _logExportSettings.copyWith(enabled: false);
    });
  }

  void _startLogTimer() {
    _logExportTimer?.cancel();
    _logExportTimer = Timer.periodic(const Duration(seconds: 1), (timer) {
      if (_logBuffer.isNotEmpty) {
        _flushLogBuffer();
      }
    });
  }

  Future<void> _flushLogBuffer() async {
    if (_logExportSettings.filePath == null) return;
    
    final content = _logBuffer.toString();
    _logBuffer.clear();
    
    try {
      final file = File(_logExportSettings.filePath!);
      await file.writeAsString(content, mode: FileMode.append);
      final length = await file.length();
      
      setState(() {
        _logExportSettings = _logExportSettings.copyWith(fileSize: length);
      });
    } catch (e) {
      debugPrint('Error writing to log file: $e');
    }
  }

  void _appendToLogBuffer(DateTime timestamp, String type, List<int> data) {
    if (!_logExportSettings.enabled) return;
    
    final dataStr = _displayOptions.hexMode
        ? data.map((b) => b.toRadixString(16).padLeft(2, '0')).join(' ')
        : String.fromCharCodes(data).replaceAll('\r', '').replaceAll('\n', '');
        
    final date = '${timestamp.year}-${timestamp.month.toString().padLeft(2, '0')}-${timestamp.day.toString().padLeft(2, '0')}';
    final time = '${timestamp.hour.toString().padLeft(2, '0')}:${timestamp.minute.toString().padLeft(2, '0')}:${timestamp.second.toString().padLeft(2, '0')}';
    _logBuffer.write('$date $time\t$type\t$dataStr\r\n');
  }

  void _openLogFile() {
    if (_logExportSettings.filePath != null) {
      if (Platform.isMacOS) {
        Process.run('open', [_logExportSettings.filePath!]);
      } else if (Platform.isWindows) {
        Process.run('explorer', [_logExportSettings.filePath!]);
      } else if (Platform.isLinux) {
        Process.run('xdg-open', [_logExportSettings.filePath!]);
      }
    }
  }

  Future<void> _exportToExcel() async {
    if (_logExportSettings.filePath == null) return;
    
    // Check if connected
    if (_isConnected) {
      _showError('Please disconnect before exporting to Excel');
      return;
    }

    try {
      // 1. Read existing log file
      final logFile = File(_logExportSettings.filePath!);
      if (!await logFile.exists()) {
        _showError('Log file not found');
        return;
      }
      
      final lines = await logFile.readAsLines();
      if (lines.isEmpty) {
        _showError('Log file is empty');
        return;
      }

      // 2. Create Excel
      final xl = excel.Excel.createExcel();
      final sheet = xl['Sheet1'];
      
      // Define Styles
      final headerStyle = excel.CellStyle(
        bold: true,
        backgroundColorHex: excel.ExcelColor.fromHexString('#E0E0E0'),
        horizontalAlign: excel.HorizontalAlign.Center,
        verticalAlign: excel.VerticalAlign.Center,
        leftBorder: excel.Border(borderStyle: excel.BorderStyle.Medium),
        rightBorder: excel.Border(borderStyle: excel.BorderStyle.Medium),
        bottomBorder: excel.Border(borderStyle: excel.BorderStyle.Medium),
        topBorder: excel.Border(borderStyle: excel.BorderStyle.Medium),
        fontFamily: excel.getFontFamily(excel.FontFamily.Arial),
      );

      final txStyle = excel.CellStyle(
        backgroundColorHex: excel.ExcelColor.fromHexString('#E3F2FD'),
        horizontalAlign: excel.HorizontalAlign.Center,
        verticalAlign: excel.VerticalAlign.Center,
        leftBorder: excel.Border(borderStyle: excel.BorderStyle.Thin),
        rightBorder: excel.Border(borderStyle: excel.BorderStyle.Thin),
        bottomBorder: excel.Border(borderStyle: excel.BorderStyle.Thin),
        topBorder: excel.Border(borderStyle: excel.BorderStyle.Thin),
        fontFamily: excel.getFontFamily(excel.FontFamily.Arial),
      );

      final rxStyle = excel.CellStyle(
        horizontalAlign: excel.HorizontalAlign.Center,
        verticalAlign: excel.VerticalAlign.Center,
        leftBorder: excel.Border(borderStyle: excel.BorderStyle.Thin),
        rightBorder: excel.Border(borderStyle: excel.BorderStyle.Thin),
        bottomBorder: excel.Border(borderStyle: excel.BorderStyle.Thin),
        topBorder: excel.Border(borderStyle: excel.BorderStyle.Thin),
        fontFamily: excel.getFontFamily(excel.FontFamily.Arial),
      );
      
      final eventStyle = excel.CellStyle(
        italic: true,
        fontColorHex: excel.ExcelColor.fromHexString('#757575'),
        horizontalAlign: excel.HorizontalAlign.Center,
        verticalAlign: excel.VerticalAlign.Center,
      );

      // Set Column Widths (Data column auto-fits)
      sheet.setColumnWidth(0, 25.0); // Timestamp
      sheet.setColumnWidth(1, 15.0); // Type

      // Header
      final headerRow = [
        excel.TextCellValue('Timestamp'), 
        excel.TextCellValue('Type'), 
        excel.TextCellValue('Data')
      ];
      sheet.appendRow(headerRow);
      
      // Apply Header Style
      for (int i = 0; i < headerRow.length; i++) {
        var cell = sheet.cell(excel.CellIndex.indexByColumnRow(columnIndex: i, rowIndex: 0));
        cell.cellStyle = headerStyle;
      }
      
      // 3. Parse and add rows
      int rowIndex = 1;
      for (final line in lines) {
        if (line.trim().isEmpty) continue;
        
        if (line.startsWith('====') || line.startsWith('CONNECTED') || line.startsWith('DISCONNECTED')) {
             sheet.appendRow([excel.TextCellValue(line)]); 
             // Merge cells for event row
             sheet.merge(
               excel.CellIndex.indexByColumnRow(columnIndex: 0, rowIndex: rowIndex),
               excel.CellIndex.indexByColumnRow(columnIndex: 2, rowIndex: rowIndex),
               customValue: excel.TextCellValue(line)
             );
             var cell = sheet.cell(excel.CellIndex.indexByColumnRow(columnIndex: 0, rowIndex: rowIndex));
             cell.cellStyle = eventStyle;
             rowIndex++;
             continue;
        }

        final parts = line.split('\t');
        if (parts.length >= 3) {
           final rowData = [
             excel.TextCellValue(parts[0]), 
             excel.TextCellValue(parts[1]), 
             excel.TextCellValue(parts.sublist(2).join('\t'))
           ];
           sheet.appendRow(rowData);
           
           // Apply Format
           final style = parts[1].contains('TX') ? txStyle : rxStyle;
           for (int i = 0; i < 3; i++) {
             var cell = sheet.cell(excel.CellIndex.indexByColumnRow(columnIndex: i, rowIndex: rowIndex));
             cell.cellStyle = style;
           }
        } else {
           sheet.appendRow([excel.TextCellValue(line)]);
           var cell = sheet.cell(excel.CellIndex.indexByColumnRow(columnIndex: 0, rowIndex: rowIndex));
           cell.cellStyle = eventStyle;
        }
        rowIndex++;
      }

      // 4. Save
      String? outputFile = await FilePicker.platform.saveFile(
        dialogTitle: 'Save Excel file',
        fileName: 'termi_log_${DateTime.now().millisecondsSinceEpoch}.xlsx',
        type: FileType.custom,
        allowedExtensions: ['xlsx'],
      );

      if (outputFile != null) {
        if (!outputFile.endsWith('.xlsx')) outputFile += '.xlsx';
        final fileParams = xl.encode();
        if (fileParams != null) {
            File(outputFile)
              ..createSync(recursive: true)
              ..writeAsBytesSync(fileParams);
            
            // Open option
             ScaffoldMessenger.of(context).showSnackBar(
              SnackBar(
                content: const Text('Export successful!'),
                action: SnackBarAction(
                  label: 'Open',
                  onPressed: () {
                     if (Platform.isMacOS) {
                        Process.run('open', [outputFile!]);
                      } else if (Platform.isWindows) {
                        Process.run('explorer', [outputFile!]);
                      }
                  },
                ),
                backgroundColor: AppColors.success,
              ),
            );
        }
      }

    } catch (e) {
      _showError('Export failed: $e');
    }
  }

  // ==================== UI BUILD ====================

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      backgroundColor: AppColors.background,
      body: Row(
        children: [
          // Sidebar
          Container(
            width: 300,
            decoration: BoxDecoration(
              color: AppColors.sidebarBackground,
              border: Border(
                right: BorderSide(color: AppColors.sidebarBorder),
              ),
            ),
            child: Column(
              children: [
                Expanded(child: _buildSidebar()),
                _buildConnectionStatus(),
              ],
            ),
          ),
          // Main content - clean container
          Expanded(
            child: Container(
              color: AppColors.surface,
              child: Column(
                children: [
                  _buildTerminalHeader(),
                  Expanded(child: _buildTerminalContent()),
                  _buildSendPanel(),
                ],
              ),
            ),
          ),
        ],
      ),
    );
  }

  Widget _buildSidebar() {
    return SingleChildScrollView(
      controller: _sidebarScrollController,
      padding: const EdgeInsets.all(12),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          // Port section
          _buildSection(
            child: PortSelector(
              ports: _ports,
              selectedPort: _selectedPort,
              isConnected: _isConnected,
              isLoading: _isLoadingPorts,
              onRefresh: _refreshPorts,
              onPortChanged: (port) => setState(() => _selectedPort = port),
              onConnect: _toggleConnection,
            ),
          ),
          const SizedBox(height: 8),
          // Config section
          _buildSection(
            child: ConfigCard(
              config: _config,
              enabled: !_isConnected,
              onChanged: (config) => setState(() => _config = config),
            ),
          ),
          const SizedBox(height: 8),
          // Signals section
          _buildSection(
            child: SignalToggles(
              dtr: _config.dtr,
              rts: _config.rts,
              enabled: !_isConnected,
              onDtrChanged: (value) {
                setState(() => _config = _config.copyWith(dtr: value));
              },
              onRtsChanged: (value) {
                setState(() => _config = _config.copyWith(rts: value));
              },
            ),
          ),
          const SizedBox(height: 8),
          // Display section
          _buildSection(
            child: DisplayOptionsCard(
              options: _displayOptions,
              onChanged: (options) => setState(() => _displayOptions = options),
            ),
          ),
          const SizedBox(height: 8),
          // Auto Send section
          _buildSection(
            child: AutoSendCard(
              settings: _autoSendSettings,
              isConnected: _isConnected,
              onChanged: (settings) => setState(() => _autoSendSettings = settings),
            ),
          ),
          const SizedBox(height: 8),
          // Log Export section
          _buildSection(
            child: LogExportCard(
              settings: _logExportSettings,
              onChanged: (enabled) => _toggleLogExport(enabled),
              onOpen: _openLogFile,
              onExportExcel: _exportToExcel,
              isConnected: _isConnected,
            ),
          ),
        ],
      ),
    );
  }

  Widget _buildSection({required Widget child}) {
    return Container(
      width: double.infinity,
      padding: const EdgeInsets.all(12),
      decoration: BoxDecoration(
        color: AppColors.surface,
        borderRadius: BorderRadius.circular(6),
        boxShadow: AppColors.subtleShadow,
      ),
      child: child,
    );
  }

  Widget _buildConnectionStatus() {
    return Padding(
      padding: const EdgeInsets.fromLTRB(16, 8, 16, 16),
      child: Row(
        mainAxisAlignment: MainAxisAlignment.center,
        children: [
          Container(
            width: 6,
            height: 6,
            decoration: BoxDecoration(
              color: _isConnected ? AppColors.success : AppColors.textTertiary,
              shape: BoxShape.circle,
            ),
          ),
          const SizedBox(width: 6),
          Text(
            _isConnected ? 'Connected' : 'Disconnected',
            style: TextStyle(
              fontSize: 10,
              fontWeight: FontWeight.w500,
              color: _isConnected ? AppColors.success : AppColors.textTertiary,
            ),
          ),
        ],
      ),
    );
  }

  Widget _buildTerminalHeader() {
    return Container(
      padding: const EdgeInsets.symmetric(horizontal: 16, vertical: 8),
      decoration: BoxDecoration(
        border: Border(bottom: BorderSide(color: AppColors.divider)),
      ),
      child: Row(
        children: [
          Text(
            'TX',
            style: TextStyle(
              fontSize: 10,
              fontWeight: FontWeight.w600,
              color: AppColors.txText,
            ),
          ),
          const SizedBox(width: 4),
          Text(
            '$_txCount',
            style: TextStyle(
              fontSize: 10,
              fontWeight: FontWeight.w500,
              color: AppColors.textTertiary,
            ),
          ),
          const SizedBox(width: 16),
          Text(
            'RX',
            style: TextStyle(
              fontSize: 10,
              fontWeight: FontWeight.w600,
              color: AppColors.rxText,
            ),
          ),
          const SizedBox(width: 4),
          Text(
            '$_rxCount',
            style: TextStyle(
              fontSize: 10,
              fontWeight: FontWeight.w500,
              color: AppColors.textTertiary,
            ),
          ),
          const Spacer(),
          GestureDetector(
            onTap: _clearTerminal,
            child: MouseRegion(
              cursor: SystemMouseCursors.click,
              child: Row(
                mainAxisSize: MainAxisSize.min,
                children: [
                  Icon(Icons.delete_outline, size: 14, color: AppColors.error),
                  const SizedBox(width: 4),
                  Text(
                    'Clear',
                    style: TextStyle(
                      fontSize: 10,
                      fontWeight: FontWeight.w500,
                      color: AppColors.error,
                    ),
                  ),
                ],
              ),
            ),
          ),
        ],
      ),
    );
  }

  Widget _buildTerminalContent() {
    if (_displayOptions.chartMode) {
      return RealtimeChart(
        key: ValueKey(_rxCount),
        data: _chartData,
        maxPoints: _maxChartPoints,
        lineColor: AppColors.primary,
      );
    }

    if (_terminalEntries.isEmpty) {
      return _buildEmptyTerminal();
    }

    return Scrollbar(
      controller: _terminalScrollController,
      thumbVisibility: true,
      child: ListView.builder(
        controller: _terminalScrollController,
        padding: const EdgeInsets.all(12),
        itemCount: _terminalEntries.length,
        itemBuilder: (context, index) {
          return _buildTerminalLine(_terminalEntries[index]);
        },
      ),
    );
  }

  Widget _buildEmptyTerminal() {
    return Center(
      child: Column(
        mainAxisSize: MainAxisSize.min,
        children: [
          Icon(
            Icons.terminal,
            size: 48,
            color: AppColors.textTertiary.withOpacity(0.5),
          ),
          const SizedBox(height: 12),
          Text(
            'No data yet',
            style: AppTypography.bodyMedium.copyWith(
              color: AppColors.textSecondary,
            ),
          ),
          const SizedBox(height: 4),
          Text(
            _isConnected ? 'Waiting for data...' : 'Connect to start',
            style: AppTypography.bodySmall.copyWith(
              color: AppColors.textTertiary,
            ),
          ),
        ],
      ),
    );
  }

  Widget _buildTerminalLine(TerminalEntry entry) {
    final isTx = entry.type == EntryType.tx;
    final dataText = _displayOptions.hexMode
        ? entry.data.map((b) => b.toRadixString(16).padLeft(2, '0').toUpperCase()).join(' ')
        : String.fromCharCodes(entry.data).replaceAll('\r', '').replaceAll('\n', '');

    // Simple flat layout: timestamp | type | data
    return Padding(
      padding: const EdgeInsets.symmetric(vertical: 2),
      child: SelectableText.rich(
        TextSpan(
          style: AppTypography.terminal,
          children: [
            TextSpan(
              text: _formatTime(entry.timestamp),
              style: TextStyle(color: AppColors.textTertiary),
            ),
            TextSpan(
              text: isTx ? '  TX  ' : '  RX  ',
              style: TextStyle(
                color: isTx ? AppColors.txText : AppColors.rxText,
                fontWeight: FontWeight.w600,
              ),
            ),
            TextSpan(
              text: dataText,
              style: TextStyle(color: AppColors.textPrimary),
            ),
          ],
        ),
      ),
    );
  }

  String _formatTime(DateTime time) {
    return '${time.hour.toString().padLeft(2, '0')}:'
        '${time.minute.toString().padLeft(2, '0')}:'
        '${time.second.toString().padLeft(2, '0')}';
  }

  Widget _buildSendPanel() {
    return Container(
      padding: const EdgeInsets.symmetric(horizontal: 12, vertical: 10),
      decoration: BoxDecoration(
        border: Border(top: BorderSide(color: AppColors.borderLight)),
      ),
      child: Row(
        children: [
          // Input field
          Expanded(
            child: TextField(
              controller: _sendController,
              enabled: _isConnected,
              style: AppTypography.terminal.copyWith(fontSize: 13),
              decoration: InputDecoration(
                hintText: _displayOptions.hexMode
                    ? 'Hex: 48 65 6C 6C 6F'
                    : 'Enter message...',
                hintStyle: AppTypography.bodySmall.copyWith(
                  color: AppColors.textTertiary,
                ),
                isDense: true,
                contentPadding: const EdgeInsets.symmetric(horizontal: 12, vertical: 10),
                border: OutlineInputBorder(
                  borderRadius: BorderRadius.circular(8),
                  borderSide: BorderSide(color: AppColors.border),
                ),
                enabledBorder: OutlineInputBorder(
                  borderRadius: BorderRadius.circular(8),
                  borderSide: BorderSide(color: AppColors.border),
                ),
                focusedBorder: OutlineInputBorder(
                  borderRadius: BorderRadius.circular(8),
                  borderSide: BorderSide(color: AppColors.primary, width: 2),
                ),
                filled: true,
                fillColor: AppColors.background,
              ),
              onSubmitted: (_) => _sendData(),
            ),
          ),
          const SizedBox(width: 12),
          // Send button
          ElevatedButton.icon(
            onPressed: _isConnected && !_autoSendSettings.enabled ? _sendData : null,
            icon: const Icon(Icons.send, size: 16),
            label: const Text('Send'),
            style: ElevatedButton.styleFrom(
              backgroundColor: AppColors.primary,
              foregroundColor: Colors.white,
              padding: const EdgeInsets.symmetric(horizontal: 16, vertical: 10),
              shape: RoundedRectangleBorder(
                borderRadius: BorderRadius.circular(6),
              ),
              elevation: 0,
            ),
          ),
          const SizedBox(width: 8),
          // Auto Send button
          ElevatedButton.icon(
            onPressed: _isConnected ? _toggleAutoSend : null,
            icon: Icon(
              _autoSendSettings.enabled ? Icons.stop : Icons.repeat,
              size: 16,
            ),
            label: Text(
              _autoSendSettings.enabled
                  ? 'Stop (${_autoSendSettings.sendCount})'
                  : 'Auto',
            ),
            style: ElevatedButton.styleFrom(
              backgroundColor: _autoSendSettings.enabled
                  ? AppColors.error
                  : AppColors.success,
              foregroundColor: Colors.white,
              padding: const EdgeInsets.symmetric(horizontal: 16, vertical: 10),
              shape: RoundedRectangleBorder(
                borderRadius: BorderRadius.circular(6),
              ),
              elevation: 0,
            ),
          ),
          const SizedBox(width: 8),
          // TX Line End selector
          _buildTxLineEndSelector(),
        ],
      ),
    );
  }

  Widget _buildTxLineEndSelector() {
    const options = ['None', 'CR', 'LF', 'CRLF'];
    return Container(
      decoration: BoxDecoration(
        color: AppColors.surfaceVariant,
        borderRadius: BorderRadius.circular(6),
      ),
      child: Row(
        mainAxisSize: MainAxisSize.min,
        children: options.map((opt) {
          final isSelected = opt == _txLineEnding;
          return GestureDetector(
            onTap: () => setState(() => _txLineEnding = opt),
            child: MouseRegion(
              cursor: SystemMouseCursors.click,
              child: Container(
                padding: const EdgeInsets.symmetric(horizontal: 10, vertical: 8),
                decoration: BoxDecoration(
                  color: isSelected ? AppColors.primary : Colors.transparent,
                  borderRadius: BorderRadius.circular(5),
                ),
                child: Text(
                  opt,
                  style: TextStyle(
                    fontSize: 11,
                    fontWeight: isSelected ? FontWeight.w600 : FontWeight.w400,
                    color: isSelected ? Colors.white : AppColors.textSecondary,
                  ),
                ),
              ),
            ),
          );
        }).toList(),
      ),
    );
  }

}

/// Terminal entry model
class TerminalEntry {
  final EntryType type;
  final List<int> data;
  final DateTime timestamp;

  TerminalEntry({
    required this.type,
    required this.data,
    required this.timestamp,
  });
}

enum EntryType { tx, rx }
