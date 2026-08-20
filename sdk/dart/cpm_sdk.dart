/// CPM Dart / Flutter SDK — Native Client for CPM / UPM Polyglot Bridge RPC
///
/// Usage:
///     import 'package:cpm_sdk/cpm_sdk.dart';
///     final bridge = CpmBridge();
///     final result = await bridge.call('python:math.sqrt', [144.0]);

import 'dart:convert';
import 'dart:io';

class CpmBridge {
  final String cpmBin;

  CpmBridge({String? cpmBin}) : cpmBin = cpmBin ?? _findCpmBin();

  static String _findCpmBin() {
    var curr = Directory.current;
    for (var i = 0; i < 5; i++) {
      for (var rel in ['target/release/cpm.exe', 'target/debug/cpm.exe', 'target/release/cpm', 'target/debug/cpm']) {
        var f = File('${curr.path}/$rel');
        if (f.existsSync()) {
          return f.path;
        }
      }
      var parent = curr.parent;
      if (parent.path == curr.path) break;
      curr = parent;
    }
    return 'cpm';
  }

  Future<dynamic> call(String target, List<dynamic> args) async {
    final argsJson = jsonEncode(args);
    final res = await Process.run(cpmBin, ['bridge', 'call', target, argsJson]);

    if (res.exitCode != 0) {
      throw Exception('CPM Bridge Call Error: ${res.stderr.toString().trim()}');
    }

    final stdoutStr = res.stdout.toString().trim();
    final lines = stdoutStr.split('\n');
    var capture = false;
    final jsonLines = <String>[];

    for (var line in lines) {
      if (line.contains('Response received:')) {
        capture = true;
        continue;
      }
      if (capture && (line.contains('round-trip via stdio RPC') || line.trim().isEmpty)) {
        if (jsonLines.isNotEmpty) break;
      }
      if (capture) {
        jsonLines.add(line);
      }
    }

    final rawJson = jsonLines.join('\n').trim();
    if (rawJson.isNotEmpty) {
      return jsonDecode(rawJson);
    }
    return stdoutStr;
  }
}
