# flutter_ffi

A new Flutter project.

## Getting Started

This project is a starting point for a Flutter application.

A few resources to get you started if this is your first Flutter project:

- [Lab: Write your first Flutter app](https://docs.flutter.dev/get-started/codelab)
- [Cookbook: Useful Flutter samples](https://docs.flutter.dev/cookbook)

For help getting started with Flutter development, view the
[online documentation](https://docs.flutter.dev/), which offers tutorials,
samples, guidance on mobile development, and a full API reference.


参考：https://www.cnblogs.com/xdd666/p/16975394.html

用到的命令：
```shell
winget install -e --id LLVM.LLVM
cargo install cargo-ndk --version 2.6.0
flutter_rust_bridge_codegen -r native/src/api.rs -d lib/ffi/rust_ffi.dart -c ios/Runner/bridge_generated.h


cargo xcode
```
