import 'dart:ffi';
import 'dart:io';

import 'package:flutter_ffi/ffi/rust_ffi.dart';

class NativeFFI {
  NativeFFI._();

  static DynamicLibrary? _dyLib;

  static DynamicLibrary get dyLib {
    if (_dyLib != null) return _dyLib!;

    const base = 'rust_demo';
    if (Platform.isIOS) {
      _dyLib = DynamicLibrary.process();
    } else if (Platform.isAndroid) {
      _dyLib = DynamicLibrary.open('lib$base.so');
    } else {
      throw Exception('DynamicLibrary初始化失败');
    }

    return _dyLib!;
  }
}

class NativeFun {
  static final _ffi = RustDemoImpl(NativeFFI.dyLib);

  static Future<void> websocketConnect(String host) async {
    return await _ffi.websocketConnect(host: host);
  }

  static Future<void> sendMessage(String host, String message) async {
    return await _ffi.sendMessage(host: host, message: message);
  }

  static Future<void> websocketDisconnect(String host) async {
    return await _ffi.websocketDisconnect(host: host);
  }
}