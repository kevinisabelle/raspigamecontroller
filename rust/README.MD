# Gamepad Rust Implementation

## Setup

1. Install Rust
2. Install WSL2
3. Install Docker Desktop
4. Install cross (https://github.com/cross-rs/cross)

## Build

1. Run build-release-arm.bat

## Deploy

1. Copy the generated binary `gamepadki` from `target\aarch64-unknown-linux-gnu\release` to the Raspberry Pi
2. Chmod the binary to make it executable
3. Run the binary with `sudo -E ./gamepadki`
4. You can now pair with Windows using the Bluetooth settings

## TODO

- [ ] Auto reconnect if already paired
- [ ] Write a new gamepad values rust output in the C# HidReportCreator
- [ ] Fix X axis starting at 50 instead of 0
- [ ] Add battery reporting
- [ ] Fix controller name that Windows detects
- [ ] Remove the default Bluez advertisement
- [ ] Check for possible integration using https://github.com/petzval/btferret