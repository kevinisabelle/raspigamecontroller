import os
import time
import dbus
from evdev import UInput, ecodes as e

def setup_bluetooth():
    os.system("bluetoothctl agent NoInputNoOutput")
    os.system("bluetoothctl default-agent")
    os.system("bluetoothctl pairable on")
    os.system("bluetoothctl discoverable on")
    os.system("hciconfig hci0 up")
    os.system("hciconfig hci0 class 0x002540")  # Set to HID device
    os.system("hciconfig hci0 name 'RPi Keyboard'")
    os.system("sdptool add HID")
    print("Bluetooth setup complete. Pairing will not require a PIN.")

def create_uinput_device():
    capabilities = {
        e.EV_KEY: [e.KEY_A],
    }
    return UInput(capabilities, name="Bluetooth Keyboard Emulator")

def wait_for_connection():
    bus = dbus.SystemBus()
    manager = dbus.Interface(bus.get_object("org.bluez", "/"), "org.freedesktop.DBus.ObjectManager")
    adapter_path = None

    # Find the adapter path
    for path, interfaces in manager.GetManagedObjects().items():
        if "org.bluez.Adapter1" in interfaces:
            adapter_path = path
            break

    if not adapter_path:
        print("Bluetooth adapter not found.")
        return

    adapter = dbus.Interface(bus.get_object("org.bluez", adapter_path), "org.freedesktop.DBus.Properties")

    print("Waiting for device to connect...")
    while True:
        objects = manager.GetManagedObjects()
        for path, interfaces in objects.items():
            if "org.bluez.Device1" in interfaces:
                device = dbus.Interface(bus.get_object("org.bluez", path), "org.freedesktop.DBus.Properties")
                connected = device.Get("org.bluez.Device1", "Connected")
                if connected:
                    print(f"Device connected: {path}")
                    return
        time.sleep(1)

def main():
    if os.geteuid() != 0:
        print("This script must be run as root.")
        exit()

    setup_bluetooth()

    print("Bluetooth keyboard setup complete. Waiting for connection...")
    wait_for_connection()

    try:
        ui = create_uinput_device()
        print("Device connected. Sending 'A' periodically...")

        while True:
            ui.write(e.EV_KEY, e.KEY_A, 1)  # Key press
            ui.write(e.EV_KEY, e.KEY_A, 0)  # Key release
            ui.syn()
            print("Sent: A")
            time.sleep(1)

    except Exception as ex:
        print(f"An error occurred: {ex}")

    finally:
        ui.close()
        print("UInput device closed.")

if __name__ == "__main__":
    main()
