# Zig Raspberry Pi Bluetooth HID Game controller

## Setup Raspberry Pi Zero 2 W

Use Rasp OS Lite 64 bits

### Setup pigpio and bluetooth

```
#sudo apt get py3-setuptools
sudo apt update && sudo apt upgrade -y
sudo apt install libbluetooth-dev

# Blutooth
sudo systemctl enable bluetooth
sudo systemctl start bluetooth
hciconfig

### Setup HID for bluetooth

```

sudo vi /lib/systemd/system/bluetooth.service
--experimental

sudo systemctl daemon-reload
sudo systemctl restart bluetooth

```

```
