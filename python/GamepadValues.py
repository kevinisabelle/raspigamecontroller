class GamepadValues1:
    def __init__(self, Btn10=0, Btn20=0, Slider0=0, Slider20=0):
        self.Btn10 = Btn10 & 0x1
        self.Btn20 = Btn20 & 0x1
        self.Slider0 = Slider0 & 0xFF
        self.Slider20 = Slider20 & 0xFF
        self.padding1 = 0  # Dedicated padding field
        self.padding2 = 0  # Dedicated padding field

    def set_Btn10(self, value):
        self.Btn10 = value & 0x1

    def set_Btn20(self, value):
        self.Btn20 = value & 0x1

    def set_Slider0(self, value):
        self.Slider0 = value & 0xFF

    def set_Slider20(self, value):
        self.Slider20 = value & 0xFF

    def get_report(self):
        total = 0
        total = (total << 1) | self.Btn10
        total = total << 7  # Padding field padding1
        total = (total << 1) | self.Btn20
        total = total << 7  # Padding field padding2
        total = (total << 8) | self.Slider0
        total = (total << 8) | self.Slider20
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
            0x29, 0x01,   # Usage Maximum (1)
            0x15, 0x00,   # Logical Minimum (0)
            0x25, 0x01,   # Logical Maximum (1)
            0x75, 0x01,   # Report Size (1)
            0x95, 0x01,   # Report Count (1)
            0x81, 0x02,   # Input (Variable, Absolute, No Wrap, Linear, Preferred State, No Null Position, Bit Field)
            0x75, 0x07,   # Padding (7)
            0x95, 0x01,   # Padding Count (1)
            0x81, 0x01,   # Padding (Constant)
            0x05, 0x09,   # Usage Page (Button)
            0x19, 0x01,   # Usage Minimum (1)
            0x29, 0x01,   # Usage Maximum (1)
            0x15, 0x00,   # Logical Minimum (0)
            0x25, 0x01,   # Logical Maximum (1)
            0x75, 0x01,   # Report Size (1)
            0x95, 0x01,   # Report Count (1)
            0x81, 0x02,   # Input (Variable, Absolute, No Wrap, Linear, Preferred State, No Null Position, Bit Field)
            0x75, 0x07,   # Padding (7)
            0x95, 0x01,   # Padding Count (1)
            0x81, 0x01,   # Padding (Constant)
            0x05, 0x01,   # Usage Page (Generic Desktop Controls)
            0x09, 0x36,   # Usage (Slider)
            0x19, 0x01,   # Usage Minimum (1)
            0x29, 0x01,   # Usage Maximum (1)
            0x15, 0x00,   # Logical Minimum (0)
            0x25, 0xFF,   # Logical Maximum (255)
            0x75, 0x08,   # Report Size (8)
            0x95, 0x01,   # Report Count (1)
            0x81, 0x02,   # Input (Variable, Absolute, No Wrap, Linear, Preferred State, No Null Position, Bit Field)
            0x05, 0x01,   # Usage Page (Generic Desktop Controls)
            0x09, 0x36,   # Usage (Slider)
            0x19, 0x01,   # Usage Minimum (1)
            0x29, 0x01,   # Usage Maximum (1)
            0x15, 0x00,   # Logical Minimum (0)
            0x25, 0xFF,   # Logical Maximum (255)
            0x75, 0x08,   # Report Size (8)
            0x95, 0x01,   # Report Count (1)
            0x81, 0x02,   # Input (Variable, Absolute, No Wrap, Linear, Preferred State, No Null Position, Bit Field)
            0xC0,   # End Collection
        ])