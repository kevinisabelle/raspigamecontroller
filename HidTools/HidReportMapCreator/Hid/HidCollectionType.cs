namespace HidReportMapCreator.Hid;

public class HidCollectionType
{
    /// <summary>
    /// A physical collection is used for a set of data items that represent 
    /// data points collected at one geometric point. This is useful for 
    /// sensing devices which may need to associate sets of measured or 
    /// sensed data with a single point. It does not indicate that a set of 
    /// data values comes from one device, such as a keyboard. In the 
    /// case of device which reports the position of multiple sensors, 
    /// physical collections are used to show which data comes from 
    /// each separate sensor.
    /// </summary>
    public const int Physical = 0x00;
    
    /// <summary>
    /// A group of Main items that might be familiar to applications. It 
    /// could also be used to identify item groups serving different 
    /// purposes in a single device. Common examples are a keyboard or 
    /// mouse. A keyboard with an integrated pointing device could be 
    /// defined as two different application collections. Data reports are 
    /// usually (but not necessarily) associated with application 
    /// collections (at least one report ID per application). 
    /// </summary>
    public const int Application = 0x01;
    
    /// <summary>
    /// A logical collection is used when a set of data items form a 
    /// composite data structure. An example of this is the association 
    /// between a data buffer and a byte count of the data. The 
    /// collection establishes the link between the count and the buffer.
    /// </summary>
    public const int Logical = 0x02;
    
    /// <summary>
    /// Defines a logical collection that wraps all the fields in a report. A 
    /// unique report ID will be contained in this collection. An 
    /// application can easily determine whether a device supports a 
    /// certain function. Note that any valid Report ID value can be 
    /// declared for a Report collection. 
    /// </summary>
    public const int Report = 0x03;
    
    /// <summary>
    /// The collection is a named array of data.
    /// </summary>
    public const int NamedArray = 0x04;
    public const int UsageSwitch = 0x05;
    public const int UsageModifier = 0x06;
    
}