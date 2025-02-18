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
                /*new Input
                {
                    Name = "Joystick",
                    Type = InputType.Joystick,
                    Group = "Axes",
                    Min = -127,
                    Max = 127,
                    Count = 1
                },*/
                new Input
                {
                    Name = "Btn1",
                    Type = InputType.Button,
                    Group = "Buttons",
                    Count = 1
                },
                new Input
                {
                    Name = "Btn2",
                    Type = InputType.Button,
                    Group = "Buttons",
                    Count = 1
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
                    Name = "Slider2",
                    Type = InputType.Slider,
                    Count = 1,
                    Min = 0,
                    Max = 255
                },
  
            ]
        };

        return device;
    }
}