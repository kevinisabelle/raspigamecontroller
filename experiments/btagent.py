#!/usr/bin/env python3
import dbus
import dbus.exceptions
import dbus.mainloop.glib
import dbus.service
from gi.repository import GLib

AGENT_PATH = "/com/kevinisabelle/gamepad/agent"

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

def register_agent(bus, capability="KeyboardDisplay"):
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
# Main: register objects manually on D-Bus with our custom introspection  
# -----------------------------------------------------------------------------
def main():
    print("Starting pairing agent...")
    dbus.mainloop.glib.DBusGMainLoop(set_as_default=True)
    bus = dbus.SystemBus()

    # Register our DisplayYesNo pairing agent
    agent = Agent(bus)
    register_agent(bus, "KeyboardDisplay")

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

    loop = GLib.MainLoop()
    print("Running main loop")
    loop.run()

if __name__ == "__main__":
    main()
