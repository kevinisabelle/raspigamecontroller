#!/usr/bin/env python3
import asyncio
from dbus_next.aio import MessageBus
from dbus_next.constants import BusType, PropertyAccess
from dbus_next import Variant
from dbus_next.service import ServiceInterface, method, dbus_property, signal
from typing import List

AGENT_PATH = "/test/agent"

# --------------------------------------------------------------------------
# Agent for DisplayYesNo Pairing
# --------------------------------------------------------------------------
class Agent(ServiceInterface):
    def __init__(self, path):
        super().__init__('org.bluez.Agent1')
        self._path = path

    @method()
    async def Release(self) -> None:
        print("Agent Released")

    @method()
    async def RequestPasskey(self, device: 'o') -> 'u': # type: ignore
        print("RequestPasskey for device:", device)
        return 0

    @method()
    async def DisplayPasskey(self, device: 'o', passkey: 'u') -> None:
        print("DisplayPasskey for device:", device, passkey)

    @method()
    async def RequestConfirmation(self, device: 'o', passkey: 's') -> None:
        print("Auto-confirming pairing for device:", device, "with passkey:", passkey)

    @method()
    async def RequestPinCode(self, device: 'o') -> 's':
        print("RequestPinCode for device:", device)
        return "0000"

    @method()
    async def RequestAuthorization(self, device: 'o') -> None:
        print("RequestAuthorization for device:", device)

    @method()
    async def AuthorizeService(self, device: 'o', uuid: 's') -> None:
        print("AuthorizeService called for device {} and service UUID {}".format(device, uuid))



# --------------------------------------------------------------------------
# Advertisement for BLE using LEAdvertisingManager1
# --------------------------------------------------------------------------
class Advertisement(ServiceInterface):
    def __init__(self, path, ad_type):
        super().__init__('org.bluez.LEAdvertisement1')
        self.path = path
        self.ad_type = ad_type  # e.g. "peripheral"
        self.service_uuids = []
        self.manufacturer_data = {}
        self.solicit_uuids = []
        self.service_data = {}
        self.include_tx_power = False
        self.appearance = 0x03C4  # Gamepad appearance

    @dbus_property(PropertyAccess.READ)
    def Type(self) -> 's':
        return self.ad_type

    @dbus_property(PropertyAccess.READ)
    def ServiceUUIDs(self) -> 'as': # type: ignore
        return self.service_uuids

    @dbus_property(PropertyAccess.READ)
    def IncludeTxPower(self) -> 'b':
        return self.include_tx_power

    @dbus_property(PropertyAccess.READ)
    def Appearance(self) -> 'q':
        return self.appearance

    @method()
    async def Release(self):
        print(f'{self.path}: Released!')


# --------------------------------------------------------------------------
# Manual GATT Server Code
# --------------------------------------------------------------------------
class ManualService(ServiceInterface):
    def __init__(self, path, characteristics):
        super().__init__('org.bluez.GattService1')
        self.path = path
        self._uuid = "00001812-0000-1000-8000-00805f9b34fb"
        self._primary = True
        self._characteristics = characteristics

    @dbus_property(PropertyAccess.READ)
    def UUID(self) -> 's':
        return self._uuid

    @dbus_property(PropertyAccess.READ)
    def Primary(self) -> 'b':
        return self._primary

    @dbus_property(PropertyAccess.READ)
    def Characteristics(self) -> 'ao':
        return self._characteristics


class ManualCharacteristic(ServiceInterface):
    def __init__(self, path, uuid, flags, service_path, read_handler=None):
        super().__init__('org.bluez.GattCharacteristic1')
        self.path = path
        self._uuid = uuid
        self._flags = flags
        self._service_path = service_path
        self._read_handler = read_handler
        self._descriptors = []  # list of object paths
        self._notifying = False
        self._value = None

    @dbus_property(PropertyAccess.READ)
    def UUID(self) -> 's':
        return self._uuid

    @dbus_property(PropertyAccess.READ)
    def Service(self) -> 'o':
        return self._service_path

    @dbus_property(PropertyAccess.READ)
    def Flags(self) -> 'as': # type: ignore
        return self._flags

    @dbus_property(PropertyAccess.READ)
    def Descriptors(self) -> 'ao':
        return self._descriptors

    @method()
    async def ReadValue(self, options: 'a{sv}') -> 'ay': # type: ignore
        print("ManualCharacteristic ReadValue called with options:", options)
        if self._read_handler:
            return self._read_handler()
        print("Default ReadValue called")
        return [0]

    @method()
    async def WriteValue(self, value, options):
        try:
            print("WriteValue called with value:", [hex(v) for v in value])
            self._value = value
            if len(value) > 0:
                report_id = value[0]
                if report_id == 0x02:  # Output report
                    led_state = value[1] & 0x01
                    print(f"Output report received: LED state = {led_state}")
        except Exception as e:
            print("Error in WriteValue:", e)
            raise e

    @signal()
    def PropertiesChanged(self, interface: 's', changed: 'a{sv}', invalidated: 'as'): # type: ignore
        # Signal: no action needed
        return

    @method()
    async def StartNotify(self):
        print("StartNotify called for", self.path)
        if not self._notifying:
            self._notifying = True
            if self._read_handler:
                self._value = self._read_handler()
                self.PropertiesChanged("org.bluez.GattCharacteristic1", {"Value": self._value}, [])

    @method()
    async def StopNotify(self):
        print("StopNotify called for", self.path)
        self._notifying = False

    def send_notification(self, value):
        if self._notifying:
            self._value = value
            self.PropertiesChanged("org.bluez.GattCharacteristic1", {"Value": self._value}, [])

    def add_descriptor(self, descriptor_path):
        self._descriptors.append(descriptor_path)


