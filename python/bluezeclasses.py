import dbus
import dbus.service
import constants  # Contient vos constantes (ex. BLUEZ_LEADVERTISEMENT_IFACE, etc.)

class InvalidArgsException(dbus.exceptions.DBusException):
    _dbus_error_name = 'org.freedesktop.DBus.Error.InvalidArgs'


class NotSupportedException(dbus.exceptions.DBusException):
    _dbus_error_name = 'org.bluez.Error.NotSupported'


class NotPermittedException(dbus.exceptions.DBusException):
    _dbus_error_name = 'org.bluez.Error.NotPermitted'


class InvalidValueLengthException(dbus.exceptions.DBusException):
    _dbus_error_name = 'org.bluez.Error.InvalidValueLength'


class FailedException(dbus.exceptions.DBusException):
    _dbus_error_name = 'org.bluez.Error.Failed'


class DBusPropertiesObject(dbus.service.Object):
    """
    Classe de base qui implémente l'interface org.freedesktop.DBus.Properties.
    Les classes dérivées doivent redéfinir get_properties() pour renvoyer un dictionnaire
    sous la forme {interface: {prop_name: value, ...}}.
    """
    def __init__(self, bus, object_path):
        super().__init__(bus, object_path)

    def get_properties(self):
        """À redéfinir dans les classes dérivées."""
        return {}

    @dbus.service.method("org.freedesktop.DBus.Properties",
                         in_signature="ss", out_signature="v")
    def Get(self, interface, prop):
        props = self.get_properties().get(interface, {})
        if prop in props:
            return props[prop]
        raise dbus.exceptions.DBusException("org.freedesktop.DBus.Error.InvalidArgs",
                                            "No such property")

    @dbus.service.method("org.freedesktop.DBus.Properties",
                         in_signature="s", out_signature="a{sv}")
    def GetAll(self, interface):
        return self.get_properties().get(interface, {})

    @dbus.service.method("org.freedesktop.DBus.Properties",
                         in_signature="ssv", out_signature="")
    def Set(self, interface, prop, value):
        raise dbus.exceptions.DBusException("org.freedesktop.DBus.Error.PropertyReadOnly",
                                            "Property is read-only")


# Agent n'implique pas de propriétés D-Bus
class Agent(dbus.service.Object):
    def __init__(self, bus, object_path):
        dbus.service.Object.__init__(self, bus, object_path)
        self.object_path = object_path

    @dbus.service.method(constants.BLUEZ_AGENT_IFACE, in_signature="", out_signature="")
    def Release(self):
        print("Agent Released")

    @dbus.service.method(constants.BLUEZ_AGENT_IFACE, in_signature="o", out_signature="u")
    def RequestPasskey(self, device):
        print("RequestPasskey for device:", device)
        return 0

    @dbus.service.method(constants.BLUEZ_AGENT_IFACE, in_signature="ou", out_signature="")
    def DisplayPasskey(self, device, passkey):
        print("DisplayPasskey for device:", device, passkey)

    @dbus.service.method(constants.BLUEZ_AGENT_IFACE, in_signature="ou", out_signature="")
    def RequestConfirmation(self, device, passkey):
        print("Auto-confirming pairing for device:", device, "with passkey:", passkey)
        return

    @dbus.service.method(constants.BLUEZ_AGENT_IFACE, in_signature="o", out_signature="s")
    def RequestPinCode(self, device):
        print("RequestPinCode for device:", device)
        return "0000"

    @dbus.service.method(constants.BLUEZ_AGENT_IFACE, in_signature="o", out_signature="")
    def RequestAuthorization(self, device):
        print("RequestAuthorization for device:", device)
        return

    @dbus.service.method(constants.BLUEZ_AGENT_IFACE, in_signature="os", out_signature="")
    def AuthorizeService(self, device, uuid):
        print("AuthorizeService called for device {} and service UUID {}".format(device, uuid))
        return

class Advertisement(dbus.service.Object):

    def __init__(self, bus, index, advertising_type):
        self.path = constants.ADVERTISEMENT_PATH + str(index)
        self.bus = bus
        self.ad_type = advertising_type
        self.service_uuids = None
        self.manufacturer_data = None
        self.solicit_uuids = None
        self.service_data = None
        self.local_name = None
        self.include_tx_power = False
        self.data = None
        self.appearance = None
        dbus.service.Object.__init__(self, bus, self.path)

    def get_properties(self):
        properties = dict()
        properties['Type'] = self.ad_type
        if self.service_uuids is not None:
            properties['ServiceUUIDs'] = dbus.Array(self.service_uuids,
                                                    signature='s')
        if self.appearance is not None:
            properties['Appearance'] = dbus.UInt16(self.appearance)
        if self.solicit_uuids is not None:
            properties['SolicitUUIDs'] = dbus.Array(self.solicit_uuids,
                                                    signature='s')
        if self.manufacturer_data is not None:
            properties['ManufacturerData'] = dbus.Dictionary(
                self.manufacturer_data, signature='qv')
        if self.service_data is not None:
            properties['ServiceData'] = dbus.Dictionary(self.service_data,
                                                        signature='sv')
        if self.local_name is not None:
            properties['LocalName'] = dbus.String(self.local_name)
        if self.include_tx_power:
            properties['Includes'] = dbus.Array(["tx-power"], signature='s')

        if self.data is not None:
            properties['Data'] = dbus.Dictionary(
                self.data, signature='yv')
        return {constants.BLUEZ_LEADVERTISEMENT_IFACE: properties}

    def get_path(self):
        return dbus.ObjectPath(self.path)

    def add_service_uuid(self, uuid):
        if not self.service_uuids:
            self.service_uuids = []
        self.service_uuids.append(uuid)

    def add_solicit_uuid(self, uuid):
        if not self.solicit_uuids:
            self.solicit_uuids = []
        self.solicit_uuids.append(uuid)

    def add_manufacturer_data(self, manuf_code, data):
        if not self.manufacturer_data:
            self.manufacturer_data = dbus.Dictionary({}, signature='qv')
        self.manufacturer_data[manuf_code] = dbus.Array(data, signature='y')

    def add_service_data(self, uuid, data):
        if not self.service_data:
            self.service_data = dbus.Dictionary({}, signature='sv')
        self.service_data[uuid] = dbus.Array(data, signature='y')

    def add_local_name(self, name):
        if not self.local_name:
            self.local_name = ""
        self.local_name = dbus.String(name)

    def add_data(self, ad_type, data):
        if not self.data:
            self.data = dbus.Dictionary({}, signature='yv')
        self.data[ad_type] = dbus.Array(data, signature='y')

    @dbus.service.method(constants.DBUS_PROPERTIES_IFACE,
                         in_signature='s',
                         out_signature='a{sv}')
    def GetAll(self, interface):
        print('GetAll')
        if interface != constants.BLUEZ_LEADVERTISEMENT_IFACE:
            raise InvalidArgsException()
        print('returning props')
        return self.get_properties()[constants.BLUEZ_LEADVERTISEMENT_IFACE]

    @dbus.service.method(constants.BLUEZ_LEADVERTISEMENT_IFACE,
                         in_signature='',
                         out_signature='')
    def Release(self):
        print('%s: Released!' % self.path)



