using System.Text;

namespace HidReportMapCreator.Translation;

public class Instruction
{
    public byte[] Data { get; set; }
   
    public string? Comment { get; set; }

    public override string ToString()
    {
        var result = "";
        
        foreach (var b in Data)
        {
            result += "0x" + b.ToString("X2") + ", ";
        }
        
        result += $"# {Comment}";
        
        return result;
    }
}

public class ReportMap
{
    public List<Instruction> Instructions { get; set; } = new();
    
    public string GeneratePythonGetReportMapFunction(ReportMap reportMap)
    {
        var sb = new StringBuilder();
        sb.AppendLine("    def get_report_map():");
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