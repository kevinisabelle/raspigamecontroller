import os
import time
import dbus
from evdev import UInput, ecodes as e, AbsInfo

# Placeholder function to get rotary encoder value
def get_encoder_value():
    """Simulate getting encoder value. Replace with real GPIO code."""
    return int(time.time() % 255)  # Fake encoder value cycling 0-255

def setup_bluetooth():
    os.system("sudo bluetoothctl agent NoInputNoOutput")
    os.system("sudo bluetoothctl default-agent")
    os.system("sudo bluetoothctl pairable on")
    os.system("sudo bluetoothctl discoverable on")
    os.system("sudo hciconfig hci0 up")
    os.system("sudo hciconfig hci0 class 0x002508")  # Set as a gamepad
    os.system("sudo hciconfig hci0 name 'RPi Gamepad'")
    os.system("sudo sdptool add HID")
    print("Bluetooth setup complete. Ready to pair.")

def create_uinput_device():
    """Create a virtual gamepad using UInput."""
    capabilities = {
        e.EV_ABS: {
            e.ABS_X: AbsInfo(127, 0, 255, 0, 0, 0),  # Joystick X axis (0-255)
        },
        e.EV_KEY: [e.BTN_A],  # Gamepad button A (optional)
    }
    return UInput(capabilities, name="RPi_Gamepad")

def wait_for_connection():
    """Wait for a Bluetooth device to connect."""
    bus = dbus.SystemBus()
    manager = dbus.Interface(bus.get_object("org.bluez", "/"), "org.freedesktop.DBus.ObjectManager")

    adapter_path = None
    for path, interfaces in manager.GetManagedObjects().items():
        if "org.bluez.Adapter1" in interfaces:
            adapter_path = path
            break

    if not adapter_path:
        print("Bluetooth adapter not found.")
        return

    print("Waiting for device to connect...")
    while True:
        objects = manager.GetManagedObjects()
        for path, interfaces in objects.items():
            if "org.bluez.Device1" in interfaces:
                device = dbus.Interface(bus.get_object("org.bluez", path), "org.freedesktop.DBus.Properties")
                connected = device.Get("org.bluez.Device1", "Connected")
                if connected:
                    print(f"Device connected: {path}")
                    print(f"Device name: {device.Get('org.bluez.Device1', 'Name')}")
                    print(f"Device address: {device.Get('org.bluez.Device1', 'Address')}")
                    return
        time.sleep(1)

def main():
    if os.geteuid() != 0:
        print("This script must be run as root.")
        exit()

    setup_bluetooth()
    wait_for_connection()

    try:
        ui = create_uinput_device()
        print("Gamepad connected. Sending joystick movement...")

        while True:
            encoder_value = get_encoder_value()  # Get encoder value
            ui.write(e.EV_ABS, e.ABS_X, encoder_value)  # Send as X-axis input
            ui.syn()
            print(f"Sent Axis X: {encoder_value}")
            time.sleep(0.1)  # Adjust update rate as needed

    except Exception as ex:
        print(f"An error occurred: {ex}")

    finally:
        ui.close()
        print("UInput device closed.")

if __name__ == "__main__":
    main()
