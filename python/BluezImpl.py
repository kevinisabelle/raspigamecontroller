import dbus
import dbus.service
import Constants  # Contient vos constantes (ex. BLUEZ_LEADVERTISEMENT_IFACE, etc.)

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

class Agent(dbus.service.Object):
    def __init__(self, bus, object_path):
        dbus.service.Object.__init__(self, bus, object_path)
        self.object_path = object_path

    @dbus.service.method(Constants.BLUEZ_AGENT_IFACE, in_signature="", out_signature="")
    def Release(self):
        print("Agent Released")

    @dbus.service.method(Constants.BLUEZ_AGENT_IFACE, in_signature="o", out_signature="u")
    def RequestPasskey(self, device):
        print("RequestPasskey for device:", device)
        return 0

    @dbus.service.method(Constants.BLUEZ_AGENT_IFACE, in_signature="ou", out_signature="")
    def DisplayPasskey(self, device, passkey):
        print("DisplayPasskey for device:", device, passkey)

    @dbus.service.method(Constants.BLUEZ_AGENT_IFACE, in_signature="ou", out_signature="")
    def RequestConfirmation(self, device, passkey):
        print("Auto-confirming pairing for device:", device, "with passkey:", passkey)
        return

    @dbus.service.method(Constants.BLUEZ_AGENT_IFACE, in_signature="o", out_signature="s")
    def RequestPinCode(self, device):
        print("RequestPinCode for device:", device)
        return "0000"

    @dbus.service.method(Constants.BLUEZ_AGENT_IFACE, in_signature="o", out_signature="")
    def RequestAuthorization(self, device):
        print("RequestAuthorization for device:", device)
        return

    @dbus.service.method(Constants.BLUEZ_AGENT_IFACE, in_signature="os", out_signature="")
    def AuthorizeService(self, device, uuid):
        print("AuthorizeService called for device {} and service UUID {}".format(device, uuid))
        return

class Advertisement(dbus.service.Object):

    def __init__(self, bus, index, advertising_type):
        self.path = Constants.ADVERTISEMENT_PATH + str(index)
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
        return {Constants.BLUEZ_LEADVERTISEMENT_IFACE: properties}

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

    @dbus.service.method(Constants.DBUS_PROPERTIES_IFACE,
                         in_signature='s',
                         out_signature='a{sv}')
    def GetAll(self, interface):
        print('GetAll')
        if interface != Constants.BLUEZ_LEADVERTISEMENT_IFACE:
            raise InvalidArgsException()
        print('returning props')
        return self.get_properties()[Constants.BLUEZ_LEADVERTISEMENT_IFACE]

    @dbus.service.method(Constants.BLUEZ_LEADVERTISEMENT_IFACE,
                         in_signature='',
                         out_signature='')
    def Release(self):
        print('%s: Released!' % self.path)

class Service(dbus.service.Object):
    """
    org.bluez.GattService1 interface implementation
    """
    def __init__(self, bus, index, uuid, primary):
        self.path = Constants.SERVICE_PATH + str(index)
        self.bus = bus
        self.uuid = uuid
        self.primary = primary
        self.characteristics = []
        dbus.service.Object.__init__(self, bus, self.path)

    def get_properties(self):
        return {
                Constants.BLUEZ_GATT_SERVICE_IFACE: {
                        'UUID': self.uuid,
                        'Primary': self.primary,
                        'Characteristics': dbus.Array(
                                self.get_characteristic_paths(),
                                signature='o')
                }
        }

    def get_path(self):
        return dbus.ObjectPath(self.path)

    def add_characteristic(self, characteristic):
        self.characteristics.append(characteristic)

    def get_characteristic_paths(self):
        result = []
        for chrc in self.characteristics:
            result.append(chrc.get_path())
        return result

    def get_characteristics(self):
        return self.characteristics

    @dbus.service.method(Constants.DBUS_PROPERTIES_IFACE,
                         in_signature='s',
                         out_signature='a{sv}')
    def GetAll(self, interface):
        if interface != Constants.BLUEZ_GATT_SERVICE_IFACE:
            raise InvalidArgsException()

        return self.get_properties()[Constants.BLUEZ_GATT_SERVICE_IFACE]


