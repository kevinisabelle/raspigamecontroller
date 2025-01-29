import dbus

# Define HID Service UUID
HID_SERVICE_UUID = "00001124-0000-1000-8000-00805f9b34fb"

def register_hid_service():
    bus = dbus.SystemBus()
    manager = dbus.Interface(bus.get_object("org.bluez", "/org/bluez"), "org.freedesktop.DBus.ObjectManager")
    
    adapter_path = None
    for path, interfaces in manager.GetManagedObjects().items():
        if "org.bluez.Adapter1" in interfaces:
            adapter_path = path
            break

    if not adapter_path:
        print("No Bluetooth adapter found!")
        return

    adapter = dbus.Interface(bus.get_object("org.bluez", adapter_path), "org.freedesktop.DBus.Properties")

    # Create HID service
    service = {
        "org.bluez.GattService1": {
            "UUID": HID_SERVICE_UUID,
            "Primary": True
        }
    }

    adapter.Set("org.freedesktop.DBus.Properties", "ServiceRecords", dbus.Array([service], signature="a{sa{sv}}"))

    print("HID service registered successfully!")

if __name__ == "__main__":
    register_hid_service()
