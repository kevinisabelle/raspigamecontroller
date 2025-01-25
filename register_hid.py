import dbus
import dbus.service
import dbus.mainloop.glib
from gi.repository import GLib

class HIDProfile(dbus.service.Object):
    def __init__(self, bus):
        self.path = "/org/bluez/example/service"
        dbus.service.Object.__init__(self, bus, self.path)

        self.bus = bus
        self.profile_manager = dbus.Interface(
            bus.get_object("org.bluez", "/org/bluez"),
            "org.bluez.ProfileManager1",
        )

        self.options = {
            "Service": "1124",
            "Name": "Bluetooth HID",
            "Description": "Bluetooth HID Device",
            "Provider": "Raspberry Pi",
            "PSM": dbus.UInt16(0x11),
            "RequireAuthentication": dbus.Boolean(True),
            "RequireAuthorization": dbus.Boolean(False),
        }

        self.profile_manager.RegisterProfile(
            self.path, "00001124-0000-1000-8000-00805f9b34fb", self.options
        )

    @dbus.service.method("org.bluez.Profile1", in_signature="", out_signature="")
    def Release(self):
        print("Profile Released")

dbus.mainloop.glib.DBusGMainLoop(set_as_default=True)
bus = dbus.SystemBus()

profile = HIDProfile(bus)
loop = GLib.MainLoop()
print("HID profile registered. Waiting for connections...")
loop.run()
