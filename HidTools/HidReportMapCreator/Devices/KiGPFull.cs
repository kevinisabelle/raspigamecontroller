using HidReportMapCreator.Definition;

namespace HidReportMapCreator.Devices;

public class KiGPFull
{
    public Device Create()
    {
        var device = new Device
        {
            Name = "KiGPFull",
            Manufacturer = "KiGP",
            Type = DeviceType.Gamepad,
            Inputs =
            [
                new Input
                {
                    Name = "Btn1",
                    Type = InputType.Button,
                    Count = 8
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
                    Name = "AxisX",
                    Type = InputType.Axis,
                    Count = 1,
                    Min = -127,
                    Max = 127,
                    AxisUsage = AxisUsage.X
                },
                new Input
                {
                    Name = "AxisY",
                    Type = InputType.Axis,
                    Count = 1,
                    Min = 0,
                    Max = 255,
                    AxisUsage = AxisUsage.Y
                },
                new Input
                {
                    Name = "AxisZ",
                    Type = InputType.Axis,
                    Count = 1,
                    Min = 0,
                    Max = 255,
                    AxisUsage = AxisUsage.Z
                },
                new Input
                {
                    Name = "AxisRx",
                    Type = InputType.Axis,
                    Count = 1,
                    Min = 0,
                    Max = 255,
                    AxisUsage = AxisUsage.Rx
                },
                new Input
                {
                    Name = "AxisRy",
                    Type = InputType.Axis,
                    Count = 1,
                    Min = 0,
                    Max = 255,
                    AxisUsage = AxisUsage.Ry
                },
                new Input
                {
                    Name = "AxisRz",
                    Type = InputType.Axis,
                    Count = 1,
                    Min = 0,
                    Max = 255,
                    AxisUsage = AxisUsage.Rz
                },
                new Input
                {
                    Name = "AxisVx",
                    Type = InputType.Slider,
                    Count = 1,
                    Min = 0,
                    Max = 255,
                    AxisUsage = AxisUsage.Slider
                },

            ]
        };
        
        return device;
    }
}