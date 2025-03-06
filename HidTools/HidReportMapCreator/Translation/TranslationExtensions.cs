using HidReportMapCreator.Definition;
using HidReportMapCreator.Hid;

namespace HidReportMapCreator.Translation;

/// <summary>
/// Contains extension methods for translating the higher abstraction level definitions to HID report maps and payloads structures.
/// </summary>
public static class TranslationExtensions
{
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
            Data = [HidReportField.USAGE, (byte)device.Type.GetUsage()],
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
    
    private static List<Instruction> GetInputInstructions(this Input input)
    {
        var instructions = new List<Instruction>();
        var usagePage = input.Type.GetUsagePage();
        var usesUsage = input.Type.GetUsages().Length > 0;
        var nbOfUsages = input.Type.GetUsagesPerInstance() * input.Count;
        var isSingleUsage = input.Type.GetUsages().Length == 1;
        
        instructions.Add(new Instruction
        {
            Comment = $"Usage Page ({Mappings.GetUsagePageName(usagePage)})",
            Data = [HidReportField.USAGE_PAGE, (byte)usagePage],
        });

        if (input.AxisUsage.HasValue)
        {
            instructions.Add(new Instruction
            {
                Comment = $"Usage ({input.AxisUsage.Value.GetUsage().GetUsageName()})",
                Data = [HidReportField.USAGE, (byte)input.AxisUsage.Value.GetUsage()],
            });
        }
        else
        {

            if (usesUsage)
            {
                if (isSingleUsage)
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

        if (input.Type != InputType.Button)
        {
            instructions.Add(new Instruction
            {
                Comment = $"Physical Minimum ({input.Min})",
                Data = [HidReportField.PHYSICAL_MINIMUM, (byte)(input.Min)],
            });

            instructions.Add(new Instruction
            {
                Comment = $"Physical Maximum ({input.Max})",
                Data = [HidReportField.PHYSICAL_MAXIMUM, (byte)(input.Max)],
            });
        }

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
        
        var inputField = (byte)(input.Type.GetInputField() & 0xFF);
        
        instructions.Add(new Instruction
        {
            // but the format in the comment to 8 bits
            Comment = $"Input ({inputField.ToString("B8")})",
            Data = [HidReportField.INPUT, inputField],
        });
        
        
        if (input.GetReportPaddingSize() > 0)
        {
            instructions.Add(new Instruction
            {
                Comment = $"Report Size ({input.GetReportPaddingSize()})",
                Data = [HidReportField.REPORT_SIZE, (byte)input.GetReportPaddingSize()],
            });
            
            instructions.Add(new Instruction
            {
                Comment = $"Report Count (1)",
                Data = [HidReportField.REPORT_COUNT, 0x01],
            });
            
            var inputFieldValue = (byte)(IOSettings.DATA_CONSTANT & 0xFF);
            
            instructions.Add(new Instruction
            {
                Comment = $"Input ({inputFieldValue.ToString("B8")}) -- Padding",
                Data = [HidReportField.INPUT, inputFieldValue],
            });
        }

        
        return instructions;
    }
    
    public static ReportPayload ToReportPayload(this Device device)
    {
        var payload = new ReportPayload
        {
            ReportId = 0x01
        };

        foreach (var input in device.Inputs)
        {
            var usagesPerInstance = input.Type.GetUsagesPerInstance();
            var totalUsages = input.Count * usagesPerInstance;
            
            var usages = input.Type.GetUsages();
            
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
        }
        
        return payload;
    }
}