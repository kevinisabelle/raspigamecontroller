import dbus
import dbus.service
import dbus.mainloop.glib
from gi.repository import GLib
import constants
from bluezeclasses import (
    Advertisement,
    Characteristic,
    Descriptor,
    InvalidArgsException,
    Service,
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
    return bytes([0x01, 0x01, 0x64])

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

class GamePadAdvertisment(Advertisement):
    def __init__(self, bus, index):
        Advertisement.__init__(self, bus, index, 'peripheral')
        self.add_service_uuid('1812')
        self.add_local_name('KiGP')
        self.appearance = constants.ADV_APPEARANCE_GAMEPAD
        self.include_tx_power = True

class ReportMapChrc(Characteristic):
    def __init__(self, bus, index, service):
        Characteristic.__init__(self, bus, index, 
                                constants.GATT_REPORT_MAP_UUID, 
                                ['read'], service)
        
    def ReadValue(self, options):
        return report_map_read_handler()

class ClientCharacteristicConfigurationDesc(Descriptor):
    def __init__(self, bus, index, characteristic):
        Descriptor.__init__(self, bus, index,
                            constants.GATT_DESC_CLIENT_DESCRIPTOR_UUID,
                            ['read', 'write'], characteristic)
        # 2 bytes: first bit for notifications, second for indications.
        # Default: both disabled.
        self.value = [0x01, 0x00]

    def ReadValue(self, options):
        # Return the current configuration.
        print("CCCD Read: ", self.value)
        return self.value

    def WriteValue(self, value, options):
        # Validate the value length.
        if len(value) != 2:
            raise InvalidArgsException("CCCD value must be 2 bytes")
        # Update the configuration value.
        self.value = value
        print("CCCD Updated to: ", self.value)
        # Optionally, you might want to notify the characteristic
        # that the configuration changed.

class ReportReferenceDesc(Descriptor):
    def __init__(self, bus, index, characteristic):
        Descriptor.__init__(self, bus, index,
                            constants.GATT_DESC_REPORT_REFERENCE_UUID,
                            ['read'], characteristic)
        self.value = [0x00, 0x01]

    def ReadValue(self, options):
        print("Report Reference Read: ", self.value)
        return self.value
    
class GamePadValues:
    def __init__(self):
        self.x = 0x05
        self.y = 0x06
        self.button1 = 0x01

    def update(self, x, y, button1):
        self.x = x
        self.y = y
        self.button1 = button1

    def get(self):
        return [self.button1, self.y]
    
    def to_bytes(self):
        return bytes([0x01, self.button1, self.y])

class ReportChrc(Characteristic):
    def __init__(self, bus, index, service, gamepad_values : GamePadValues):
        Characteristic.__init__(self, bus, index, 
                                constants.GATT_REPORT_UUID, 
                                ['read', 'notify', 'write-without-response'], service)
        self.add_descriptor(ClientCharacteristicConfigurationDesc(bus, 0, self))
        self.add_descriptor(ReportReferenceDesc(bus, 1, self))
        self.notifying = False
        self.notify_timer = None
        self.gamepad_values = gamepad_values

    def ReadValue(self, options):
        print("Report read handler called")
        return self.gamepad_values.to_bytes()
    
    def StartNotify(self):
        print("Notification started")
        self.notifying = True
        self.notify_timer = GLib.timeout_add(1000, self.send_notification)
    
    def StopNotify(self):
        print("Notification stopped")
        self.notifying = False
        GLib.source_remove(self.notify_timer)

    def send_notification(self):
        if not self.notifying:
            return False
        value = self.gamepad_values.to_bytes()
        print("Sending notification with values:", value.hex())
        self.PropertiesChanged(constants.BLUEZ_GATT_CHARACTERISTIC_IFACE, {'Value': value}, [])
        return True

class ProtocolModeChrc(Characteristic):
    def __init__(self, bus, index, service):
        Characteristic.__init__(self, bus, index, 
                                constants.GATT_PROTOCOL_MODE_UUID, 
                                ['read', 'write', 'write-without-response'], service)
        
    def ReadValue(self, options):
        print("Protocol Mode read handler called")
        return protocol_mode_read_handler()

class HidInfoChrc(Characteristic):
    def __init__(self, bus, index, service):
        Characteristic.__init__(self, bus, index, 
                                constants.GATT_HID_INFORMATION_UUID, 
                                ['read'], service)
        
    def ReadValue(self, options):
        print("HID Information read handler called")
        return hid_info_read_handler()
        
class HidControlPointChrc(Characteristic):
    def __init__(self, bus, index, service):
        Characteristic.__init__(self, bus, index, 
                                constants.GATT_HID_CONTROL_POINT_UUID, 
                                ['write'], service)
        
    def WriteValue(self, value, options):
        print("HID Control Point write handler called")
        print("Value:", value)
        
class HidGattService(Service):
    def __init__(self, bus, index, gamepad_values : GamePadValues):
        Service.__init__(self, bus, index, constants.GATT_SERVICE_HID_UUID, True)
        self.add_characteristic(ReportMapChrc(bus, 0, self))
        self.add_characteristic(ReportChrc(bus, 1, self, gamepad_values))
        self.add_characteristic(ProtocolModeChrc(bus, 2, self))
        self.add_characteristic(HidInfoChrc(bus, 3, self))
        self.add_characteristic(HidControlPointChrc(bus, 4, self))
        self.energy_expended = 0

class Application(dbus.service.Object):
    """
    org.bluez.GattApplication1 interface implementation
    """
    def __init__(self, bus, gamepad_values : GamePadValues):
        self.path = '/'
        self.services = []
        dbus.service.Object.__init__(self, bus, self.path)
        self.add_service(HidGattService(bus, 0, gamepad_values))
       
    def get_path(self):
        return dbus.ObjectPath(self.path)

    def add_service(self, service):
        self.services.append(service)

    @dbus.service.method(constants.DBUS_OM_IFACE, out_signature='a{oa{sa{sv}}}')
    def GetManagedObjects(self):
        response = {}
        print('GetManagedObjects')

        for service in self.services:
            response[service.get_path()] = service.get_properties()
            chrcs = service.get_characteristics()
            for chrc in chrcs:
                response[chrc.get_path()] = chrc.get_properties()
                descs = chrc.get_descriptors()
                for desc in descs:
                    response[desc.get_path()] = desc.get_properties()

        return response
    
def find_adapter(bus):
    remote_om = dbus.Interface(bus.get_object(constants.BLUEZ_SERVICE, '/'), constants.DBUS_OM_IFACE)
    objects = remote_om.GetManagedObjects()

    for o, props in objects.items():
        if constants.BLUEZ_GATT_MANAGER_IFACE in props.keys():
            return o

    return None

gamepad_values = GamePadValues()

def main():
    # Initialiser la boucle principale de GLib pour dbus
    dbus.mainloop.glib.DBusGMainLoop(set_as_default=True)
    print("Starting GamepadKi...")
    bus = dbus.SystemBus()

    adapter = find_adapter(bus)

    # Créer et exporter l'agent
    Agent(bus, constants.AGENT_PATH)

    mainloop = GLib.MainLoop()

    register_agent(bus)

    characteristics = []

    service_manager = dbus.Interface(
            bus.get_object(constants.BLUEZ_SERVICE, adapter),
            constants.BLUEZ_GATT_MANAGER_IFACE)

    app = Application(bus, gamepad_values)

    def register_app_cb():
        print("Application registered")

    def register_app_error_cb(error):
        print("Failed to register application:", error)
        mainloop.quit()

    service_manager.RegisterApplication(app.get_path(), {},
                                    reply_handler=register_app_cb,
                                    error_handler=register_app_error_cb)
    
    # Créer et exporter la publicité BLE
    advertisement = GamePadAdvertisment(bus, 0)

    # Enregistrer la publicité auprès de BlueZ
    register_advertisement(bus, advertisement)

    try:
        mainloop.run()
    except KeyboardInterrupt:
        print("Exiting...")

if __name__ == "__main__":
    main()
