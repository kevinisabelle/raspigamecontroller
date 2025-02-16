#!/usr/bin/env python3
import dbus
import dbus.exceptions
import dbus.mainloop.glib
import dbus.service
from gi.repository import GLib

AGENT_PATH = "/test/agent"

# -----------------------------------------------------------------------------  
# Agent for JustWorks Pairing  
# -----------------------------------------------------------------------------
class Agent(dbus.service.Object):
    AGENT_INTERFACE = "org.bluez.Agent1"

    def __init__(self, bus, path=AGENT_PATH):
        dbus.service.Object.__init__(self, bus, path)

    @dbus.service.method(AGENT_INTERFACE, in_signature="", out_signature="")
    def Release(self):
        print("Agent Released")

    @dbus.service.method(AGENT_INTERFACE, in_signature="o", out_signature="u")
    def RequestPasskey(self, device):
        # Not used in JustWorks; return a dummy passkey.
        print("RequestPasskey for device:", device)
        return dbus.UInt32(0)

    @dbus.service.method(AGENT_INTERFACE, in_signature="ou", out_signature="")
    def DisplayPasskey(self, device, passkey):
        # No display needed for JustWorks.
        print("DisplayPasskey for device:", device, passkey)

    @dbus.service.method(AGENT_INTERFACE, in_signature="ou", out_signature="")
    def RequestConfirmation(self, device, passkey):
        print("Auto-confirming pairing for device:", device, "with passkey:", passkey)
        # Simply return without raising an exception.
        return

    @dbus.service.method(AGENT_INTERFACE, in_signature="o", out_signature="s")
    def RequestPinCode(self, device):
        # Return a dummy PIN if requested.
        print("RequestPinCode for device:", device)
        return "0000"

    @dbus.service.method(AGENT_INTERFACE, in_signature="o", out_signature="")
    def RequestAuthorization(self, device):
        # Auto-authorize pairing.
        print("RequestAuthorization for device:", device)
        return
    
    # --- Add the missing method ---
    @dbus.service.method(AGENT_INTERFACE, in_signature="os", out_signature="")
    def AuthorizeService(self, device, uuid):
        print("AuthorizeService called for device {} and service UUID {}".format(device, uuid))
        # Auto-authorize the service request.
        return

def register_agent(bus, agent, capability="KeyboardDisplay"):
    try:
        manager = dbus.Interface(
            bus.get_object("org.bluez", "/org/bluez"),
            "org.bluez.AgentManager1")
        manager.RegisterAgent(AGENT_PATH, capability)
        manager.RequestDefaultAgent(AGENT_PATH)
        print("Agent registered as default with {} capability".format(capability))
    except Exception as e:
        print("Failed to register agent:", e)


# -----------------------------------------------------------------------------  
# Advertisement for BLE using LEAdvertisingManager1  
# -----------------------------------------------------------------------------
class Advertisement(dbus.service.Object):
    PATH_BASE = '/org/bluez/example/advertisement'
    
    def __init__(self, bus, index, ad_type):
        self.path = self.PATH_BASE + str(index)
        self.bus = bus
        self.ad_type = ad_type  # e.g., "peripheral"
        self.service_uuids = []
        self.manufacturer_data = {}
        self.solicit_uuids = []
        self.service_data = {}
        self.include_tx_power = False
        dbus.service.Object.__init__(self, bus, self.path)
    
    def get_properties(self):
        properties = {
            'Type': self.ad_type,
            'ServiceUUIDs': dbus.Array(self.service_uuids, signature='s'),
            'IncludeTxPower': dbus.Boolean(True),
            'Appearance': dbus.UInt16(0x03C4)  # Gamepad appearance
        }
        return properties
    
    def get_path(self):
        print('get_path: %s' % dbus.ObjectPath(self.path))
        return dbus.ObjectPath(self.path)
    
    @dbus.service.method('org.freedesktop.DBus.Properties',
                         in_signature='s', out_signature='a{sv}')
    def GetAll(self, interface):
        if interface != 'org.bluez.LEAdvertisement1':
            raise dbus.exceptions.DBusException('org.freedesktop.DBus.Error.InvalidArgs',
                                                'Invalid interface %s' % interface)
        print('GetAll: %s' % interface)
        return self.get_properties()['org.bluez.LEAdvertisement1']
    
    @dbus.service.method('org.bluez.LEAdvertisement1', in_signature='', out_signature='')
    def Release(self):
        print('%s: Released!' % self.path)


