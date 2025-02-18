using HidReportMapCreator.Definition;

namespace HidReportMapCreator.Devices;

public class KiGPSimple
{
    public Device Create()
    {
        var device = new Device
        {
            Name = "KiGP",
            Manufacturer = "KiGP",
            Type = DeviceType.Gamepad,
            Inputs =
            [
                new Input
                {
                    Name = "Joystick",
                    Type = InputType.Joystick,
                    Group = "Axes",
                    Min = -127,
                    Max = 127,
                    Count = 1
                },
                new Input
                {
                    Name = "Button",
                    Type = InputType.Button,
                    Group = "Buttons",
                    Count = 2
                }
            ]
        };

        return device;
    }
}