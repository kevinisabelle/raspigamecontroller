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
                    Name = "Btn1",
                    Type = InputType.Button,
                    Group = "Buttons",
                    Count = 2
                },
                new Input
                {
                    Name = "Slider",
                    Type = InputType.Slider,
                    Count = 1,
                    Min = 0,
                    Max = 255
                },
                new Input
                {
                    Name = "Dial1",
                    Type = InputType.Dial,
                    Count = 1,
                    Min = -127,
                    Max = 127
                },
                new Input
                {
                    Name = "Dial2",
                    Type = InputType.Dial,
                    Count = 1,
                    Min = -127,
                    Max = 127
                },
            ]
        };

        return device;
    }
}