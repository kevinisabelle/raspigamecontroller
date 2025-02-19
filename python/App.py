import dbus
import dbus.service
import dbus.mainloop.glib
from gi.repository import GLib
import Constants
from BluezImpl import Agent, register_advertisement, register_agent, register_application
from HidServiceImpl import GamePadAdvertisment, Application
from UpdaterService import GamepadUpdater
from GamepadValues import GamepadValues1
from Hardware import init_hardware

def main():

    init_hardware()

    gamepadDef = GamepadValues1()

    # Initialiser la boucle principale de GLib pour dbus
    dbus.mainloop.glib.DBusGMainLoop(set_as_default=True)
    
    print("Starting GamepadKi...")
    bus = dbus.SystemBus()

    mainloop = GLib.MainLoop()

    agent = Agent(bus, Constants.AGENT_PATH)
    register_agent(bus, agent)
    
    app = Application(bus, gamepadDef)
    register_application(bus, app, mainloop)
    
    advertisement = GamePadAdvertisment(bus, 0)
    register_advertisement(bus, advertisement)

    # Print the report map bytes for debugging in hex format
    reportMap = gamepadDef.get_report_map()
    print("Report Map Bytes:", " ".join(f"{b:02X}" for b in reportMap))

    gamepadUpdater = GamepadUpdater(gamepadDef, app)
    gamepadUpdater.start()

    try:
        mainloop.run()
    except KeyboardInterrupt:
        print("Exiting...")
        gamepadUpdater.stop()
        mainloop.quit()

if __name__ == "__main__":
    main()
