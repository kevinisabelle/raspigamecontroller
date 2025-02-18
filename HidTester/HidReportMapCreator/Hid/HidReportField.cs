namespace HidReportMapCreator.Hid;

public static class HidReportField
{
    // Global Items

    public const int USAGE_PAGE       = 0x05;
    public const int LOGICAL_MINIMUM  = 0x15;
    public const int LOGICAL_MAXIMUM  = 0x25;
    public const int PHYSICAL_MINIMUM = 0x35;
    public const int PHYSICAL_MAXIMUM = 0x45;
    public const int UNIT_EXPONENT    = 0x55;
    public const int UNIT             = 0x65;
    public const int REPORT_SIZE      = 0x75;
    public const int REPORT_ID        = 0x85;
    public const int REPORT_COUNT     = 0x95;

    // Local Items
    public const int USAGE            = 0x09;
    public const int USAGE_MINIMUM    = 0x19;
    public const int USAGE_MAXIMUM    = 0x29;

    // Main Items
    public const int INPUT            = 0x81;
    public const int OUTPUT           = 0x91;
    public const int FEATURE          = 0xB1;
    public const int COLLECTION       = 0xA1;
    public const int END_COLLECTION   = 0xC0;
}