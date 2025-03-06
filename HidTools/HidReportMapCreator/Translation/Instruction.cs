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