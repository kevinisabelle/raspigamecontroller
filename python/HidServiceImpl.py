import dbus
import dbus.service
import dbus.mainloop.glib
from gi.repository import GLib
import Constants
from BluezImpl import (
    Advertisement,
    Characteristic,
    Descriptor,
    InvalidArgsException,
    Service,
)
from HidGamepadReport import GamepadDefinition

class GamePadAdvertisment(Advertisement):
    def __init__(self, bus, index):
        Advertisement.__init__(self, bus, index, 'peripheral')
        self.add_service_uuid('1812')
        self.add_local_name('KiGP')
        self.appearance = Constants.ADV_APPEARANCE_GAMEPAD
        self.include_tx_power = True

class ReportMapChrc(Characteristic):
    def __init__(self, bus, index, service, gamepad_values : GamepadDefinition):
        Characteristic.__init__(self, bus, index, 
                                Constants.GATT_REPORT_MAP_UUID, 
                                ['read'], service)
        self.gamepad_values = gamepad_values
        
    def ReadValue(self, options):
        reportMap = self.gamepad_values.get_report_map_bytes()
        print("Report Map read handler called, Hex: ", " ".join(f"{b:02X}" for b in reportMap))
        return reportMap

class ClientCharacteristicConfigurationDesc(Descriptor):
    def __init__(self, bus, index, characteristic):
        Descriptor.__init__(self, bus, index,
                            Constants.GATT_DESC_CLIENT_DESCRIPTOR_UUID,
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
        self.characteristic.StartNotify() if self.value[0] & 0x01 else self.characteristic.StopNotify()

class ReportReferenceDesc(Descriptor):
    def __init__(self, bus, index, characteristic):
        Descriptor.__init__(self, bus, index,
                            Constants.GATT_DESC_REPORT_REFERENCE_UUID,
                            ['read'], characteristic)
        self.value = [0x00, 0x01]

    def ReadValue(self, options):
        print("Report Reference Read: ", self.value)
        return self.value

class ReportChrc(Characteristic):
    def __init__(self, bus, index, service, gamepad_values : GamepadDefinition):
        Characteristic.__init__(self, bus, index, 
                                Constants.GATT_REPORT_UUID, 
                                ['read', 'notify', 'write-without-response'], service)
        self.add_descriptor(ClientCharacteristicConfigurationDesc(bus, 0, self))
        self.add_descriptor(ReportReferenceDesc(bus, 1, self))
        self.notifying = False
        self.notify_timer = None
        self.gamepad_values = gamepad_values

    def ReadValue(self, options):
        report = self.gamepad_values.get_report_bytes()
        print("Report read handler called, Hex: ", " ".join(f"{b:02X}" for b in report))

        return self.gamepad_values.get_report_bytes()
    
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
        value_list = self.gamepad_values.get_report_bytes()  # e.g. [0x00, 0x23, 0x54, 0x76]
        print("Sending notification with values. Hex: ", " ".join(f"{b:02X}" for b in value_list))
        
        # Convert to a dbus Array of bytes
        value_dbus = dbus.Array(value_list, signature='y')
        
        self.PropertiesChanged(
            Constants.BLUEZ_GATT_CHARACTERISTIC_IFACE,
            {'Value': value_dbus},
            []
        )
        return True

class ProtocolModeChrc(Characteristic):
    def __init__(self, bus, index, service):
        Characteristic.__init__(self, bus, index, 
                                Constants.GATT_PROTOCOL_MODE_UUID, 
                                ['read', 'write', 'write-without-response'], service)
        
    def ReadValue(self, options):
        print("Protocol Mode read handler called")
        return bytes([0x01]) # Report Protocol mode (1 = Input, 2 = Output)

class HidInfoChrc(Characteristic):
    def __init__(self, bus, index, service):
        Characteristic.__init__(self, bus, index, 
                                Constants.GATT_HID_INFORMATION_UUID, 
                                ['read'], service)
        
    def ReadValue(self, options):
        print("HID Information read handler called")
        return bytes([0x11, 0x01, 0x00, 0x03]) # bcdHID, bCountryCode, Flags (RemoteWake, NormallyConnectable)
        
class HidControlPointChrc(Characteristic):
    def __init__(self, bus, index, service):
        Characteristic.__init__(self, bus, index, 
                                Constants.GATT_HID_CONTROL_POINT_UUID, 
                                ['write'], service)
        
    def WriteValue(self, value, options):
        print("HID Control Point write handler called")
        print("Value:", value)
        
class HidGattService(Service):
    def __init__(self, bus, index, gamepad_values : GamepadDefinition):
        Service.__init__(self, bus, index, Constants.GATT_SERVICE_HID_UUID, True)
        self.add_characteristic(ReportMapChrc(bus, 0, self, gamepad_values))
        self.add_characteristic(ReportChrc(bus, 1, self, gamepad_values))
        self.add_characteristic(ProtocolModeChrc(bus, 2, self))
        self.add_characteristic(HidInfoChrc(bus, 3, self))
        self.add_characteristic(HidControlPointChrc(bus, 4, self))

class ManufacturerNameChrc(Characteristic):
    def __init__(self, bus, index, service):
        Characteristic.__init__(self, bus, index, '2a29', ['read'], service)
        
    def ReadValue(self, options):
        print("Manufacturer Name read handler called")
        return list("Ki".encode())
    
class ModelNumberChrc(Characteristic):
    def __init__(self, bus, index, service):
        Characteristic.__init__(self, bus, index, '2a24', ['read'], service)
        
    def ReadValue(self, options):
        print("Model Number read handler called")
        return list("GP".encode())
    
class SerialNumberChrc(Characteristic):
    def __init__(self, bus, index, service):
        Characteristic.__init__(self, bus, index, Constants.SERIAL_NUMBER_CHARACTERISTIC_UUID, ['read'], service)
        
    def ReadValue(self, options):
        print("Serial Number read handler called")
        return list("123456".encode())
    
class HardwareRevisionChrc(Characteristic):
    def __init__(self, bus, index, service):
        Characteristic.__init__(self, bus, index, '2a27', ['read'], service)
        
    def ReadValue(self, options):
        print("Hardware Revision read handler called")
        return list("1.0".encode())

class DeviceInfoService(Service):
    def __init__(self, bus, index):
        Service.__init__(self, bus, index, Constants.DEVICE_INFORMATION_SERVICE_UUID, True) # Device Information Service (0x180a)
        self.add_characteristic(ManufacturerNameChrc(bus, 0, self))
        self.add_characteristic(ModelNumberChrc(bus, 1, self))
        self.add_characteristic(SerialNumberChrc(bus, 2, self))
        self.add_characteristic(HardwareRevisionChrc(bus, 3, self))

class Application(dbus.service.Object):
    """
    org.bluez.GattApplication1 interface implementation
    """
    def __init__(self, bus, gamepad_values : GamepadDefinition):
        self.path = '/'
        self.services = []
        dbus.service.Object.__init__(self, bus, self.path)
        self.add_service(HidGattService(bus, 0, gamepad_values))
        self.add_service(DeviceInfoService(bus, 1))
       
    def get_path(self):
        return dbus.ObjectPath(self.path)

    def add_service(self, service):
        self.services.append(service)

    def notify_hid_report(self):
        for service in self.services:
            for chrc in service.characteristics:
                if chrc.uuid == Constants.GATT_REPORT_UUID:
                    chrc.send_notification()

    @dbus.service.method(Constants.DBUS_OM_IFACE, out_signature='a{oa{sa{sv}}}')
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