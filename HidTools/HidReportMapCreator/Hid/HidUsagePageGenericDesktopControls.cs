namespace HidReportMapCreator.Hid;

public static class HidUsagePageGenericDesktopControls
{
    public const ushort Pointer = 0x01;
    public const ushort Mouse = 0x02;
    public const ushort Joystick = 0x04;
    public const ushort GamePad = 0x05;
    public const ushort Keyboard = 0x06;
    public const ushort Keypad = 0x07;
    public const ushort MultiAxisController = 0x08;
    public const ushort TabletPCSystemControls = 0x09;
    public const ushort WaterCoolingDevice = 0x0A;
    public const ushort ComputerChassisDevice = 0x0B;
    public const ushort WirelessRadioControls = 0x0C;
    public const ushort PortableDeviceControl = 0x0D;
    public const ushort SystemMultiAxisController = 0x0E;
    public const ushort SpatialController = 0x0F;
        
    public const ushort X = 0x30;
    public const ushort Y = 0x31;
    public const ushort Z = 0x32;
    public const ushort Rx = 0x33;
    public const ushort Ry = 0x34;
    public const ushort Rz = 0x35;
    public const ushort Slider = 0x36;
    public const ushort Dial = 0x37;
    public const ushort Wheel = 0x38;
    public const ushort HatSwitch = 0x39;
    
    public const ushort CountedBuffer = 0x3A;
    public const ushort ByteCount = 0x3B;
    public const ushort MotionWakeup = 0x3C;
    
    public const ushort Start = 0x3D;
    public const ushort Select = 0x3E;
    
    public const ushort Vx = 0x40;
    public const ushort Vy = 0x41;
    public const ushort Vz = 0x42;
    public const ushort Vbrx = 0x43;
    public const ushort Vbry = 0x44;
    public const ushort Vbrz = 0x45;
    public const ushort Vno = 0x46;
    
    public const ushort FeatureNotification = 0x47;
    public const ushort ResolutionMultiplier = 0x48;
    
    public const ushort SystemControl = 0x80;
    public const ushort SystemPowerDown = 0x81;
    public const ushort SystemSleep = 0x82;
    public const ushort SystemWakeUp = 0x83;
    public const ushort SystemContextMenu = 0x84;
    public const ushort SystemMainMenu = 0x85;
    public const ushort SystemAppMenu = 0x86;
    public const ushort SystemMenuHelp = 0x87;
    public const ushort SystemMenuExit = 0x88;
    public const ushort SystemMenuSelect = 0x89;
    public const ushort SystemMenuRight = 0x8A;
    public const ushort SystemMenuLeft = 0x8B;
    public const ushort SystemMenuUp = 0x8C;
    public const ushort SystemMenuDown = 0x8D;
    public const ushort SystemColdRestart = 0x8E;
    public const ushort SystemWarmRestart = 0x8F;
    
    public const ushort DPadUp = 0x90;
    public const ushort DPadDown = 0x91;
    public const ushort DPadRight = 0x92;
    public const ushort DPadLeft = 0x93;
        
}