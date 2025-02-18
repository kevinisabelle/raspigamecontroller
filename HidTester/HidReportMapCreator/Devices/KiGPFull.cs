using HidReportMapCreator.Definition;

namespace HidReportMapCreator.Devices;

public class KiGPFull
{
    public Device Create()
    {
        var device = new Device
        {
            Name = "KiGP",
            Manufacturer = "KiGP",
            Type = DeviceType.Gamepad,
            Inputs = new List<Input>
            {
                new Input()
                {
                    Name = "Joystick",
                    Type = InputType.Joystick,
                    Group = "Axes",
                    Min = -127,
                    Max = 127,
                    Count = 2
                },  
                new Input()
                {
                    Name = "Slider",
                    Type = InputType.Slider,
                    Group = "Axes",
                    Count = 4,
                    Min = 0,
                    Max = 255
                },
                new Input()
                {
                    Name = "Rotary 1",
                    Type = InputType.Wheel,
                    Group = "Axes",
                    Count = 4,
                    Min = -127,
                    Max = 127
                },
                new Input()
                {
                    Name = "Pot 1",
                    Type = InputType.Dial,
                    Group = "Axes",
                    Count = 4,
                    Min = 0,
                    Max = 255
                },
                new Input
                {
                    Name = "Button",
                    Type = InputType.Button,
                    Group = "Buttons",
                    Count = 6
                }
            }
        };

        return device;
    }
}