# Zig Raspberry Pi Bluetooth HID Game controller

## Setup Raspberry Pi Zero 2 W

Use Rasp OS Lite 32 bits

### Setup pigpio and bluetooth

```
#sudo apt get py3-setuptools
sudo apt update && sudo apt upgrade -y
sudo apt install libbluetooth-dev

# Blutooth
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

### Setup HID for bluetooth

```
sudo sdptool add --xml=/etc/bluetooth/hid_sdp_record.xml

sudo bluetoothctl

power on
discoverable on
pairable on
agent KeyboardOnly
default-agent

sudo systemctl enable bluetooth

sudo vi /lib/systemd/system/bluetooth.service
--experimental

sudo systemctl daemon-reload
sudo systemctl restert bluetooth
```
