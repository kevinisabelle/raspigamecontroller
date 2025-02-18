namespace HidReportMapCreator.Definition;

public class Device
{
    public string Name { get; set; }
    
    public DeviceType Type { get; set; }
    
    public string Manufacturer { get; set; }
    
    public List<Input> Inputs { get; set; }
    public List<Output> Outputs { get; set; }
}