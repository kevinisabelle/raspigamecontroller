import dbus
import dbus.service
import dbus.mainloop.glib
from gi.repository import GLib
import constants
from bluezeclasses import (
    Advertisement,
    ManualCharacteristic,
    ManualDescriptor,
    ManualService,
    Agent,
)

# Handlers (renvoyant des tableaux d’octets)
def report_map_read_handler():
    print("Report Map read handler called")
    return bytes([
        0x05, 0x01,       # Usage Page (Generic Desktop)
        0x09, 0x05,       # Usage (Game Pad)
        0xA1, 0x01,       # Collection (Application)
        0x85, 0x01,       # Report ID (1)
        0x05, 0x09,       # Usage Page (Button)
        0x19, 0x01,       # Usage Minimum (Button 1)
        0x29, 0x01,       # Usage Maximum (Button 1)
        0x15, 0x00,       # Logical Minimum (0)
        0x25, 0x01,       # Logical Maximum (1)
        0x75, 0x01,       # Report Size (1)
        0x95, 0x01,       # Report Count (1)
        0x81, 0x02,       # Input (Data, Variable, Absolute)
        0x75, 0x01,       # Report Size (1)
        0x95, 0x07,       # Report Count (7)
        0x81, 0x03,       # Input (Constant)
        0x05, 0x01,       # Usage Page (Generic Desktop)
        0x09, 0x30,       # Usage (X)
        0x15, 0x81,       # Logical Minimum (-127)
        0x25, 0x7F,       # Logical Maximum (127)
        0x75, 0x08,       # Report Size (8)
        0x95, 0x01,       # Report Count (1)
        0x81, 0x02,       # Input (Data, Variable, Absolute)
        0x85, 0x02,       # Report ID (2)
        0x09, 0x48,       # Usage (LED)
        0x15, 0x00,       # Logical Minimum (0)
        0x25, 0x01,       # Logical Maximum (1)
        0x75, 0x01,       # Report Size (1)
        0x95, 0x01,       # Report Count (1)
        0x91, 0x02,       # Output (Data, Variable, Absolute)
        0x75, 0x07,       # Report Size (7)
        0x95, 0x01,       # Report Count (1)
        0x91, 0x03,       # Output (Constant)
        0xC0              # End Collection
    ])

def gamepad_report_read_handler():
    print("Gamepad Report read handler called")
    return bytes([0x00, 0x00])

def hid_info_read_handler():
    print("HID Information read handler called")
    return bytes([0x11, 0x01, 0x00, 0x03])

def protocol_mode_read_handler():
    print("Protocol Mode read handler called")
    return bytes([0x01])


def register_advertisement(bus, advertisement):
    print("Registering advertisement...")
    adapter = bus.get_object(constants.BLUEZ_SERVICE, constants.ADAPTER_PATH)
    ad_manager = dbus.Interface(adapter, "org.bluez.LEAdvertisingManager1")
    # Utiliser des gestionnaires pour éviter le blocage en cas de non-réponse
    def reply_handler(*args):
        print("Advertisement registered (reply_handler)")
    def error_handler(e):
        print("Advertisement registration error (error_handler):", e)
    ad_manager.RegisterAdvertisement(
        advertisement.get_path(),
        {},
        reply_handler=reply_handler,
        error_handler=error_handler
    )

def register_agent(bus, capability="KeyboardDisplay"):
    try:
        print("Registering agent...")
        agent_manager_obj = bus.get_object(constants.BLUEZ_SERVICE, constants.BLUEZ_SERVICE_PATH)
        agent_manager = dbus.Interface(agent_manager_obj, "org.bluez.AgentManager1")
        agent_manager.RegisterAgent(constants.AGENT_PATH, capability)
        agent_manager.RequestDefaultAgent(constants.AGENT_PATH)
        print("Agent registered as default with {} capability".format(capability))
        try:
            adapter_obj = bus.get_object(constants.BLUEZ_SERVICE, constants.ADAPTER_PATH)
            props_iface = dbus.Interface(adapter_obj, "org.freedesktop.DBus.Properties")
            props_iface.Set("org.bluez.Adapter1", "Powered", dbus.Boolean(True))
            props_iface.Set("org.bluez.Adapter1", "Discoverable", dbus.Boolean(True))
            props_iface.Set("org.bluez.Adapter1", "Pairable", dbus.Boolean(True))
        except Exception as e:
            print("Error setting adapter properties:", e)
    except Exception as e:
        print("Failed to register agent:", e)
        raise

def register_profile(bus, service):
    print("Registering profile...")
    manager = dbus.Interface(
        bus.get_object(constants.BLUEZ_SERVICE, constants.BLUEZ_SERVICE_PATH),
        "org.bluez.ProfileManager1"
    )
    manager.RegisterProfile(service.object_path, constants.GATT_SERVICE_HID_UUID, {})
    print("Profile registered")