class ManualDescriptor(ServiceInterface):
    def __init__(self, path, characteristic_path, uuid, value):
        super().__init__('org.bluez.GattDescriptor1')
        self.path = path
        self._uuid = uuid
        self._value = value
        self._characteristic_path = characteristic_path

    @dbus_property(PropertyAccess.READ)
    def UUID(self) -> 's':
        return self._uuid

    @dbus_property(PropertyAccess.READ)
    def Characteristic(self) -> 'o':
        return self._characteristic_path

    @method()
    async def ReadValue(self, options: 'a{sv}') -> 'ay': # type: ignore
        return self._value


# --------------------------------------------------------------------------
# Handlers for characteristic reads
# --------------------------------------------------------------------------
def report_map_read_handler():
    report_map = [
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
    ]
    return report_map

def gamepad_report_read_handler():
    return [0x00, 0x00]

def hid_info_read_handler():
    # bcdHID=0x0111, country code=0, flags=0x03 (remote wake + normally connectable)
    return [0x11, 0x01, 0x00, 0x03]

def protocol_mode_read_handler():
    return [0x01]


# --------------------------------------------------------------------------
# Registration helpers
# --------------------------------------------------------------------------
async def register_agent(bus, agent, capability="KeyboardDisplay"):
    try:
        introspection = await bus.introspect("org.bluez", "/org/bluez")
        obj = bus.get_proxy_object("org.bluez", "/org/bluez", introspection)
        agent_manager = obj.get_interface("org.bluez.AgentManager1")
        await agent_manager.call_register_agent(AGENT_PATH, capability)
        await agent_manager.call_request_default_agent(AGENT_PATH)
        print(f"Agent registered as default with {capability} capability")
    except Exception as e:
        print("Failed to register agent:", e)

async def register_advertisement(bus, advertisement):
    try:
        introspection = await bus.introspect("org.bluez", "/org/bluez/hci0")
        adapter = bus.get_proxy_object("org.bluez", "/org/bluez/hci0", introspection)
        ad_manager = adapter.get_interface("org.bluez.LEAdvertisingManager1")
        await ad_manager.call_register_advertisement(advertisement.path, {})
        print("Advertisement registered")
    except Exception as e:
        print("Failed to register advertisement:", e)

async def register_hid_profile(bus, object_path, sdp_record):
    try:
        introspection = await bus.introspect("org.bluez", object_path)
        profile_obj = bus.get_proxy_object("org.bluez", object_path, introspection)
        profile = profile_obj.get_interface("org.bluez.Profile1")
        try:
            await profile.call_release()
        except Exception as e:
            print("Failed to release HID profile (maybe not registered yet):", e)
        await profile.call_register(object_path, "00001124-0000-1000-8000-00805f9b34fb", sdp_record)
        print("HID Profile registered with SDP record")
        return profile
    except Exception as e:
        print("Failed to register HID profile:", e)
        return None


