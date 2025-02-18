using HidReportMapCreator.Hid;

namespace HidReportMapCreator.Definition;

public class Device
{
    public string Name { get; set; }
    
    public DeviceType Type { get; set; }
    
    public string Manufacturer { get; set; }
    
    public List<Input> Inputs { get; set; }
    public List<Output> Outputs { get; set; }
}

public enum DeviceType
{
    Pointer = 0x01,
    Mouse = 0x02,
    Joystick = 0x04,
    Gamepad = 0x05,
    Keyboard = 0x06,
    Keypad = 0x07,
    MultiAxisController = 0x08,
    TabletPCSystemControls = 0x09,
    WaterCoolingDevice = 0x0A,
    ComputerChassisDevice = 0x0B,
    WirelessRadioControls = 0x0C,
    PortableDeviceControl = 0x0D,
    SystemMultiAxisController = 0x0E,
    SpatialController = 0x0F
}

public enum InputType
{
    Undefined,
    /// <summary>
    /// A button or switch
    /// </summary>
    Button = 0x09,
    
    /// <summary>
    /// A joystick control with 2 axes
    /// </summary>
    Joystick = 0x04,
    
    /// <summary>
    /// A linear control that can be moved up or down
    /// </summary>
    Slider = 0x36,
    
    /// <summary>
    /// A rotary control for generating a variable value, normally in the form of a knob
    /// spun by the index finger and thumb. Report values should increase as controls are
    /// spun clockwise. This usage does not follow the HID orientation conventions.
    /// </summary>
    Dial = 0x37,
    
    /// <summary>
    /// A rotary control for generating a variable value, normally rolled, unlike a dial.
    /// Report values should increase as controls are rolled forward, away from the user.
    /// This usage does not follow the HID orientation conventions.
    /// </summary>
    Wheel = 0x38,
    
    /// <summary>
    /// A hat switch. A hat switch is a switch that can be in one of several positions, such as up, down, left, right, or centered.
    /// </summary>
    HatSwitch = 0x39,
}

public static class InputTypeExtensions
{
    public static ushort[] GetUsages(this InputType type)
    {
        return type switch
        {
            InputType.Joystick => new[]
            {
                HidUsagePageGenericDesktopControls.X, HidUsagePageGenericDesktopControls.Y, 
                HidUsagePageGenericDesktopControls.Rx, HidUsagePageGenericDesktopControls.Ry,
                HidUsagePageGenericDesktopControls.Vx, HidUsagePageGenericDesktopControls.Vy,
                HidUsagePageGenericDesktopControls.Vbrx, HidUsagePageGenericDesktopControls.Vbry
            },
            InputType.Slider => new[] { HidUsagePageGenericDesktopControls.Slider },
            InputType.Dial => new[] { HidUsagePageGenericDesktopControls.Dial },
            InputType.Wheel => new[] { HidUsagePageGenericDesktopControls.Wheel },
            InputType.HatSwitch => new[] { HidUsagePageGenericDesktopControls.HatSwitch },
            _ => new ushort[] { }
        };
    }
    
    public static int GetUsagesPerInstance(this InputType type)
    {
        return type switch
        {
            InputType.Joystick => 2,
            _ => 1
        };
    }
    
    public static string GetUsageName(this ushort usageShort)
    {
        return usageShort switch
        {
            HidUsagePageGenericDesktopControls.X => "X",
            HidUsagePageGenericDesktopControls.Y => "Y",
            HidUsagePageGenericDesktopControls.Z => "Z",
            HidUsagePageGenericDesktopControls.Rx => "Rx",
            HidUsagePageGenericDesktopControls.Ry => "Ry",
            HidUsagePageGenericDesktopControls.Rz => "Rz",
            HidUsagePageGenericDesktopControls.Vx => "Vx",
            HidUsagePageGenericDesktopControls.Vy => "Vy",
            HidUsagePageGenericDesktopControls.Vz => "Vz",
            HidUsagePageGenericDesktopControls.Vbrx => "Vbrx",
            HidUsagePageGenericDesktopControls.Vbry => "Vbry",
            HidUsagePageGenericDesktopControls.Vbrz => "Vbrz",
            HidUsagePageGenericDesktopControls.Vno => "Vno",
            HidUsagePageGenericDesktopControls.Slider => "Slider",
            HidUsagePageGenericDesktopControls.Dial => "Dial",
            HidUsagePageGenericDesktopControls.Wheel => "Wheel",
            HidUsagePageGenericDesktopControls.HatSwitch => "Hat Switch",
            _ => "Unspecified"
        };
    }
}

public enum OutputType
{
    Undefined,
    LED,
    Rumble,
    Display,
    Audio,
    Light
}

public enum GroupType
{
    Application,
    Physical,
    Logical
}

public class Input
{
    public string Name { get; set; }
    
    public InputType Type { get; set; }
    
    public string? Group { get; set; }

    public int Min { get; set; } = 0;

    public int Max { get; set; } = 1;

    public int Count { get; set; } = 1;
}

public class Output
{
    public string Name { get; set; }
    
    public OutputType Type { get; set; }
    
    public int? Min { get; set; }
    
    public int? Max { get; set; }
    
    public string? Group { get; set; }
}