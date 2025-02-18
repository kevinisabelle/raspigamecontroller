using HidReportMapCreator.Definition;
using HidReportMapCreator.Hid;

namespace HidReportMapCreator.Translation;

public static class DeviceExtensions
{
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

    public static string GetUsagePageName(int usagePage)
    {
        return usagePage switch
        {
            HidUsagePage.Button => "Button",
            HidUsagePage.GenericDesktopControls => "Generic Desktop Controls",
            HidUsagePage.KeyboardKeypad => "Keyboard/Keypad",
            _ => "Undefined"
        };
    }
    
    public static int GetValueBitSize(this Input input)
    {
        return (int)Math.Ceiling(Math.Log(input.Max - input.Min + 1, 2));
    }
    
    public static int GetValuePadding(this Input input)
    {
        var requiredBitsForValue = input.GetValueBitSize();
        return requiredBitsForValue >= 5 ? 8 - requiredBitsForValue : 0;
    }
    
    public static int GetReportPaddingSize(this Input input)
    {
        var bitsInPayload = (input.GetValueBitSize()) * input.Count;
        var uncompletedBytesBits = bitsInPayload % 8;
        
        return uncompletedBytesBits == 0 ? 0 : 8 - uncompletedBytesBits;
    }

    public static List<Instruction> GetInputInstructions(this Input input)
    {
        var instructions = new List<Instruction>();
        var usagePage = input.Type.GetUsagePage();
        var usesUsage = input.Type.GetUsages().Length > 0;
        var nbOfUsages = input.Type.GetUsagesPerInstance() * input.Count;
        var isSingleUsage = input.Type.GetUsages().Length == 1;
        
        instructions.Add(new Instruction
        {
            Comment = $"Usage Page ({GetUsagePageName(usagePage)})",
            Data = [HidReportField.USAGE_PAGE, (byte)usagePage],
        });
        
        if (usesUsage)
        {
            if  (isSingleUsage)
            {
                instructions.Add(new Instruction
                {
                    Comment = $"Usage ({input.Type.GetUsages().First().GetUsageName()})",
                    Data = [HidReportField.USAGE, (byte)input.Type.GetUsages().First()],
                });
            }
            else
            {
                for (var i = 0; i < nbOfUsages; i++)
                {
                    var usage = input.Type.GetUsages().ElementAtOrDefault(i);
                
                    // Use the first usage if index out of bounds
                    if (usage == 0)
                    {
                        usage = input.Type.GetUsages().First();
                    }

                    instructions.Add(new Instruction
                    {
                        Comment = $"Usage ({usage.GetUsageName()})",
                        Data = [HidReportField.USAGE, (byte)usage],
                    });
                }
            }
        }

        instructions.Add(new Instruction
        {
            Comment = $"Usage Minimum (1)",
            Data = [HidReportField.USAGE_MINIMUM, 0x01],
        });
        
        instructions.Add(new Instruction
        {
            Comment = $"Usage Maximum ({input.Count * input.Type.GetUsagesPerInstance()})",
            Data = [HidReportField.USAGE_MAXIMUM, (byte)(input.Count * input.Type.GetUsagesPerInstance())],
        });
        
        instructions.Add(new Instruction
        {
            Comment = $"Logical Minimum ({input.Min})",
            Data = [HidReportField.LOGICAL_MINIMUM, (byte)(input.Min)],
        });
        
        instructions.Add(new Instruction
        {
            Comment = $"Logical Maximum ({input.Max})",
            Data = [HidReportField.LOGICAL_MAXIMUM, (byte)(input.Max)],
        });
        
        instructions.Add(new Instruction
        {
            Comment = $"Report Size ({input.GetValueBitSize()})",
            Data = [HidReportField.REPORT_SIZE, (byte)input.GetValueBitSize()],
        });

        instructions.Add(new Instruction
        {
            Comment = $"Report Count ({input.Count * input.Type.GetUsagesPerInstance()})",
            Data = [HidReportField.REPORT_COUNT, (byte)(input.Count * input.Type.GetUsagesPerInstance())],
        });
        
        instructions.Add(new Instruction
        {
            Comment = $"Input (Variable, Absolute, No Wrap, Linear, Preferred State, No Null Position, Bit Field)",
            Data = [HidReportField.INPUT, IOSettings.VARIABLE & 0xFF],
        });
        
        
        if (input.GetReportPaddingSize() > 0)
        {
            instructions.Add(new Instruction
            {
                Comment = $"Padding ({input.GetReportPaddingSize()})",
                Data = [HidReportField.REPORT_SIZE, (byte)input.GetReportPaddingSize()],
            });
            
            instructions.Add(new Instruction
            {
                Comment = $"Padding Count (1)",
                Data = [HidReportField.REPORT_COUNT, 0x01],
            });
            
            instructions.Add(new Instruction
            {
                Comment = $"Padding (Constant, Array, Absolute)",
                Data = [HidReportField.INPUT, 0x00],
            });
        }

        
        return instructions;
    }
    
    public static ReportMap ToReportMap(this Device device)
    {
        var reportMap = new ReportMap();
        
        reportMap.Instructions.Add(new Instruction
        {
            Comment = "Usage Page (Generic Desktop Controls)",
            Data = [HidReportField.USAGE_PAGE, HidUsagePage.GenericDesktopControls],
        });
        
        reportMap.Instructions.Add(new Instruction
        {
            Comment = $"Usage ({device.Type})",
            Data = [HidReportField.USAGE, (byte)device.Type.GetHashCode()],
        });
        
        reportMap.Instructions.Add(new Instruction
        {
            Comment = "Collection (Application)",
            Data = [HidReportField.COLLECTION, HidCollectionType.Application],
        });
        
        reportMap.Instructions.Add(new Instruction
        {
            Comment = "Report ID 1",
            Data = [HidReportField.REPORT_ID, 0x01],
        });

        foreach (var input in device.Inputs)
        {
            reportMap.Instructions.AddRange(input.GetInputInstructions());
        }
        
        reportMap.Instructions.Add(new Instruction
        {
            Comment = "End Collection",
            Data = [HidReportField.END_COLLECTION],
        });
        
        return reportMap;
    }
    
    public static ReportInputPayloadInterface ToReportPayload(this Device device)
    {
        var payload = new ReportInputPayloadInterface
        {
            ReportId = 0x01
        };

        foreach (var input in device.Inputs)
        {
            var usagesPerInstance = input.Type.GetUsagesPerInstance();
            var totalUsages = input.Count * usagesPerInstance;
            
            var usages = input.Type.GetUsages();
            
            for (var i = 0; i < totalUsages; i++)
            {
                var usage = usages.ElementAtOrDefault(i);
                
                if (usage == 0)
                {
                    usage = usages.FirstOrDefault();
                }
                
                payload.Fields.Add(new Field()
                {
                    Comment = $"{usage.GetUsageName()}",
                    BitSize = input.GetValueBitSize(),
                    Padding = 0,
                    Index = i,
                    Input = input
                });
            }
            
            if (input.GetReportPaddingSize() > 0)
            {
                payload.Fields.Add(new Field()
                {
                    Comment = $"_ {input.Name} {input.GetReportPaddingSize()} Padding",
                    BitSize = input.GetReportPaddingSize(),
                    Padding = 0,
                    Index = -1,
                    Input = input
                });
            }
        }
        
        return payload;
    }
}