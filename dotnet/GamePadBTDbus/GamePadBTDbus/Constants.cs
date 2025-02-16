namespace GamePadBTDbus;

public static class Constants
{
    public const string BLUEZ_SERVICE = "org.bluez";
    public const ushort ADV_APPEARANCE_GAMEPAD = 0x03C4;
    
    public const string GATT_SERVICE_HID_UUID = "00001812-0000-1000-8000-00805f9b34fb";
    public const string GATT_REPORT_MAP_UUID = "00002A4B-0000-1000-8000-00805f9b34fb";
    public const string GATT_REPORT_UUID = "00002A4D-0000-1000-8000-00805f9b34fb";
    public const string GATT_HID_INFORMATION_UUID = "00002A4A-0000-1000-8000-00805f9b34fb";
    public const string GATT_PROTOCOL_MODE_UUID = "00002A4E-0000-1000-8000-00805f9b34fb";
    
    public const string GATT_DESC_REPORT_REFERENCE_UUID = "00002908-0000-1000-8000-00805f9b34fb";
    public const string GATT_DESC_CLIENT_DESCRIPTOR_UUID = "00002902-0000-1000-8000-00805f9b34fb";
    
    public const string SERVICE_PATH = "/org/bluez/gamepadki/service0";
    public const string ADVERTISEMENT_PATH = "/org/bluez/gamepadki/advertisement0";
    public const string ADAPTER_PATH = "/org/bluez/hci0"; 
}