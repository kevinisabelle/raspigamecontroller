namespace HciExample.Bluetooth;

public static class HciConstants
{
    // Definitions from Linux headers (bluetooth/hci.h)
    public const int AF_BLUETOOTH = 31;
    public const int SOCK_RAW = 3;
    public const int BTPROTO_HCI = 1;
    
    // HCIGETDEVINFO ioctl command (from bluetooth/hci.h)
    public const uint HCIGETDEVINFO = 0x800448d3;
    
    // HCI channels as defined in BlueZ
    public const ushort HCI_CHANNEL_RAW = 0;
    public const ushort HCI_CHANNEL_USER = 1;

    // Socket options for HCI (from BlueZ code)
    public const int SOL_HCI = 0;
    public const int HCI_FILTER = 2;

    // H4 packet types (commonly defined in BlueZ's headers)
    public const byte BT_H4_CMD_PKT = 0x01;
    public const byte BT_H4_ACL_PKT = 0x02;
    public const byte BT_H4_SCO_PKT = 0x03;
    public const byte BT_H4_EVT_PKT = 0x04;

    // A no-operation command used internally in BlueZ
    public const ushort BT_HCI_CMD_NOP = 0x0000;
}