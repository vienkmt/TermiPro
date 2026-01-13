class LogExportSettings {
  final bool enabled;
  final String? filePath;
  final int fileSize; // in bytes

  const LogExportSettings({
    this.enabled = false,
    this.filePath,
    this.fileSize = 0,
  });

  LogExportSettings copyWith({
    bool? enabled,
    String? filePath,
    int? fileSize,
  }) {
    return LogExportSettings(
      enabled: enabled ?? this.enabled,
      filePath: filePath ?? this.filePath,
      fileSize: fileSize ?? this.fileSize,
    );
  }
}
