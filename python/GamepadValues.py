class GamepadValues1:
    def __init__(self, Slider0=0, Slider20=0, Button0=0, Button1=0, Button2=0, Button3=0):
        self.Slider0 = Slider0 & 0xFF
        self.Slider20 = Slider20 & 0xFF
        self.Button0 = Button0 & 0x1
        self.Button1 = Button1 & 0x1
        self.Button2 = Button2 & 0x1
        self.Button3 = Button3 & 0x1
        self.padding1 = 0  # Dedicated padding field

    def set_Slider0(self, value):
        self.Slider0 = value & 0xFF

    def set_Slider20(self, value):
        self.Slider20 = value & 0xFF

    def set_Button0(self, value):
        self.Button0 = value & 0x1

    def set_Button1(self, value):
        self.Button1 = value & 0x1

    def set_Button2(self, value):
        self.Button2 = value & 0x1

    def set_Button3(self, value):
        self.Button3 = value & 0x1

    def get_report(self):
        total = 0
        total = (total << 8) | self.Slider0
        total = (total << 8) | self.Slider20
        total = (total << 1) | self.Button0
        total = (total << 1) | self.Button1
        total = (total << 1) | self.Button2
        total = (total << 1) | self.Button3
        total = total << 4  # Padding field padding1
        return total.to_bytes(3, byteorder='big')
        # return bytes([0x01] + list(result))


    def get_report_map(self):
        return bytes([
            0x05, 0x01,   # Usage Page (Generic Desktop Controls)
            0x09, 0x05,   # Usage (Gamepad)
            0xA1, 0x01,   # Collection (Application)
            # 0x85, 0x01,   # Report ID 1
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
            0x15, 0x81,   # Logical Minimum (-127)
            0x25, 0x7F,   # Logical Maximum (127)
            0x75, 0x08,   # Report Size (8)
            0x95, 0x01,   # Report Count (1)
            0x81, 0x02,   # Input (Variable, Absolute, No Wrap, Linear, Preferred State, No Null Position, Bit Field)
            0x05, 0x09,   # Usage Page (Button)
            0x19, 0x01,   # Usage Minimum (1)
            0x29, 0x04,   # Usage Maximum (4)
            0x15, 0x00,   # Logical Minimum (0)
            0x25, 0x01,   # Logical Maximum (1)
            0x75, 0x01,   # Report Size (1)
            0x95, 0x04,   # Report Count (4)
            0x81, 0x02,   # Input (Variable, Absolute, No Wrap, Linear, Preferred State, No Null Position, Bit Field)
            0x75, 0x04,   # Padding (4)
            0x95, 0x01,   # Padding Count (1)
            0x81, 0x01,   # Padding (Constant)
            0xC0,   # End Collection
        ])