class GamePadAdvertisment(Advertisement):
    def __init__(self, bus, index):
        Advertisement.__init__(self, bus, index, 'peripheral')
        self.add_service_uuid('1812')
        self.add_local_name('KiGP')
        self.appearance = constants.ADV_APPEARANCE_GAMEPAD
        self.include_tx_power = True

def main():
    # Initialiser la boucle principale de GLib pour dbus
    dbus.mainloop.glib.DBusGMainLoop(set_as_default=True)
    print("Starting GamepadKi...")
    bus = dbus.SystemBus()

    # Créer et exporter l'agent
    agent = Agent(bus, constants.AGENT_PATH)

    mainloop = GLib.MainLoop()

    register_agent(bus)

    characteristics = []

    # Créer le service HID
    hid_service = ManualService(bus, constants.SERVICE_PATH, characteristics, constants.GATT_SERVICE_HID_UUID)
    print("HID service created at", constants.SERVICE_PATH)
    
    # Créer et exporter les caractéristiques
    report_map_char = ManualCharacteristic(
        bus,
        f"{constants.SERVICE_PATH}/char0",
        constants.GATT_REPORT_MAP_UUID,
        ["read"],
        constants.SERVICE_PATH,
        read_handler=report_map_read_handler,
    )

    gamepad_report_char = ManualCharacteristic(
        bus,
        f"{constants.SERVICE_PATH}/char1",
        constants.GATT_REPORT_UUID,
        ["read", "notify", "write-without-response"],
        constants.SERVICE_PATH,
        read_handler=gamepad_report_read_handler,
    )

    hid_info_char = ManualCharacteristic(
        bus,
        f"{constants.SERVICE_PATH}/char2",
        constants.GATT_HID_INFORMATION_UUID,
        ["read"],
        constants.SERVICE_PATH,
        read_handler=hid_info_read_handler,
    )

    protocol_mode_char = ManualCharacteristic(
        bus,
        f"{constants.SERVICE_PATH}/char3",
        constants.GATT_PROTOCOL_MODE_UUID,
        ["read", "write", "write-without-response"],
        constants.SERVICE_PATH,
        read_handler=protocol_mode_read_handler,
    )

    # Créer et exporter les descripteurs pour la caractéristique du rapport gamepad
    report_ref_desc = ManualDescriptor(
        bus,
        f"{constants.SERVICE_PATH}/char1/desc0",
        f"{constants.SERVICE_PATH}/char1",
        constants.GATT_DESC_REPORT_REFERENCE_UUID,
        bytes([0x01, 0x01])
    )

    ccc_desc = ManualDescriptor(
        bus,
        f"{constants.SERVICE_PATH}/char1/desc1",
        f"{constants.SERVICE_PATH}/char1",
        constants.GATT_DESC_CLIENT_DESCRIPTOR_UUID,
        bytes([0x00, 0x01])
    )

    output_report_ref = ManualDescriptor(
        bus,
        f"{constants.SERVICE_PATH}/char1/desc2",
        f"{constants.SERVICE_PATH}/char1",
        constants.GATT_DESC_REPORT_REFERENCE_UUID,
        bytes([0x00, 0x02])
    )

    # Ajouter les chemins des descripteurs à la caractéristique gamepad
    gamepad_report_char.AddDescriptor(f"{constants.SERVICE_PATH}/char1/desc0")
    gamepad_report_char.AddDescriptor(f"{constants.SERVICE_PATH}/char1/desc1")
    gamepad_report_char.AddDescriptor(f"{constants.SERVICE_PATH}/char1/desc2")

    # Ajouter les chemins des caractéristiques au service HID
    characteristics.extend([
        report_map_char.object_path,
        gamepad_report_char.object_path,
        hid_info_char.object_path,
        protocol_mode_char.object_path,
    ])

    print("Manual GATT server running with:")
    print(f"  - Report Map Characteristic at {constants.SERVICE_PATH}/char0")
    print(f"  - Gamepad Report Characteristic at {constants.SERVICE_PATH}/char1")
    print(f"  - HID Information Characteristic at {constants.SERVICE_PATH}/char2")
    print(f"  - Protocol Mode Characteristic at {constants.SERVICE_PATH}/char3")

    # Créer et exporter la publicité BLE
    advertisement = GamePadAdvertisment(bus, 0)

    # Enregistrer la publicité auprès de BlueZ
    register_advertisement(bus, advertisement)

    # register_profile(bus, hid_service)
    
   

    # Boucle principale GLib pour garder le service en vie
    
    try:
        mainloop.run()
    except KeyboardInterrupt:
        print("Exiting...")

if __name__ == "__main__":
    main()
