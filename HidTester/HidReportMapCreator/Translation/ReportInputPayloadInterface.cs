using System.Text;
using HidReportMapCreator.Definition;

namespace HidReportMapCreator.Translation;

public class ReportInputPayloadInterface
{
    public int ReportId { get; set; }
    public List<Field> Fields { get; set; } = new();

    public string GetPayloadLine(int lineNb = 1)
    {
        var result = "";

        foreach (var field in Fields)
        {
            var fieldStr = lineNb switch
            {
                1 => field.Input.Name.Substring(0, 1),
                2 => (field.Index + 1).ToString(),
                _ => "_"
            };

            if (field.Index == -1)
            {
                fieldStr = "_";
            }
    
            var numberOfOccurences = field.BitSize;
    
            // Repeat the full field string the number of times it occurs
            result += new string(Enumerable.Range(0, numberOfOccurences).SelectMany(i => fieldStr).ToArray());
    
            if (field.Padding > 0)
            {
                // Print the padding character the number of times it occurs
                result += new string(Enumerable.Range(0, field.Padding).SelectMany(i => "_".ToCharArray()).ToArray());
            }
        }

        result = string.Join(" ", Enumerable.Range(0, result.Length / 8).Select(i => result.Substring(i * 8, 8)));
        
        return result;
    }
    
    public string GeneratePythonClassCode()
    {
        var sb = new StringBuilder();

        // Name the Python class using the ReportId
        string className = $"GamepadValues{ReportId}";
        sb.AppendLine($"class {className}:");

        // Prepare __init__ method
        // Only non-padding fields (Index != -1) become parameters.
        var initParams = string.Join(", ", Fields
            .Where(f => f.Index != -1 && f.BitSize > 0)
            .Select(f => $"{f.Input.Name}{f.Index}=0"));
        sb.AppendLine($"    def __init__(self, {initParams}):");

        // For non-padding fields, assign with proper bitmask.
        foreach (var field in Fields.Where(f => f.Index != -1 && f.BitSize > 0))
        {
            int mask = (1 << field.BitSize) - 1;
            string fieldName = $"{field.Input.Name}{field.Index}";
            sb.AppendLine($"        self.{fieldName} = {fieldName} & 0x{mask:X}");
        }
        // For dedicated padding fields, assign 0.
        int paddingCount = 1;
        foreach (var field in Fields.Where(f => f.Index == -1))
        {
            string padName = $"padding{paddingCount}";
            sb.AppendLine($"        self.{padName} = 0  # Dedicated padding field");
            paddingCount++;
        }
        sb.AppendLine();

        // Add setter functions for non-padding fields.
        foreach (var field in Fields.Where(f => f.Index != -1 && f.BitSize > 0))
        {
            int mask = (1 << field.BitSize) - 1;
            string fieldName = $"{field.Input.Name}{field.Index}";
            sb.AppendLine($"    def set_{fieldName}(self, value):");
            sb.AppendLine($"        self.{fieldName} = value & 0x{mask:X}");
            sb.AppendLine();
        }

        // Generate the to_bytes method using bitwise packing.
        // First, calculate total bits.
        int totalBits = Fields.Sum(f => f.BitSize + f.Padding);
        int totalBytes = (totalBits + 7) / 8;
        sb.AppendLine("    def get_report(self):");
        sb.AppendLine("        total = 0");

        // Reset padding counter for use in to_bytes.
        paddingCount = 1;
        foreach (var field in Fields)
        {
            if (field.Index != -1 && field.BitSize > 0)
            {
                string fieldName = $"{field.Input.Name}{field.Index}";
                sb.AppendLine($"        total = (total << {field.BitSize}) | self.{fieldName}");
            }
            else if (field.Index == -1)
            {
                // Dedicated padding field: value is always 0.
                sb.AppendLine($"        total = total << {field.BitSize}  # Padding field padding{paddingCount}");
                paddingCount++;
                continue;
            }

            // If the field has additional padding, shift total accordingly.
            if (field.Padding > 0)
            {
                sb.AppendLine($"        total = total << {field.Padding}  # Field-specific padding");
            }
        }

        // # sb.AppendLine($"        total = total << {ReportId} # Report ID"); 
        
        sb.AppendLine($"        result = total.to_bytes({totalBytes}, byteorder='big')");
        
        // Prepend the Report ID to the result.
        sb.AppendLine($"        return bytes([0x{ReportId:X2}] + list(result))");

        return sb.ToString();
    }

}

public class Field
{
    public string Comment { get; set; }
    
    public Input Input { get; set; }
    
    public int BitSize { get; set; }
    
    public int Padding { get; set; }
    
    public int Index { get; set; }
}