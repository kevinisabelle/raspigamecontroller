import dbus
import dbus.service
import dbus.mainloop.glib
from gi.repository import GLib
import Constants
from BluezImpl import Agent, register_advertisement, register_agent, register_application
from HidGamepadReport import GamepadDefinition, GamepadControl, HIDControlType
from HidServiceImpl import GamePadAdvertisment, Application
from UpdaterService import GamepadUpdater

gamepadDef = GamepadDefinition("KiGP", [
    

    #GamepadControl(HIDControlType.ROTARY_ENCODER, "R1", 1, -127, 127, 0x00),
    #GamepadControl(HIDControlType.ROTARY_ENCODER, "R2", 1, -127, 127, 0x00),
    #GamepadControl(HIDControlType.ROTARY_ENCODER, "R3", 1, -127, 127, 0x00),
    #GamepadControl(HIDControlType.ROTARY_ENCODER, "R4", 1, -127, 127, 0x00),

    #GamepadControl(HIDControlType.SLIDER, "SL1234", 4, 0, 127, 0x00),
    #GamepadControl(HIDControlType.SLIDER, "SL2", 1, 0, 127, 0x00),
    #GamepadControl(HIDControlType.SLIDER, "SL3", 1, 0, 127, 0x00),
    #GamepadControl(HIDControlType.SLIDER, "SL4", 1, 0, 127, 0x00),

    #GamepadControl(HIDControlType.BUTTON, "B123456", 6, 0, 1, 0x00),

    #GamepadControl(HIDControlType.POT, "PT1", 1, 0, 127, 0x00),
    #GamepadControl(HIDControlType.POT, "PT2", 1, 0, 127, 0x00),
    #GamepadControl(HIDControlType.POT, "PT3", 1, 0, 127, 0x00),
    #GamepadControl(HIDControlType.POT, "PT4", 1, 0, 127, 0x00),

    GamepadControl(HIDControlType.JOYSTICK, "J1", 2, -127, 127, 0x00235476),
    #GamepadControl(HIDControlType.JOYSTICK, "J2", 1, -127, 127, 0x00),

])

def main():
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
    print("Report Map Bytes:", " ".join(f"{b:02X}" for b in gamepadDef.get_report_map_bytes()))

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
