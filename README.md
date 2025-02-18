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

## Hid Creator output example:

```
Hid Report Map and Payload interface creator
---------------------------------------------------

Notes:
-------
1. This tool is used to generate a report map and payload for a HID device.
2. The report map is a list of instructions that the device uses to send data to the host.
3. It will generate the Python class code for the report payload to use in the python part of this project.

---------------------
Report Map Descriptor
---------------------

0x05, 0x01      # Usage Page (Generic Desktop Controls)
0x09, 0x05      # Usage (Gamepad)
0xA1, 0x01      # Collection (Application)
0x85, 0x01      # Report ID 1
0x05, 0x09      # Usage Page (Button)
0x19, 0x01      # Usage Minimum (1)
0x29, 0x08      # Usage Maximum (8)
0x15, 0x00      # Logical Minimum (0)
0x25, 0x01      # Logical Maximum (1)
0x75, 0x01      # Report Size (1)
0x95, 0x08      # Report Count (8)
0x81, 0x02      # Input (00000010)
0x05, 0x01      # Usage Page (Generic Desktop Controls)
0x09, 0x36      # Usage (Slider)
0x19, 0x01      # Usage Minimum (1)
0x29, 0x01      # Usage Maximum (1)
0x15, 0x00      # Logical Minimum (0)
0x25, 0xFF      # Logical Maximum (255)
0x75, 0x08      # Report Size (8)
0x95, 0x01      # Report Count (1)
0x81, 0x02      # Input (00000010)
0x05, 0x01      # Usage Page (Generic Desktop Controls)
0x09, 0x36      # Usage (Slider)
0x19, 0x01      # Usage Minimum (1)
0x29, 0x01      # Usage Maximum (1)
0x15, 0x00      # Logical Minimum (0)
0x25, 0xFF      # Logical Maximum (255)
0x75, 0x08      # Report Size (8)
0x95, 0x01      # Report Count (1)
0x81, 0x02      # Input (00000010)
0x05, 0x01      # Usage Page (Generic Desktop Controls)
0x09, 0x36      # Usage (Slider)
0x19, 0x01      # Usage Minimum (1)
0x29, 0x01      # Usage Maximum (1)
0x15, 0x00      # Logical Minimum (0)
0x25, 0xFF      # Logical Maximum (255)
0x75, 0x08      # Report Size (8)
0x95, 0x01      # Report Count (1)
0x81, 0x02      # Input (00000010)
0xC0    # End Collection

----------------------
Report Payload Example
----------------------

Btn1 #1 (1 bits) Unspecified
Btn1 #2 (1 bits) Unspecified
Btn1 #3 (1 bits) Unspecified
Btn1 #4 (1 bits) Unspecified
Btn1 #5 (1 bits) Unspecified
Btn1 #6 (1 bits) Unspecified
Btn1 #7 (1 bits) Unspecified
Btn1 #8 (1 bits) Unspecified
Slider #1 (8 bits) Slider
Slider2 #1 (8 bits) Slider
Slider3 #1 (8 bits) Slider

----------------------------------
Report Payload Example (Formatted)
----------------------------------

BBBBBBBB SSSSSSSS SSSSSSSS SSSSSSSS
12345678 11111111 11111111 11111111

------------
Python class
------------

class GamepadValues1:
    def __init__(self, Btn10=0, Btn11=0, Btn12=0, Btn13=0, Btn14=0, Btn15=0, Btn16=0, Btn17=0, Slider0=0, Slider20=0, Slider30=0):
        self.Btn10 = Btn10 & 0x1
        self.Btn11 = Btn11 & 0x1
        self.Btn12 = Btn12 & 0x1
        self.Btn13 = Btn13 & 0x1
        self.Btn14 = Btn14 & 0x1
        self.Btn15 = Btn15 & 0x1
        self.Btn16 = Btn16 & 0x1
        self.Btn17 = Btn17 & 0x1
        self.Slider0 = Slider0 & 0xFF
        self.Slider20 = Slider20 & 0xFF
        self.Slider30 = Slider30 & 0xFF

    def set_Btn10(self, value):
        self.Btn10 = value & 0x1

    def set_Btn11(self, value):
        self.Btn11 = value & 0x1

    def set_Btn12(self, value):
        self.Btn12 = value & 0x1

    def set_Btn13(self, value):
        self.Btn13 = value & 0x1

    def set_Btn14(self, value):
        self.Btn14 = value & 0x1

    def set_Btn15(self, value):
        self.Btn15 = value & 0x1

    def set_Btn16(self, value):
        self.Btn16 = value & 0x1

    def set_Btn17(self, value):
        self.Btn17 = value & 0x1

    def set_Slider0(self, value):
        self.Slider0 = value & 0xFF

    def set_Slider20(self, value):
        self.Slider20 = value & 0xFF

    def set_Slider30(self, value):
        self.Slider30 = value & 0xFF

    def get_report(self):
        total = 0
        total = (total << 1) | self.Btn10
        total = (total << 1) | self.Btn11
        total = (total << 1) | self.Btn12
        total = (total << 1) | self.Btn13
        total = (total << 1) | self.Btn14
        total = (total << 1) | self.Btn15
        total = (total << 1) | self.Btn16
        total = (total << 1) | self.Btn17
        total = (total << 8) | self.Slider0
        total = (total << 8) | self.Slider20
        total = (total << 8) | self.Slider30
        result = total.to_bytes(4, byteorder='big')
        return bytes([0x01] + list(result))


    def get_report_map(self):
        return bytes([
            0x05, 0x01,   # Usage Page (Generic Desktop Controls)
            0x09, 0x05,   # Usage (Gamepad)
            0xA1, 0x01,   # Collection (Application)
            0x85, 0x01,   # Report ID 1
            0x05, 0x09,   # Usage Page (Button)
            0x19, 0x01,   # Usage Minimum (1)
            0x29, 0x08,   # Usage Maximum (8)
            0x15, 0x00,   # Logical Minimum (0)
            0x25, 0x01,   # Logical Maximum (1)
            0x75, 0x01,   # Report Size (1)
            0x95, 0x08,   # Report Count (8)
            0x81, 0x02,   # Input (00000010)
            0x05, 0x01,   # Usage Page (Generic Desktop Controls)
            0x09, 0x36,   # Usage (Slider)
            0x19, 0x01,   # Usage Minimum (1)
            0x29, 0x01,   # Usage Maximum (1)
            0x15, 0x00,   # Logical Minimum (0)
            0x25, 0xFF,   # Logical Maximum (255)
            0x75, 0x08,   # Report Size (8)
            0x95, 0x01,   # Report Count (1)
            0x81, 0x02,   # Input (00000010)
            0x05, 0x01,   # Usage Page (Generic Desktop Controls)
            0x09, 0x36,   # Usage (Slider)
            0x19, 0x01,   # Usage Minimum (1)
            0x29, 0x01,   # Usage Maximum (1)
            0x15, 0x00,   # Logical Minimum (0)
            0x25, 0xFF,   # Logical Maximum (255)
            0x75, 0x08,   # Report Size (8)
            0x95, 0x01,   # Report Count (1)
            0x81, 0x02,   # Input (00000010)
            0x05, 0x01,   # Usage Page (Generic Desktop Controls)
            0x09, 0x36,   # Usage (Slider)
            0x19, 0x01,   # Usage Minimum (1)
            0x29, 0x01,   # Usage Maximum (1)
            0x15, 0x00,   # Logical Minimum (0)
            0x25, 0xFF,   # Logical Maximum (255)
            0x75, 0x08,   # Report Size (8)
            0x95, 0x01,   # Report Count (1)
            0x81, 0x02,   # Input (00000010)
            0xC0,   # End Collection
        ])

```
