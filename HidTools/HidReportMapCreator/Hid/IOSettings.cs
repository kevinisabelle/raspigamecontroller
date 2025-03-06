namespace HidReportMapCreator.Hid;

public class IOSettings
{
    // Bit 0: {Data (0) | Constant (1)}
    /// <summary>
    /// Data (0) or Constant (1)
    /// </summary>
    public const int DATA_CONSTANT = 0x01;       // Mask for bit 0

    
    // Bit 1: {Array (0) | Variable (1)}
    
    /// <summary>
    /// Array (0) or Variable (1)
    /// </summary>
    public const int VARIABLE = 0x02;        // Mask for bit 1

    // Bit 2: {Absolute (0) | Relative (1)}
    /// <summary>
    /// Absolute (0) or Relative (1)
    /// </summary>
    public const int RELATIVE = 0x04;     // Mask for bit 2

    // Bit 3: {No Wrap (0) | Wrap (1)}
    /// <summary>
    /// No Wrap (0) or Wrap (1)
    /// </summary>
    public const int WRAP = 0x08;          // Mask for bit 3

    // Bit 4: {Linear (0) | Non Linear (1)}
    public const int NON_LINEAR = 0x10;     // Mask for bit 4

    // Bit 5: {Preferred State (0) | No Preferred (1)}
    public const int NO_PREFERRED_STATE = 0x20;       // Mask for bit 5

    // Bit 6: {No Null position (0) | Null state (1)}
    public const int NULLABLE = 0x40;          // Mask for bit 6

    // Bit 7: Reserved (always 0)

    // Bit 8: {Bit Field (0) | Buffered Bytes (1)}
    public const int BUFFERED_BYTES = 0x100; // Mask for bit 8

    // Bits 9-31: Reserved (always 0)
}