# --------------------------------------------------------------------------
# Main: register objects on D-Bus
# --------------------------------------------------------------------------
async def main():
    print("Starting manual GATT server with BLE advertisement and DisplayYesNo pairing agent...")
    bus = await MessageBus(bus_type=BusType.SYSTEM).connect()

    # Register pairing agent
    agent = Agent(AGENT_PATH)
    bus.export(AGENT_PATH, agent)
    await register_agent(bus, agent)

    # Set adapter properties
    try:
        introspection = await bus.introspect("org.bluez", "/org/bluez/hci0")
        adapter_obj = bus.get_proxy_object("org.bluez", "/org/bluez/hci0", introspection)
        props_iface = adapter_obj.get_interface("org.freedesktop.DBus.Properties")
        await props_iface.call_set("org.bluez.Adapter1", "Powered", Variant('b', True))
        await props_iface.call_set("org.bluez.Adapter1", "Discoverable", Variant('b', True))
        await props_iface.call_set("org.bluez.Adapter1", "Pairable", Variant('b', True))
        try:
            await props_iface.call_set("org.bluez.Adapter1", "MaxConnectionInterval", Variant('q', 16))
            await props_iface.call_set("org.bluez.Adapter1", "ConnectionLatency", Variant('q', 0))
            await props_iface.call_set("org.bluez.Adapter1", "SupervisionTimeout", Variant('q', 500))
        except Exception as e:
            print("Warning: Could not set connection parameters:", e)
        print("Adapter set to Powered, Discoverable, and Pairable")
    except Exception as e:
        print("Error setting adapter properties:", e)

    # Register HID Profile (optional)
    sdp_record = {
        "ServiceRecordHandle": 0x10006,
        "ServiceClassIDList": ["00001812-0000-1000-8000-00805f9b34fb"],
        "ProtocolDescriptorList": [
            ["L2CAP", 0x0011],
            ["L2CAP", 0x0013],
            ["HIDP", 0x0100]
        ],
        "AttributeList": [
            ["VersionNumberList", 0x0100],
            ["ServiceName", "HID Gamepad"],
            ["ServiceDescription", "Bluetooth HID Gamepad"],
            ["ProviderName", "BlueZ"],
            ["DeviceReleaseNumber", 0x0100],
            ["ParserVersion", 0x0111],
            ["DeviceSubclass", 0x08],
            ["CountryCode", 0x00],
            ["VirtualCable", True],
            ["BootDevice", False],
            ["SupervisionTimeout", 0x0C80],
            ["NormallyConnectable", True],
            ["ReconnectInitiate", True]
        ]
    }
    hid_profile = await register_hid_profile(bus, "/org/bluez/hid/profile0", sdp_record)

    # Register HID service (GATT)
    hid_service = ManualService("/org/bluez/example/service0", [])
    bus.export("/org/bluez/example/service0", hid_service)
    # print("HID Service registered with UUID 00001812-0000-1000-8000-00805f9b34fb")

    # Register characteristics
    report_map_char = ManualCharacteristic(
        "/org/bluez/example/service0/char0",
        "00002A4B-0000-1000-8000-00805f9b34fb",
        ["read"],
        "/org/bluez/example/service0",
        read_handler=report_map_read_handler
    )
    bus.export("/org/bluez/example/service0/char0", report_map_char)

    gamepad_report_char = ManualCharacteristic(
        "/org/bluez/example/service0/char1",
        "00002A4D-0000-1000-8000-00805f9b34fb",
        ["read", "notify", "write-without-response"],
        "/org/bluez/example/service0",
        read_handler=gamepad_report_read_handler
    )
    bus.export("/org/bluez/example/service0/char1", gamepad_report_char)

    hid_info_char = ManualCharacteristic(
        "/org/bluez/example/service0/char2",
        "00002A4A-0000-1000-8000-00805f9b34fb",
        ["read"],
        "/org/bluez/example/service0",
        read_handler=hid_info_read_handler
    )
    bus.export("/org/bluez/example/service0/char2", hid_info_char)

    protocol_mode_char = ManualCharacteristic(
        "/org/bluez/example/service0/char3",
        "00002A4E-0000-1000-8000-00805f9b34fb",
        ["read", "write", "write-without-response"],
        "/org/bluez/example/service0",
        read_handler=protocol_mode_read_handler
    )
    bus.export("/org/bluez/example/service0/char3", protocol_mode_char)

    # Add descriptors to gamepad report characteristic
    report_ref_desc = ManualDescriptor(
        "/org/bluez/example/service0/char1/desc0",
        "/org/bluez/example/service0/char1",
        "00002908-0000-1000-8000-00805f9b34fb",
        [0x01, 0x01]
    )
    bus.export("/org/bluez/example/service0/char1/desc0", report_ref_desc)

    ccc_desc = ManualDescriptor(
        "/org/bluez/example/service0/char1/desc1",
        "/org/bluez/example/service0/char1",
        "00002902-0000-1000-8000-00805f9b34fb",
        [0x00, 0x01]
    )
    bus.export("/org/bluez/example/service0/char1/desc1", ccc_desc)

    output_report_ref = ManualDescriptor(
        "/org/bluez/example/service0/char1/desc2",
        "/org/bluez/example/service0/char1",
        "00002908-0000-1000-8000-00805f9b34fb",
        [0x00, 0x02]
    )
    bus.export("/org/bluez/example/service0/char1/desc2", output_report_ref)

    gamepad_report_char.add_descriptor(report_ref_desc.path)
    gamepad_report_char.add_descriptor(ccc_desc.path)
    gamepad_report_char.add_descriptor(output_report_ref.path)

    # Add characteristic object paths to HID service
    hid_service._characteristics.extend([
        report_map_char.path,
        gamepad_report_char.path,
        hid_info_char.path,
        protocol_mode_char.path
    ])

    print("Manual GATT server running with:")
    print("  - Report Map Characteristic at /org/bluez/example/service0/char0")
    print("  - Gamepad Report Characteristic at /org/bluez/example/service0/char1")
    print("  - HID Information Characteristic at /org/bluez/example/service0/char2")
    print("  - Protocol Mode Characteristic at /org/bluez/example/service0/char3")

    # --- Create and register BLE Advertisement ---
    advertisement = Advertisement("/org/bluez/example/advertisement0", "peripheral")
    advertisement.service_uuids.append("00001812-0000-1000-8000-00805f9b34fb")
    advertisement.include_tx_power = True
    bus.export(advertisement.path, advertisement)
    await register_advertisement(bus, advertisement)

    # Run forever
    await asyncio.Future()

if __name__ == "__main__":
    asyncio.run(main())
