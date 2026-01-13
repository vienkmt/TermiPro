import 'package:flutter/material.dart';
import 'package:macos_ui/macos_ui.dart';
import 'package:termimax_flutter/src/rust/frb_generated.dart';
import 'package:termimax_flutter/screens/serial_screen.dart';
import 'package:termimax_flutter/theme/theme.dart';

Future<void> main() async {
  WidgetsFlutterBinding.ensureInitialized();
  await RustLib.init();
  runApp(const MyApp());
}

class MyApp extends StatelessWidget {
  const MyApp({super.key});

  @override
  Widget build(BuildContext context) {
    return MacosTheme(
      data: MacosThemeData.light().copyWith(
        primaryColor: AppColors.primary,
      ),
      child: MaterialApp(
        title: 'vToolbox',
        debugShowCheckedModeBanner: false,
        theme: ThemeData(
          useMaterial3: true,
          colorScheme: ColorScheme.light(
            primary: AppColors.primary,
            secondary: AppColors.primaryLight,
            surface: AppColors.surface,
            error: AppColors.error,
          ),
          scaffoldBackgroundColor: AppColors.background,
          fontFamily: 'SF Pro Display',
        ),
        home: const SerialScreen(),
      ),
    );
  }
}
