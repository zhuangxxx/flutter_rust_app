# Flutter + Rust example for `flutter_rust_bridge`

In order to build and run this example, please visit [the tutorial section](https://github.com/fzyzcjy/flutter_rust_bridge#-tutorial-a-flutterrust-app) of repository readme.

For a pure-Dart example without UI, please see the `pure_dart` example which is a neighbor of this example.

For full documentation, please see README.md of the main repository.

# Run generator

```powershell
flutter_rust_bridge_codegen --llvm-path D:/zx/lib/llvm --rust-input rust/src/api.rs --dart-output lib/bridge_generated.dart --c-output ios/Runner/bridge_generated.h
```

# Android app build

```powershell
cd rust
ANDROID_NDK_HOME=D:/zx/lib/android-sdk/ndk/21.1.6352462 cargo ndk -o ../android/app/src/main/jniLibs --target x86_64-linux-android build
```

# iOS app build

```powershell
cargo lipo && cp target/universal/debug/libflutter_rust_bridge_example.a ../ios/Runner
```
