namespace HidReportMapCreator.Definition;

public class Output
{
    public string Name { get; set; }
    
    public OutputType Type { get; set; }
    
    public int? Min { get; set; }
    
    public int? Max { get; set; }
    
    public string? Group { get; set; }
}