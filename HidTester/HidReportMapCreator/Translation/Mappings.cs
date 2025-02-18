using HidReportMapCreator.Definition;
using HidReportMapCreator.Hid;

namespace HidReportMapCreator.Translation;

/// <summary>
/// Contains mappings for the different types of devices and inputs.
/// This class is used to map the device and input types to their respective HID usages.
/// </summary>
public static class Mappings
{
    public static int GetUsage(this DeviceType type)
    {
        return type switch
        {
            DeviceType.Joystick => HidUsagePageGenericDesktopControls.Joystick,
            DeviceType.Gamepad => HidUsagePageGenericDesktopControls.GamePad,
            _ => HidUsagePage.Undefined
        };
    }
    
    public static int GetUsagePage(this InputType type)
    {
        return type switch
        {
            InputType.Button => HidUsagePage.Button,
            InputType.HatSwitch => HidUsagePage.GenericDesktopControls,
            InputType.Slider => HidUsagePage.GenericDesktopControls,
            InputType.Dial => HidUsagePage.GenericDesktopControls,
            InputType.Wheel => HidUsagePage.GenericDesktopControls,
            InputType.Joystick => HidUsagePage.GenericDesktopControls,
            _ => HidUsagePage.Undefined
        };
    }

    public static byte GetInputField(this InputType inputType)
    {
        return inputType switch
        {
            InputType.Button => IOSettings.VARIABLE,
            InputType.Joystick => IOSettings.VARIABLE,
            InputType.Slider => IOSettings.VARIABLE,
            InputType.Dial => IOSettings.VARIABLE | IOSettings.RELATIVE,
            InputType.Wheel => IOSettings.VARIABLE,
            InputType.HatSwitch => IOSettings.VARIABLE,
            _ => 0x00
        };
    }
    
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
    
    public static string GetUsagePageName(int usagePage)
    {
        return usagePage switch
        {
            HidUsagePage.Button => "Button",
            HidUsagePage.GenericDesktopControls => "Generic Desktop Controls",
            HidUsagePage.KeyboardKeypad => "Keyboard/Keypad",
            HidUsagePage.SimulationControls => "Simulation Controls",
            _ => "Undefined"
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