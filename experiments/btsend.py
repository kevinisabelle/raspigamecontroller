import dbus
import dbus.lowlevel

def main():
    bus = dbus.SystemBus()

    # Construct a low-level method call message.
    message = dbus.lowlevel.MethodCallMessage(
        destination='org.bluez',
        path='/com/kevinisabelle/gamepad/agent',
        interface='org.bluez.Agent1',
        method='RequestConfirmation'
    )
    # Append the arguments with the correct DBus types.
    message.append(dbus.ObjectPath('/org/bluez/hci0/dev_B0_A4_60_E7_88_52'))
    message.append(dbus.UInt32(123456))

    # Pass the timeout as a positional argument.
    reply = bus.send_message_with_reply_and_block(message, 30)
    print("Reply:", reply)

if __name__ == '__main__':
    main()
