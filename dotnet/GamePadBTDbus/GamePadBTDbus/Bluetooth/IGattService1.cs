using Tmds.DBus;

namespace GamePadBTDbus.Bluetooth;

[DBusInterface("org.bluez.GattService1")]
public interface IGattService1 : IDBusObject
{
    string UUID { get; }
    bool Primary { get; }
    ObjectPath[] Characteristics { get; }
}