using Tmds.DBus;
using ObjectPath = Tmds.DBus.Protocol.ObjectPath;

namespace GamePadBTDbus.Bluetooth;

[DBusInterface("org.bluez.GattCharacteristic1")]
public interface IGattCharacteristic1 : IDBusObject
{
    string UUID { get; }
    ObjectPath Service { get; }
    string[] Flags { get; }
    ObjectPath[] Descriptors { get; }
    Task<byte[]> ReadValueAsync(IDictionary<string, object> options);
    Task WriteValueAsync(byte[] value, IDictionary<string, object> options);
    Task StartNotifyAsync();
    Task StopNotifyAsync();
    // (PropertiesChanged signal omitted for brevity)
}