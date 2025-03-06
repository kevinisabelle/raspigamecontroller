namespace HidReportMapCreator.Definition;

public class Input
{
    public string Name { get; set; }
    
    public InputType Type { get; set; }
    
    public AxisUsage? AxisUsage { get; set; }
    
    public string? Group { get; set; }

    public int Min { get; set; } = 0;

    public int Max { get; set; } = 1;

    public int Count { get; set; } = 1;
    
    public int GetValueBitSize()
    {
        return (int)Math.Ceiling(Math.Log(Max - Min + 1, 2));
    }
    
    public int GetReportPaddingSize()
    {
        var bitsInPayload = (GetValueBitSize()) * Count;
        var uncompletedBytesBits = bitsInPayload % 8;
        
        return uncompletedBytesBits == 0 ? 0 : 8 - uncompletedBytesBits;
    }
}