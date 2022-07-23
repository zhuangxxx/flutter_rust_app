import 'dart:async';
import 'dart:ffi';
import 'dart:io';

import 'package:flutter/material.dart' hide Size;
import 'package:flutter_rust_app/bridge_generated.dart';

// Simple Flutter code. If you are not familiar with Flutter, this may sounds a bit long. But indeed
// it is quite trivial and Flutter is just like that. Please refer to Flutter's tutorial to learn Flutter.

const base = 'flutter_rust_app';
final path = Platform.isWindows ? '$base.dll' : 'lib$base.so';
late final dylib = Platform.isIOS
    ? DynamicLibrary.process()
    : Platform.isMacOS
        ? DynamicLibrary.executable()
        : DynamicLibrary.open(path);
late final api = FlutterRustAppImpl(dylib);

void main() => runApp(const MyApp());

class MyApp extends StatefulWidget {
  const MyApp({Key? key}) : super(key: key);

  @override
  State<MyApp> createState() => _MyAppState();
}

class _MyAppState extends State<MyApp> {
  late int _count;

  @override
  void initState() {
    super.initState();
    _count = 1;
  }

  @override
  Widget build(BuildContext context) => MaterialApp(
        home: Scaffold(
          appBar: AppBar(title: const Text('Flutter Rust Application')),
          body: Column(
            mainAxisAlignment: MainAxisAlignment.spaceAround,
            children: [
              Container(height: 16),
              Container(
                padding: const EdgeInsets.symmetric(horizontal: 24),
                child: Text("$_count"),
              ),
              Container(height: 16),
              Column(mainAxisAlignment: MainAxisAlignment.end, children: [
                TextButton(
                  onPressed: _callMul2,
                  child: const Text('×2'),
                ),
                TextButton(
                  onPressed: () => setState(() {
                    _count = 0;
                  }),
                  child: const Text('清零'),
                )
              ])
            ],
          ),
        ),
      );

  Future<void> _callMul2() async {
    final value = await api.mul2(n: _count);
    setState(() {
      _count = value;
    });
  }
}
