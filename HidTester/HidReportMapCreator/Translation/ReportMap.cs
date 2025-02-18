using System.Text;

namespace HidReportMapCreator.Translation;

/// <summary>
/// Represents an HID report map. 
/// </summary>
public class ReportMap
{
    public List<Instruction> Instructions { get; set; } = new();
    
    public string GeneratePythonGetReportMapFunction(ReportMap reportMap)
    {
        var sb = new StringBuilder();
        sb.AppendLine("    def get_report_map(self):");
        sb.AppendLine("        return bytes([");
    
        foreach (var instruction in reportMap.Instructions)
        {
            sb.Append("            ");
            // Write each byte as 0xXX, with a trailing comma.
            foreach (var b in instruction.Data)
            {
                sb.Append($"0x{b:X2}, ");
            }
            // Append the comment if it exists.
            if (!string.IsNullOrEmpty(instruction.Comment))
            {
                sb.Append($"  # {instruction.Comment}");
            }
            sb.AppendLine();
        }
    
        sb.AppendLine("        ])");
        return sb.ToString();
    }
}