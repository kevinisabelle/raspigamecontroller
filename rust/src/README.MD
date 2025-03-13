# Gamepad Rust Implementation

## Setup

1. Install Rust
2. Install WSL2
3. Install Docker Desktop
4. Install cross (https://github.com/cross-rs/cross)

## Build

1. Run build-release-arm.bat

## Deploy

1. Copy the generated binary `gamepadki` from `target\aarch64-unknown-linux-gnu\debug` to the Raspberry Pi
2. Chmod the binary to make it executable
3. Run the binary with `sudo ./gamepadki`