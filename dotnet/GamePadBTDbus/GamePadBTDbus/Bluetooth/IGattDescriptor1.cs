using Tmds.DBus;

namespace GamePadBTDbus.Bluetooth;

[DBusInterface("org.bluez.GattDescriptor1")]
public interface IGattDescriptor1 : IDBusObject
{
    string UUID { get; }
    ObjectPath Characteristic { get; }
    Task<byte[]> ReadValueAsync(IDictionary<string, object> options);
}