class Characteristic(dbus.service.Object):
    """
    org.bluez.GattCharacteristic1 interface implementation
    """
    def __init__(self, bus, index, uuid, flags, service):
        self.path = service.path + '/char' + str(index)
        self.bus = bus
        self.uuid = uuid
        self.service = service
        self.flags = flags
        self.descriptors = []
        dbus.service.Object.__init__(self, bus, self.path)

    def get_properties(self):
        return {
                Constants.BLUEZ_GATT_CHARACTERISTIC_IFACE: {
                        'Service': self.service.get_path(),
                        'UUID': self.uuid,
                        'Flags': self.flags,
                        'Descriptors': dbus.Array(
                                self.get_descriptor_paths(),
                                signature='o')
                }
        }

    def get_path(self):
        return dbus.ObjectPath(self.path)

    def add_descriptor(self, descriptor):
        self.descriptors.append(descriptor)

    def get_descriptor_paths(self):
        result = []
        for desc in self.descriptors:
            result.append(desc.get_path())
        return result

    def get_descriptors(self):
        return self.descriptors

    @dbus.service.method(Constants.DBUS_PROPERTIES_IFACE,
                         in_signature='s',
                         out_signature='a{sv}')
    def GetAll(self, interface):
        if interface != Constants.BLUEZ_GATT_CHARACTERISTIC_IFACE:
            raise InvalidArgsException()

        return self.get_properties()[Constants.BLUEZ_GATT_CHARACTERISTIC_IFACE]

    @dbus.service.method(Constants.BLUEZ_GATT_CHARACTERISTIC_IFACE,
                        in_signature='a{sv}',
                        out_signature='ay')
    def ReadValue(self, options):
        print('Default ReadValue called, returning error')
        raise NotSupportedException()

    @dbus.service.method(Constants.BLUEZ_GATT_CHARACTERISTIC_IFACE, in_signature='aya{sv}')
    def WriteValue(self, value, options):
        print('Default WriteValue called, returning error')
        raise NotSupportedException()

    @dbus.service.method(Constants.BLUEZ_GATT_CHARACTERISTIC_IFACE)
    def StartNotify(self):
        print('Default StartNotify called, returning error')
        raise NotSupportedException()

    @dbus.service.method(Constants.BLUEZ_GATT_CHARACTERISTIC_IFACE)
    def StopNotify(self):
        print('Default StopNotify called, returning error')
        raise NotSupportedException()

    @dbus.service.signal(Constants.DBUS_PROPERTIES_IFACE,
                         signature='sa{sv}as')
    def PropertiesChanged(self, interface, changed, invalidated):
        pass


class Descriptor(dbus.service.Object):
    """
    org.bluez.GattDescriptor1 interface implementation
    """
    def __init__(self, bus, index, uuid, flags, characteristic):
        self.path = characteristic.path + '/desc' + str(index)
        self.bus = bus
        self.uuid = uuid
        self.flags = flags
        self.chrc = characteristic
        dbus.service.Object.__init__(self, bus, self.path)

    def get_properties(self):
        return {
                Constants.BLUEZ_GATT_DESCRIPTOR_IFACE: {
                        'Characteristic': self.chrc.get_path(),
                        'UUID': self.uuid,
                        'Flags': self.flags,
                }
        }

    def get_path(self):
        return dbus.ObjectPath(self.path)

    @dbus.service.method(Constants.DBUS_PROPERTIES_IFACE,
                         in_signature='s',
                         out_signature='a{sv}')
    def GetAll(self, interface):
        if interface != Constants.BLUEZ_GATT_DESCRIPTOR_IFACE:
            raise InvalidArgsException()

        return self.get_properties()[Constants.BLUEZ_GATT_DESCRIPTOR_IFACE]

    @dbus.service.method(Constants.BLUEZ_GATT_DESCRIPTOR_IFACE,
                        in_signature='a{sv}',
                        out_signature='ay')
    def ReadValue(self, options):
        print ('Default ReadValue called, returning error')
        raise NotSupportedException()

    @dbus.service.method(Constants.BLUEZ_GATT_DESCRIPTOR_IFACE, in_signature='aya{sv}')
    def WriteValue(self, value, options):
        print('Default WriteValue called, returning error')
        raise NotSupportedException()
    

def find_adapter(bus):
    remote_om = dbus.Interface(bus.get_object(Constants.BLUEZ_SERVICE, '/'), Constants.DBUS_OM_IFACE)
    objects = remote_om.GetManagedObjects()

    for o, props in objects.items():
        if Constants.BLUEZ_GATT_MANAGER_IFACE in props.keys():
            return o

    return None

def register_advertisement(bus, advertisement):
    print("Registering advertisement...")
    adapter = bus.get_object(Constants.BLUEZ_SERVICE, Constants.ADAPTER_PATH)
    ad_manager = dbus.Interface(adapter, Constants.BLUEZ_LEADVERTISEMENT_MANAGER_IFACE)
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

def register_agent(bus, agent, capability="KeyboardDisplay"):
    try:
        print("Registering agent...")
        agent_manager_obj = bus.get_object(Constants.BLUEZ_SERVICE, Constants.BLUEZ_SERVICE_PATH)
        agent_manager = dbus.Interface(agent_manager_obj, "org.bluez.AgentManager1")
        agent_manager.RegisterAgent(agent.object_path, capability)
        agent_manager.RequestDefaultAgent(agent.object_path)
        print("Agent registered as default with {} capability".format(capability))
        try:
            adapter_obj = bus.get_object(Constants.BLUEZ_SERVICE, Constants.ADAPTER_PATH)
            props_iface = dbus.Interface(adapter_obj, "org.freedesktop.DBus.Properties")
            props_iface.Set("org.bluez.Adapter1", "Powered", dbus.Boolean(True))
            props_iface.Set("org.bluez.Adapter1", "Discoverable", dbus.Boolean(True))
            props_iface.Set("org.bluez.Adapter1", "Pairable", dbus.Boolean(True))
        except Exception as e:
            print("Error setting adapter properties:", e)
    except Exception as e:
        print("Failed to register agent:", e)
        raise

def register_application(bus, app, mainloop):
    adapter = find_adapter(bus)
    service_manager = dbus.Interface(bus.get_object(Constants.BLUEZ_SERVICE, adapter), Constants.BLUEZ_GATT_MANAGER_IFACE)

    def register_app_cb():
        print("Application registered")

    def register_app_error_cb(error):
        print("Failed to register application:", error)
        mainloop.quit()

    service_manager.RegisterApplication(app.get_path(), {},
                                    reply_handler=register_app_cb,
                                    error_handler=register_app_error_cb)