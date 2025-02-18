namespace HidReportMapCreator.Hid;

public static class HidUsagePage
{
    public const int Undefined                  = 0x00;
    public const int GenericDesktopControls     = 0x01;
    public const int SimulationControls         = 0x02;
    public const int VRControls                 = 0x03;
    public const int SportControls              = 0x04;
    public const int GameControls               = 0x05;
    public const int GenericDeviceControls      = 0x06;
    public const int KeyboardKeypad             = 0x07;
    public const int LEDs                       = 0x08;
    public const int Button                     = 0x09;
    public const int Ordinal                    = 0x0A;
    public const int Telephony                  = 0x0B;
    public const int Consumer                   = 0x0C;
    public const int Digitizer                  = 0x0D;
    // 0x0E is reserved.
    public const int PID                        = 0x0F;
    public const int Unicode                    = 0x10;
    
    // Additional Usage Pages (not an exhaustive list)
    public const int AlphanumericDisplay        = 0x14;
    public const int MedicalInstruments         = 0x40;
    public const int Monitor                    = 0x80;
    public const int MonitorEnumeratedControls  = 0x81;
    public const int VESAVirtualControls        = 0x82;
    // 0x83 - 0x87 reserved.
    public const int BarcodeScanner             = 0x8C;
    public const int Scale                      = 0x8D;
    public const int MagneticStripeReader       = 0x8E;
    public const int CameraControl              = 0x90;
    public const int Arcade                     = 0x91;
}