def register_advertisement(bus, advertisement):
    try:
        adapter = bus.get_object('org.bluez', '/org/bluez/hci0')
        ad_manager = dbus.Interface(adapter, 'org.bluez.LEAdvertisingManager1')
        ad_manager.RegisterAdvertisement(advertisement.get_path(), {},
                                         reply_handler=lambda: print("Advertisement registered"),
                                         error_handler=lambda e: print("Failed to register advertisement:", e))
    except Exception as e:
        print("Exception while registering advertisement:", e)


# -----------------------------------------------------------------------------  
# (Existing) Manual GATT Server Code  
# -----------------------------------------------------------------------------
class ManualService(dbus.service.Object):
    introspection_xml = """<node>
  <interface name="org.bluez.GattService1">
    <property name="UUID" type="s" access="read"/>
    <property name="Primary" type="b" access="read"/>
    <property name="Characteristics" type="ao" access="read"/>
  </interface>
</node>
"""
    def __init__(self, bus, object_path, characteristics):
        self.object_path = object_path
        self.characteristics = characteristics
        dbus.service.Object.__init__(self, bus, object_path)

    def __introspect__(self):
        return self.introspection_xml

    @dbus.service.method("org.freedesktop.DBus.Properties",
                         in_signature="ss", out_signature="v")
    def Get(self, interface, prop):
        print("ManualService Get called with interface={} and prop={}".format(interface, prop))
        if interface == "org.bluez.GattService1":
            if prop == "UUID":
                return "00001812-0000-1000-8000-00805f9b34fb"  # HID Service UUID
            elif prop == "Primary":
                return True
            elif prop == "Characteristics":
                return self.characteristics
        raise dbus.exceptions.DBusException("UnknownProperty", "Property not found")


class ManualCharacteristic(dbus.service.Object):
    introspection_xml = """<node>
  <interface name="org.bluez.GattCharacteristic1">
    <method name="ReadValue">
      <arg name="options" type="a{sv}" direction="in"/>
      <arg name="value" type="ay" direction="out"/>
    </method>
    <method name="WriteValue">
      <arg name="value" type="ay" direction="in"/>
      <arg name="options" type="a{sv}" direction="in"/>
    </method>
    <method name="StartNotify"/>
    <method name="StopNotify"/>
    <property name="UUID" type="s" access="read"/>
    <property name="Service" type="o" access="read"/>
    <property name="Flags" type="as" access="read"/>
  </interface>
</node>
"""
    def __init__(self, bus, object_path, uuid, flags, service_path, read_handler=None):
        self.object_path = object_path
        self._uuid = uuid
        self._flags = flags
        self._service_path = service_path
        self._read_handler = read_handler
        self._descriptors = []  # Add this line
        self._notifying = False
        self._bus = bus
        self._value = None
        dbus.service.Object.__init__(self, bus, object_path)
    
    def add_descriptor(self, descriptor):
        self._descriptors.append(descriptor.path)

    def __introspect__(self):
        return self.introspection_xml

    @dbus.service.method("org.bluez.GattCharacteristic1",
                         in_signature="a{sv}", out_signature="ay")
    def ReadValue(self, options):
        print("ManualCharacteristic ReadValue called with options:", options)
        if self._read_handler:
            return self._read_handler()
        print("Default ReadValue called")
        return [dbus.Byte(0x00)]

    @dbus.service.method("org.bluez.GattCharacteristic1",
                    in_signature="aya{sv}", out_signature="")
    def WriteValue(self, value, options):
        try:
            print("WriteValue called with value:", [hex(v) for v in value])
            self._value = value
            # Store the written value
            if len(value) > 0:
                report_id = value[0]
                if report_id == 0x02:  # Output report
                    led_state = value[1] & 0x01
                    print(f"Output report received: LED state = {led_state}")
        except Exception as e:
            print("Error in WriteValue:", e)
            raise dbus.exceptions.DBusException(str(e))

    @dbus.service.signal("org.freedesktop.DBus.Properties",
                     signature="sa{sv}as")
    def PropertiesChanged(self, interface, changed, invalidated):
        pass

    @dbus.service.method("org.bluez.GattCharacteristic1")
    def StartNotify(self):
        print("StartNotify called for {}".format(self.object_path))
        if not self._notifying:
            self._notifying = True
            # Send initial notification
            if self._read_handler:
                self._value = self._read_handler()
                self.PropertiesChanged(
                    "org.bluez.GattCharacteristic1",
                    {"Value": self._value}, [])

    @dbus.service.method("org.bluez.GattCharacteristic1")
    def StopNotify(self):
        print("StopNotify called for {}".format(self.object_path))
        if self._notifying:
            self._notifying = False

    def send_notification(self, value):
        if self._notifying:
            self._value = value
            self.PropertiesChanged(
                "org.bluez.GattCharacteristic1",
                {"Value": self._value}, [])

    @dbus.service.method("org.freedesktop.DBus.Properties",
                         in_signature="ss", out_signature="v")
    def Get(self, interface, prop):
        print("ManualCharacteristic Get called with interface={} and prop={}".format(interface, prop))
        if interface == "org.bluez.GattCharacteristic1":
            if prop == "UUID":
                return self._uuid
            elif prop == "Service":
                return self._service_path
            elif prop == "Flags":
                return self._flags
            elif prop == "Descriptors":  # Add this
                return self._descriptors
        raise dbus.exceptions.DBusException("UnknownProperty", "Property not found")


