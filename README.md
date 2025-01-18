# Zig Raspberry Pi Bluetooth HID Game controller

# Setup Raspberry Pi Zero 2 W

Use Raspberry Pi OS Lite (32 bits)

## Install Pigpio

```
sudo apt install python-setuptools python3-setuptools
git clone https://github.com/joan2937/pigpio.git
make
sudo make install
```

## Install bluetooth with bluez

```
sudo apt update && sudo apt upgrade -y
sudo apt install bluez -y
sudo systemctl enable bluetooth
sudo systemctl start bluetooth
hciconfig

# Make discoverable
sudo bluetoothctl

# Add inside the prompt:
power on
discoverable on
pairable on
agent on
```

## Enable Gadget mode for bluetooth
