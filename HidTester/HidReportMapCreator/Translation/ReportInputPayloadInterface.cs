using HidReportMapCreator.Definition;

namespace HidReportMapCreator.Translation;

public class ReportInputPayloadInterface
{
    public int ReportId { get; set; }
    public List<Field> Fields { get; set; } = new();
}

public class Field
{
    public string Comment { get; set; }
    
    public Input Input { get; set; }
    
    public int BitSize { get; set; }
    
    public int Padding { get; set; }
    
    public int Index { get; set; }
}