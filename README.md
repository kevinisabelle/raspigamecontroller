# RaspberryPi Bluetooth LE HID Game controller

## Reference documentation

- _HID Usage Table_: https://www.usb.org/document-library/hid-usage-tables-16
- _Device Class Definition for Human Interface Devices (HID)_: https://www.usb.org/document-library/device-class-definition-hid-111
- _HID over GATT Profile 1.0_: https://www.bluetooth.com/specifications/specs/hid-over-gatt-profile-1-0/
- _Bluez dbus interfaces_: https://github.com/bluez/bluez/tree/master/doc

## Current Issues:

- Buttons not working
- 0 to 255 axis are left shift by 127
- Reconnection not working
- Need to create a service to start when the Pi starts.
- Packing axies don't work in Windows
- The number of axies also seems to be limited.

## Setup

### Install python and bluez

```bash
#sudo apt get py3-setuptools
sudo apt update && sudo apt upgrade -y
sudo apt install libbluetooth-dev

# Blutooth
sudo systemctl enable bluetooth
sudo systemctl start bluetooth
```

### Enable Spi0 and Spi1

- Run the raspi-config app and enable Interface Spi.
- Then modify /boot/firmware/config.txt
- Add these if not present:

```
dtparam=spi=on
dtoverlay=spi1-3cs
```

- Reboot

### Setup bluez service parameters

```bash
sudo nano /lib/systemd/system/bluetooth.service

# Modify the exec start
ExecStart=/usr/libexec/bluetooth/bluetoothd -E --noplugin=*

# Save the file and restart bluetooth

sudo systemctl daemon-reload
sudo systemctl restart bluetooth
```

### Generating the Hid Report Map and Payload Interface

In the HidTools folder, use the HidReportMapCreator program.

- Create a copy of the KiGPSimple.cs class and name it as you like
- Modify the Program.cs and use the new class to create the device reference
- Run the program
- Copy the python code from the output in the python/GamepadValues.py file
- Modify the python/UpdaterService.py and adapt the \_update_gamepad_controls according to the generated GamepadValues1 class

## Running

### Start the program

- Copy the content of the python folder on the Pi
- Run with

```bash
sudo python3 App.py
```

- Once the program is properly started, you can connect it to Windows using the Bluetooth configuration like any device.
- Once the device is bonded in Windows, you can open the "Set up USB controller" to display connected Gamepads.

## Debugging

### Using dbus-tool

Use dbus-tool to check Bluez is connecting with the program properly:

```

sudo dbus-monitor --system

```

### Using Wireshark

Use Wireshark on Windows to inspect the GATT services packets.

- Start a session using Pcap2
- Filter out protocol USB

```

!(\_ws.col.protocol == "USB")

```

- Or you can filter in the proper services:

```

(\_ws.col.protocol == "ATT" || \_ws.col.protocol == "SDP" || \_ws.col.protocol == "SMP" || \_ws.col.protocol == "GAP" || \_ws.col.protocol == "GATT" || \_ws.col.protocol == "HCI_EVT")

```

### Using Hid Explorer

You can also use this great online tool (which runs in the browser): https://nondebug.github.io/webhid-explorer/ . Using this tool you can connect to the recognized HID devices and see their definition and notifications.
