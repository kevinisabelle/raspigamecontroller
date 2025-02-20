class GamepadValues1:
    def __init__(self, Btn10=0, Btn11=0, Slider0=0, Dial10=0, Dial20=0):
        self.Btn10 = Btn10 & 0x1
        self.Btn11 = Btn11 & 0x1
        self.Slider0 = Slider0 & 0xFF
        self.Dial10 = Dial10 & 0xFF
        self.Dial20 = Dial20 & 0xFF
        self.padding1 = 0  # Dedicated padding field

    def set_Btn10(self, value):
        self.Btn10 = value & 0x1

    def set_Btn11(self, value):
        self.Btn11 = value & 0x1

    def set_Slider0(self, value):
        self.Slider0 = value & 0xFF

    def set_Dial10(self, value):
        self.Dial10 = value & 0xFF

    def set_Dial20(self, value):
        self.Dial20 = value & 0xFF

    def get_report(self):
        total = 0
        total = total << 6  # Padding field padding1
        total = (total << 1) | self.Btn10
        total = (total << 1) | self.Btn11
        total = (total << 8) | self.Slider0
        total = (total << 8) | self.Dial10
        total = (total << 8) | self.Dial20
        result = total.to_bytes(4, byteorder='big')
        return bytes([0x01] + list(result))


    def get_report_map(self):
        return bytes([
            0x05, 0x01,   # Usage Page (Generic Desktop Controls)
            0x09, 0x05,   # Usage (Gamepad)
            0xA1, 0x01,   # Collection (Application)
            0x85, 0x01,   # Report ID 1
            0x05, 0x09,   # Usage Page (Button)
            0x19, 0x01,   # Usage Minimum (1)
            0x29, 0x02,   # Usage Maximum (2)
            0x15, 0x00,   # Logical Minimum (0)
            0x25, 0x01,   # Logical Maximum (1)
            0x75, 0x01,   # Report Size (1)
            0x95, 0x02,   # Report Count (2)
            0x81, 0x02,   # Input (00000010)
            0x75, 0x06,   # Report Size (6)
            0x95, 0x01,   # Report Count (1)
            0x81, 0x01,   # Input (00000001) -- Padding
            0x05, 0x01,   # Usage Page (Generic Desktop Controls)
            0x09, 0x36,   # Usage (Slider)
            0x19, 0x01,   # Usage Minimum (1)
            0x29, 0x01,   # Usage Maximum (1)
            0x15, 0x00,   # Logical Minimum (0)
            0x25, 0xFF,   # Logical Maximum (255)
            0x35, 0x00,   # Physical Minimum (0)
            0x45, 0xFF,   # Physical Maximum (255)
            0x75, 0x08,   # Report Size (8)
            0x95, 0x01,   # Report Count (1)
            0x81, 0x02,   # Input (00000010)
            0x05, 0x01,   # Usage Page (Generic Desktop Controls)
            0x09, 0x37,   # Usage (Dial)
            0x19, 0x01,   # Usage Minimum (1)
            0x29, 0x01,   # Usage Maximum (1)
            0x15, 0x00,   # Logical Minimum (0)
            0x25, 0xFF,   # Logical Maximum (255)
            0x35, 0x00,   # Physical Minimum (0)
            0x45, 0xFF,   # Physical Maximum (255)
            0x75, 0x08,   # Report Size (8)
            0x95, 0x01,   # Report Count (1)
            0x81, 0x06,   # Input (00000110)
            0x05, 0x01,   # Usage Page (Generic Desktop Controls)
            0x09, 0x37,   # Usage (Dial)
            0x19, 0x01,   # Usage Minimum (1)
            0x29, 0x01,   # Usage Maximum (1)
            0x15, 0x00,   # Logical Minimum (0)
            0x25, 0xFF,   # Logical Maximum (255)
            0x35, 0x00,   # Physical Minimum (0)
            0x45, 0xFF,   # Physical Maximum (255)
            0x75, 0x08,   # Report Size (8)
            0x95, 0x01,   # Report Count (1)
            0x81, 0x06,   # Input (00000110)
            0xC0,   # End Collection
        ])
