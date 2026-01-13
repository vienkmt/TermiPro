/// Serial configuration model for Flutter side
class SerialConfigModel {
  final String portName;
  final int baudRate;
  final int dataBits;
  final String stopBits;
  final String parity;
  final bool dtr;
  final bool rts;

  const SerialConfigModel({
    this.portName = '',
    this.baudRate = 115200,
    this.dataBits = 8,
    this.stopBits = '1',
    this.parity = 'None',
    this.dtr = true,
    this.rts = true,
  });

  SerialConfigModel copyWith({
    String? portName,
    int? baudRate,
    int? dataBits,
    String? stopBits,
    String? parity,
    bool? dtr,
    bool? rts,
  }) {
    return SerialConfigModel(
      portName: portName ?? this.portName,
      baudRate: baudRate ?? this.baudRate,
      dataBits: dataBits ?? this.dataBits,
      stopBits: stopBits ?? this.stopBits,
      parity: parity ?? this.parity,
      dtr: dtr ?? this.dtr,
      rts: rts ?? this.rts,
    );
  }

  /// Available baud rates (common speeds only)
  static const List<int> baudRates = [
    9600, 19200, 38400, 57600, 115200, 230400, 460800, 921600,
  ];

  /// Available data bits options
  static const List<int> dataBitsOptions = [5, 6, 7, 8];

  /// Available stop bits options
  static const List<String> stopBitsOptions = ['1', '1.5', '2'];

  /// Available parity options
  static const List<String> parityOptions = ['None', 'Odd', 'Even'];
}

/// Display options model
class DisplayOptions {
  final String displayMode; // 'Text', 'Hex', 'Chart'
  final bool autoScroll;
  final String lineEnding;
  final bool showTimestamp;

  const DisplayOptions({
    this.displayMode = 'Text',
    this.autoScroll = true,
    this.lineEnding = 'LF',
    this.showTimestamp = true,
  });

  DisplayOptions copyWith({
    String? displayMode,
    bool? autoScroll,
    String? lineEnding,
    bool? showTimestamp,
  }) {
    return DisplayOptions(
      displayMode: displayMode ?? this.displayMode,
      autoScroll: autoScroll ?? this.autoScroll,
      lineEnding: lineEnding ?? this.lineEnding,
      showTimestamp: showTimestamp ?? this.showTimestamp,
    );
  }

  /// Available display modes
  static const List<String> displayModes = ['Text', 'Hex', 'Chart'];

  /// Available line ending options
  static const List<String> lineEndingOptions = ['None', 'CR', 'LF', 'CRLF'];

  /// Check if hex mode
  bool get hexMode => displayMode == 'Hex';

  /// Check if chart mode
  bool get chartMode => displayMode == 'Chart';

  /// Get line ending characters
  String get lineEndingChars {
    switch (lineEnding) {
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
}

/// Auto send settings model
class AutoSendSettings {
  final int intervalMs;
  final int byteDelayUs;
  final bool enabled;
  final int sendCount;

  const AutoSendSettings({
    this.intervalMs = 1000,
    this.byteDelayUs = 0,
    this.enabled = false,
    this.sendCount = 0,
  });

  AutoSendSettings copyWith({
    int? intervalMs,
    int? byteDelayUs,
    bool? enabled,
    int? sendCount,
  }) {
    return AutoSendSettings(
      intervalMs: intervalMs ?? this.intervalMs,
      byteDelayUs: byteDelayUs ?? this.byteDelayUs,
      enabled: enabled ?? this.enabled,
      sendCount: sendCount ?? this.sendCount,
    );
  }
}
