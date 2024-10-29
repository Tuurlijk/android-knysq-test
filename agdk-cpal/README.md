# android-knyst-test
Generate sound data in rust and play on android

The library generates white noise using https://github.com/ErikNatanael/knyst

This is a minimal test application based on `GameActivity` that just
runs a mainloop based on android_activity::poll_events() and plays white noise from knyst.

## Cargo ndk

We use [cargo-ndk](https://github.com/bbqsrc/cargo-ndk) to compile Rust projects against the Android NDK without hassle.

You can install it using:

```shell
cargo install cargo-ndk
```

And setup the toolchains you intend to use using:

```shell
rustup target add \
    aarch64-linux-android \
    armv7-linux-androideabi \
    x86_64-linux-android \
    i686-linux-android
```

You may also need to set these env vars so ndk knows where to read from:

```shell
export ANDROID_HOME=${HOME}/Applications/Android/Sdk
export ANDROID_SDK_ROOT=${HOME}/Applications/Android/Sdk
export ANDROID_NDK_ROOT=${ANDROID_SDK_ROOT}/ndk/28.0.12433566
export ANDROID_NDK_HOME=${HOME}/Applications/Android/Sdk/ndk/28.0.12433566
export ANDROID_API=29
```

Point them to your NDK and SDK locations. The easiest way I have found is to install those using the SDK manager in Android Studio. But other setups should work just as fine.

## How to run

Connect your phone in USB debugging mode.

Build the libs with the build script: `./build.sh`

Then run the app on the USB device (-d):

```shell
adb -d shell am start -n com.example.androidcpaltest/.MainActivity
```

If you have multiple devices attached, you can list them with: `adb devices`. Then target a device to run the activity on:

```shell
 adb devices
List of devices attached
c0ad78fa	device
emulator-5554	device
```

```shell
adb -s c0ad78fa shell am start -n com.example.androidcpaltest/.MainActivity
```

## Inspect logs

You can see the logs from the usb device using:

```shell
adb -d logcat -v color
```

## Get the crash log

Sometimes there will be a crash log called a *tombstone*. It will be stored on the device in `/data/tombstones/`. You can get it using:

```shell
adb bugreport ./bugreport.zip
```

### Get the stack tool to inspect tombstones

```shell
git clone https://android.googlesource.com/platform/development
```

### parse the tombstone

```shell
./development/scripts/stack < ./bugreport/FS/data/tombstones/tombstone_20
```