# ManualCharacteristic implémente l'interface GattCharacteristic et Properties.
class ManualCharacteristic(DBusPropertiesObject):
    def __init__(self, bus, object_path, uuid, flags, service_path, read_handler=None):
        super().__init__(bus, object_path)
        self.object_path = object_path
        self._uuid = uuid
        self._service = service_path
        self._flags = flags
        self._read_handler = read_handler
        self._descriptors = []
        self._notifying = False
        self._value = b""

    def get_properties(self):
        return {
            constants.BLUEZ_GATT_CHARACTERISTIC_IFACE: {
                "UUID": self._uuid,
                "Service": self._service,
                "Flags": self._flags,
                "Descriptors": self._descriptors
            }
        }

    @dbus.service.method(constants.BLUEZ_GATT_CHARACTERISTIC_IFACE,
                         in_signature="a{sv}", out_signature="ay")
    def ReadValue(self, options):
        print("ManualCharacteristic ReadValue called")
        if self._read_handler:
            # On suppose que le handler renvoie un objet bytes
            return list(self._read_handler())
        print("Default ReadValue called")
        return [0]

    @dbus.service.method(constants.BLUEZ_GATT_CHARACTERISTIC_IFACE,
                         in_signature="aya{sv}", out_signature="")
    def WriteValue(self, value, options):
        try:
            # value est une liste d'entiers
            value_bytes = bytes(value)
            print("WriteValue called with value:", value_bytes.hex())
            self._value = value_bytes
            if len(value_bytes) > 0:
                report_id = value_bytes[0]
                if report_id == 0x02 and len(value_bytes) > 1:
                    led_state = value_bytes[1] & 0x01
                    print(f"Output report received: LED state = {led_state}")
        except Exception as ex:
            print("Error in WriteValue:", ex)
            raise

    @dbus.service.method(constants.BLUEZ_GATT_CHARACTERISTIC_IFACE,
                         in_signature="", out_signature="")
    def StartNotify(self):
        print("StartNotify called for", self.object_path)
        if not self._notifying:
            self._notifying = True
            if self._read_handler:
                self._value = self._read_handler()
                # Dans une implémentation complète, émettre un signal PropertiesChanged

    @dbus.service.method(constants.BLUEZ_GATT_CHARACTERISTIC_IFACE,
                         in_signature="", out_signature="")
    def StopNotify(self):
        print("StopNotify called for", self.object_path)
        self._notifying = False

    def SendNotification(self, value: bytes):
        if self._notifying:
            self._value = value
            # Ici, vous pourriez émettre un signal PropertiesChanged
            print("Notification sent with value:", value.hex())

    def AddDescriptor(self, descriptor_path):
        self._descriptors.append(descriptor_path)


# ManualDescriptor implémente l'interface GattDescriptor et Properties.
class ManualDescriptor(DBusPropertiesObject):
    def __init__(self, bus, object_path, characteristic_path, uuid, value):
        super().__init__(bus, object_path)
        self.object_path = object_path
        self._characteristic = characteristic_path
        self._uuid = uuid
        self._value = value  # de type bytes

    def get_properties(self):
        return {
            constants.BLUEZ_GATT_DESCRIPTOR_IFACE: {
                "UUID": self._uuid,
                "Characteristic": self._characteristic,
            }
        }

    @dbus.service.method(constants.BLUEZ_GATT_DESCRIPTOR_IFACE,
                         in_signature="a{sv}", out_signature="ay")
    def ReadValue(self, options):
        # Retourne la valeur stockée sous forme de liste d'entiers
        return list(self._value)


# ManualService implémente l'interface GattService et Properties.
class ManualService(DBusPropertiesObject):
    def __init__(self, bus, object_path, characteristics, service_uuid):
        super().__init__(bus, object_path)
        self.object_path = object_path
        self._characteristics = characteristics
        self._uuid = service_uuid
        self._primary = True

    def get_properties(self):
        return {
            constants.BLUEZ_GATT_SERVICE_IFACE: {
                "UUID": self._uuid,
                "Primary": dbus.Boolean(self._primary),
                "Characteristics": self._characteristics
            }
        }