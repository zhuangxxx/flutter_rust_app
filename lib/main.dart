import 'dart:async';
import 'dart:ffi';
import 'dart:io';

import 'package:flutter/material.dart' hide Size;
import 'package:flutter_rust_app/bridge_generated.dart';
import 'package:flutter_rust_app/map.dart';

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
  late List<GeoMap> _geoMapList = <GeoMap>[];

  @override
  void initState() {
    super.initState();
    _callLoadGeojson("static/data/340800_full.json");
  }

  @override
  Widget build(BuildContext context) => buildMapPageUi(_geoMapList);

  Future<void> _callLoadGeojson(String path) async {
    final loadedGeoMapList = await api.loadGeojson(path: path);
    if (mounted) setState(() => _geoMapList = loadedGeoMapList);
  }
}