class ManualDescriptor(dbus.service.Object):
    """Add descriptor support for Report Reference"""
    
    introspection_xml = """<node>
        <interface name="org.bluez.GattDescriptor1">
            <method name="ReadValue">
                <arg name="options" type="a{sv}" direction="in"/>
                <arg name="value" type="ay" direction="out"/>
            </method>
            <property name="UUID" type="s" access="read"/>
            <property name="Characteristic" type="o" access="read"/>
        </interface>
    </node>"""
    
    def __init__(self, bus, path, characteristic_path, uuid, value):
        self.path = path
        self._uuid = uuid
        self._value = value
        self._characteristic_path = characteristic_path
        dbus.service.Object.__init__(self, bus, path)
    
    def __introspect__(self):
        return self.introspection_xml
    
    @dbus.service.method("org.bluez.GattDescriptor1",
                         in_signature="a{sv}", out_signature="ay")
    def ReadValue(self, options):
        return self._value
    
    @dbus.service.method("org.freedesktop.DBus.Properties",
                         in_signature="ss", out_signature="v")
    def Get(self, interface, prop):
        if interface == "org.bluez.GattDescriptor1":
            if prop == "UUID":
                return self._uuid
            elif prop == "Characteristic":
                return self._characteristic_path
        raise dbus.exceptions.DBusException("UnknownProperty")


def report_map_read_handler():
    """
    Returns the HID Report Map descriptor for a gamepad with:
    - One button (1 bit) with 7 bits padding
    - One axis (X axis) as an 8-bit signed value
    - One output report for LED/rumble
    """
    report_map = [
        0x05, 0x01,       # Usage Page (Generic Desktop)
        0x09, 0x05,       # Usage (Game Pad)
        0xA1, 0x01,       # Collection (Application)
        
        # Input report
        0x85, 0x01,       # Report ID (1)
        0x05, 0x09,       # Usage Page (Button)
        0x19, 0x01,       # Usage Minimum (Button 1)
        0x29, 0x01,       # Usage Maximum (Button 1)
        0x15, 0x00,       # Logical Minimum (0)
        0x25, 0x01,       # Logical Maximum (1)
        0x75, 0x01,       # Report Size (1)
        0x95, 0x01,       # Report Count (1)
        0x81, 0x02,       # Input (Data, Variable, Absolute)
        # Padding: 7 bits
        0x75, 0x01,       # Report Size (1)
        0x95, 0x07,       # Report Count (7)
        0x81, 0x03,       # Input (Constant)
        # X axis
        0x05, 0x01,       # Usage Page (Generic Desktop)
        0x09, 0x30,       # Usage (X)
        0x15, 0x81,       # Logical Minimum (-127)
        0x25, 0x7F,       # Logical Maximum (127)
        0x75, 0x08,       # Report Size (8)
        0x95, 0x01,       # Report Count (1)
        0x81, 0x02,       # Input (Data, Variable, Absolute)

        # Output report
        0x85, 0x02,       # Report ID (2)
        0x09, 0x48,       # Usage (LED)
        0x15, 0x00,       # Logical Minimum (0)
        0x25, 0x01,       # Logical Maximum (1)
        0x75, 0x01,       # Report Size (1)
        0x95, 0x01,       # Report Count (1)
        0x91, 0x02,       # Output (Data, Variable, Absolute)
        # Padding
        0x75, 0x07,       # Report Size (7)
        0x95, 0x01,       # Report Count (1)
        0x91, 0x03,       # Output (Constant)
        
        0xC0              # End Collection
    ]
    return [dbus.Byte(b) for b in report_map]

