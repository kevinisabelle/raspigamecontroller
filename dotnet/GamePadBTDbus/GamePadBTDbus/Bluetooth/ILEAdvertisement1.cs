using Tmds.DBus;

namespace GamePadBTDbus.Bluetooth;

[DBusInterface("org.bluez.LEAdvertisement1")]
public interface ILEAdvertisement1 : IDBusObject
{
    // These properties will be exported
    string Type { get; }
    string[] ServiceUUIDs { get; }
    bool IncludeTxPower { get; }
    ushort Appearance { get; }
    Task ReleaseAsync();
}