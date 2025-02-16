using GamePadBTDbus;
using Tmds.DBus.Protocol;

namespace bluez.DBus;

public class LEAdvertisement : bluezObject
{
    private const string __Interface = "org.bluez.LEAdvertisement1";
    public ObjectPath ObjectPath { get; }
    public LEAdvertisement(bluezService service, ObjectPath path, string type) : base(service, path)
    {
        ObjectPath = path;
        Type = type;
        // Default empty lists
        ServiceUUIDs = new string[0];
        IncludeTxPower = false;
        Appearance = Constants.ADV_APPEARANCE_GAMEPAD; // Gamepad appearance
    }
    
    public string Type { get; }
    public string[] ServiceUUIDs { get; set; }
    public bool IncludeTxPower { get; set; }
    public ushort Appearance { get; }
    public Task ReleaseAsync()
    {
        Console.WriteLine($"{ObjectPath}: Released!");
        return Task.CompletedTask;
    }

}