def gamepad_report_read_handler():
    """
    Returns a dummy gamepad report.
    Byte 0: Button state (0x00 = not pressed, 0x01 = pressed)
    Byte 1: Axis value (signed 8-bit; here fixed to 0x00)
    """
    return [dbus.Byte(0x00), dbus.Byte(0x00)]

def hid_info_read_handler():
    """
    Returns the HID Information characteristic value.
    - Byte 0-1: bcdHID (HID version 1.11)
    - Byte 2: bCountryCode (0x00 = Not Localized)
    - Byte 3: Flags (bit 0: RemoteWake = 1, bit 1: NormallyConnectable = 1)
    """
    return [
        dbus.Byte(0x11),  # HID version 1.11 (LSB)
        dbus.Byte(0x01),  # HID version 1.11 (MSB)
        dbus.Byte(0x00),  # Not localized
        dbus.Byte(0x03)   # Remote wake + normally connectable
    ]

def protocol_mode_read_handler():
    """
    Returns the Protocol Mode characteristic value.
    0x01 = Report Protocol Mode (default)
    """
    return [dbus.Byte(0x01)]

def register_hid_profile(bus, object_path, sdp_record):
    try:
        profile = dbus.Interface(
            bus.get_object("org.bluez", object_path),
            "org.bluez.Profile1")
        profile.Release()
    except Exception as e:
        print("Failed to release HID profile:", e)

    try:
        profile = dbus.Interface(
            bus.get_object("org.bluez", object_path),
            "org.bluez.Profile1")
        profile.Register(object_path, "00001124-0000-1000-8000-00805f9b34fb", sdp_record)
        print("HID Profile registered with SDP record")
        return profile
    except Exception as e:
        print("Failed to register HID profile:", e)
        return None

