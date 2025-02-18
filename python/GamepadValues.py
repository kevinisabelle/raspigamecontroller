class GamepadValues1:
    def __init__(self, Btn10=0, Btn11=0, Btn12=0, Btn13=0, Btn14=0, Btn15=0, Btn16=0, Btn17=0, Slider0=0, Slider20=0, Slider30=0):
        self.Btn10 = Btn10 & 0x1
        self.Btn11 = Btn11 & 0x1
        self.Btn12 = Btn12 & 0x1
        self.Btn13 = Btn13 & 0x1
        self.Btn14 = Btn14 & 0x1
        self.Btn15 = Btn15 & 0x1
        self.Btn16 = Btn16 & 0x1
        self.Btn17 = Btn17 & 0x1
        self.Slider0 = Slider0 & 0xFF
        self.Slider20 = Slider20 & 0xFF
        self.Slider30 = Slider30 & 0xFF

    def set_Btn10(self, value):
        self.Btn10 = value & 0x1

    def set_Btn11(self, value):
        self.Btn11 = value & 0x1

    def set_Btn12(self, value):
        self.Btn12 = value & 0x1

    def set_Btn13(self, value):
        self.Btn13 = value & 0x1

    def set_Btn14(self, value):
        self.Btn14 = value & 0x1

    def set_Btn15(self, value):
        self.Btn15 = value & 0x1

    def set_Btn16(self, value):
        self.Btn16 = value & 0x1

    def set_Btn17(self, value):
        self.Btn17 = value & 0x1

    def set_Slider0(self, value):
        self.Slider0 = value & 0xFF

    def set_Slider20(self, value):
        self.Slider20 = value & 0xFF

    def set_Slider30(self, value):
        self.Slider30 = value & 0xFF

    def get_report(self):
        total = 0
        total = (total << 1) | self.Btn10
        total = (total << 1) | self.Btn11
        total = (total << 1) | self.Btn12
        total = (total << 1) | self.Btn13
        total = (total << 1) | self.Btn14
        total = (total << 1) | self.Btn15
        total = (total << 1) | self.Btn16
        total = (total << 1) | self.Btn17
        total = (total << 8) | self.Slider0
        total = (total << 8) | self.Slider20
        total = (total << 8) | self.Slider30
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
            0x29, 0x08,   # Usage Maximum (8)
            0x15, 0x00,   # Logical Minimum (0)
            0x25, 0x01,   # Logical Maximum (1)
            0x75, 0x01,   # Report Size (1)
            0x95, 0x08,   # Report Count (8)
            0x81, 0x02,   # Input (00000010)
            0x05, 0x01,   # Usage Page (Generic Desktop Controls)
            0x09, 0x36,   # Usage (Slider)
            0x19, 0x01,   # Usage Minimum (1)
            0x29, 0x01,   # Usage Maximum (1)
            0x15, 0x00,   # Logical Minimum (0)
            0x25, 0xFF,   # Logical Maximum (255)
            0x75, 0x08,   # Report Size (8)
            0x95, 0x01,   # Report Count (1)
            0x81, 0x02,   # Input (00000010)
            0x05, 0x01,   # Usage Page (Generic Desktop Controls)
            0x09, 0x36,   # Usage (Slider)
            0x19, 0x01,   # Usage Minimum (1)
            0x29, 0x01,   # Usage Maximum (1)
            0x15, 0x00,   # Logical Minimum (0)
            0x25, 0xFF,   # Logical Maximum (255)
            0x75, 0x08,   # Report Size (8)
            0x95, 0x01,   # Report Count (1)
            0x81, 0x02,   # Input (00000010)
            0x05, 0x01,   # Usage Page (Generic Desktop Controls)
            0x09, 0x36,   # Usage (Slider)
            0x19, 0x01,   # Usage Minimum (1)
            0x29, 0x01,   # Usage Maximum (1)
            0x15, 0x00,   # Logical Minimum (0)
            0x25, 0xFF,   # Logical Maximum (255)
            0x75, 0x08,   # Report Size (8)
            0x95, 0x01,   # Report Count (1)
            0x81, 0x02,   # Input (00000010)
            0xC0,   # End Collection
        ])