# -----------------------------------------------------------------------------  
# Main: register objects manually on D-Bus with our custom introspection  
# -----------------------------------------------------------------------------
def main():
    print("Starting manual GATT server with BLE advertisement and DisplayYesNo pairing agent...")
    dbus.mainloop.glib.DBusGMainLoop(set_as_default=True)
    bus = dbus.SystemBus()

    # Register our DisplayYesNo pairing agent
    agent = Agent(bus)
    register_agent(bus, agent, capability="KeyboardDisplay")

    # Enable adapter pairing and discovery (if not already enabled)
    try:
        adapter = bus.get_object("org.bluez", "/org/bluez/hci0")
        props_iface = dbus.Interface(adapter, "org.freedesktop.DBus.Properties")
        props_iface.Set("org.bluez.Adapter1", "Powered", dbus.Boolean(True))
        props_iface.Set("org.bluez.Adapter1", "Discoverable", dbus.Boolean(True))
        props_iface.Set("org.bluez.Adapter1", "Pairable", dbus.Boolean(True))
        try:
            props_iface.Set("org.bluez.Adapter1", "MinConnectionInterval", dbus.UInt16(6))  # 7.5ms
            props_iface.Set("org.bluez.Adapter1", "MaxConnectionInterval", dbus.UInt16(16)) # 20ms
            props_iface.Set("org.bluez.Adapter1", "ConnectionLatency", dbus.UInt16(0))
            props_iface.Set("org.bluez.Adapter1", "SupervisionTimeout", dbus.UInt16(500))  # 5s
        except Exception as e:
            print("Warning: Could not set connection parameters:", e)
        print("Adapter set to Powered, Discoverable, and Pairable")
    except Exception as e:
        print("Error setting adapter properties:", e)

    # Ensure HID service is registered before other characteristics
    hid_service = ManualService(
        bus, 
        "/org/bluez/example/service0", 
        "00001812-0000-1000-8000-00805f9b34fb"  # HID Service UUID
    )

    # Add service to SDP database
    sdp_record = {
        "ServiceRecordHandle": 0x10006,
        "ServiceClassIDList": ["00001812-0000-1000-8000-00805f9b34fb"],
        "ProtocolDescriptorList": [
            ["L2CAP", 0x0011],  # HID Control PSM
            ["L2CAP", 0x0013],  # HID Interrupt PSM
            ["HIDP", 0x0100]    # HID Profile
        ],
        "AttributeList": [
            ["VersionNumberList", 0x0100],
            ["ServiceName", "HID Gamepad"],
            ["ServiceDescription", "Bluetooth HID Gamepad"],
            ["ProviderName", "BlueZ"],
            ["DeviceReleaseNumber", 0x0100],
            ["ParserVersion", 0x0111],    # HID 1.11
            ["DeviceSubclass", 0x08],     # Gamepad
            ["CountryCode", 0x00],
            ["VirtualCable", True],
            ["BootDevice", False],
            ["SupervisionTimeout", 0x0C80],
            ["NormallyConnectable", True],
            ["ReconnectInitiate", True]
        ]
    }

    # Register the HID service with the SDP database
    hid_profile = register_hid_profile(bus, "/org/bluez/hid/profile0", sdp_record)
    print("HID Profile registered with SDP record")

    # Add Report Map characteristic
    report_map_char = ManualCharacteristic(
        bus,
        "/org/bluez/example/service0/char0",
        "00002A4B-0000-1000-8000-00805f9b34fb",  # Report Map UUID
        ["read"],
        "/org/bluez/example/service0",
        read_handler=report_map_read_handler
    )

    # Add Report characteristic with proper descriptors
    gamepad_report_char = ManualCharacteristic(
        bus,
        "/org/bluez/example/service0/char1",
        "00002A4D-0000-1000-8000-00805f9b34fb",  # Report UUID
        ["read", "notify", "write-without-response"],
        "/org/bluez/example/service0",
        read_handler=gamepad_report_read_handler
    )

    # Set appearance value for Windows
    #props_iface.Set(
    #    "org.bluez.Adapter1",
    #    "Appearance",
    #    dbus.UInt16(0x03C4)  # Gamepad appearance value
    #)

    # Create the HID Information Characteristic at /org/bluez/example/service0/char2
    # Update HID Information Characteristic initialization
    hid_info_char = ManualCharacteristic(
        bus,
        "/org/bluez/example/service0/char2", 
        "00002A4A-0000-1000-8000-00805f9b34fb",  # HID Information UUID
        ["read"],
        "/org/bluez/example/service0",
        read_handler=hid_info_read_handler  # Add the read handler
    )

    # Update Protocol Mode Characteristic initialization
    protocol_mode_char = ManualCharacteristic(
        bus,
        "/org/bluez/example/service0/char3",
        "00002A4E-0000-1000-8000-00805f9b34fb",  # Protocol Mode UUID 
        ["read", "write", "write-without-response"],
        "/org/bluez/example/service0",
        read_handler=protocol_mode_read_handler  # Add the read handler
    )

    # Add Report Reference descriptor to gamepad report characteristic
    report_ref_desc = ManualDescriptor(
        bus,
        "/org/bluez/example/service0/char1/desc0",
        "/org/bluez/example/service0/char1",
        "00002908-0000-1000-8000-00805f9b34fb",
        [dbus.Byte(0x01), dbus.Byte(0x01)]  # Report ID 1, Input Report
    )
    gamepad_report_char.add_descriptor(report_ref_desc)

    # Add Client Characteristic Configuration descriptor
    ccc_desc = ManualDescriptor(
        bus,
        "/org/bluez/example/service0/char1/desc1",
        "/org/bluez/example/service0/char1",
        "00002902-0000-1000-8000-00805f9b34fb",  # Full CCC UUID
        [dbus.Byte(0x00), dbus.Byte(0x01)]  # Changed to enable notifications (0x0100)
    )

    output_report_ref = ManualDescriptor(
        bus,
        "/org/bluez/example/service0/char1/desc2",
        "/org/bluez/example/service0/char1",
        "00002908-0000-1000-8000-00805f9b34fb",
        [dbus.Byte(0x00), dbus.Byte(0x02)]  # Output report with ID 0
    )

    gamepad_report_char.add_descriptor(ccc_desc)
    gamepad_report_char.add_descriptor(output_report_ref)

    print("Manual GATT server running with:")
    print("  - Report Map Characteristic at /org/bluez/example/service0/char0")
    print("  - Gamepad Report Characteristic at /org/bluez/example/service0/char1")
    print("  - HID Information Characteristic at /org/bluez/example/service0/char2")
    print("  - Protocol Mode Characteristic at /org/bluez/example/service0/char3")

    # --- Create and register BLE Advertisement ---
    advertisement = Advertisement(bus, 0, "peripheral")
    # Advertise the HID Service UUID so that Windows can identify the device as a gamepad.
    advertisement.service_uuids.append("00001812-0000-1000-8000-00805f9b34fb")  # HID Service UUID
    advertisement.include_tx_power = True

    register_advertisement(bus, advertisement)

    loop = GLib.MainLoop()
    loop.run()

if __name__ == "__main__":
    main()
