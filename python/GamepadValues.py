class GamepadValues1:
    def __init__(self, Btn10=0, Btn11=0, Btn12=0, Btn13=0, Btn14=0, Btn15=0, Btn16=0, Btn17=0, Slider0=0, AxisX0=0, AxisY0=0, AxisZ0=0, AxisRx0=0, AxisRy0=0, AxisRz0=0, AxisVx0=0):
        self.Btn10 = Btn10 & 0x1
        self.Btn11 = Btn11 & 0x1
        self.Btn12 = Btn12 & 0x1
        self.Btn13 = Btn13 & 0x1
        self.Btn14 = Btn14 & 0x1
        self.Btn15 = Btn15 & 0x1
        self.Btn16 = Btn16 & 0x1
        self.Btn17 = Btn17 & 0x1
        self.Slider0 = Slider0 & 0xFF
        self.AxisX0 = AxisX0 & 0xFF
        self.AxisY0 = AxisY0 & 0xFF
        self.AxisZ0 = AxisZ0 & 0xFF
        self.AxisRx0 = AxisRx0 & 0xFF
        self.AxisRy0 = AxisRy0 & 0xFF
        self.AxisRz0 = AxisRz0 & 0xFF
        self.AxisVx0 = AxisVx0 & 0xFF

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

    def set_AxisX0(self, value):
        self.AxisX0 = value & 0xFF

    def set_AxisY0(self, value):
        self.AxisY0 = value & 0xFF

    def set_AxisZ0(self, value):
        self.AxisZ0 = value & 0xFF

    def set_AxisRx0(self, value):
        self.AxisRx0 = value & 0xFF

    def set_AxisRy0(self, value):
        self.AxisRy0 = value & 0xFF

    def set_AxisRz0(self, value):
        self.AxisRz0 = value & 0xFF

    def set_AxisVx0(self, value):
        self.AxisVx0 = value & 0xFF

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
        total = (total << 8) | self.AxisX0
        total = (total << 8) | self.AxisY0
        total = (total << 8) | self.AxisZ0
        total = (total << 8) | self.AxisRx0
        total = (total << 8) | self.AxisRy0
        total = (total << 8) | self.AxisRz0
        total = (total << 8) | self.AxisVx0
        result = total.to_bytes(9, byteorder='big')
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
            0x35, 0x00,   # Physical Minimum (0)
            0x45, 0xFF,   # Physical Maximum (255)
            0x75, 0x08,   # Report Size (8)
            0x95, 0x01,   # Report Count (1)
            0x81, 0x02,   # Input (00000010)
            0x05, 0x01,   # Usage Page (Generic Desktop Controls)
            0x09, 0x30,   # Usage (X)
            0x19, 0x01,   # Usage Minimum (1)
            0x29, 0x01,   # Usage Maximum (1)
            0x15, 0x81,   # Logical Minimum (-127)
            0x25, 0x7F,   # Logical Maximum (127)
            0x35, 0x81,   # Physical Minimum (-127)
            0x45, 0x7F,   # Physical Maximum (127)
            0x75, 0x08,   # Report Size (8)
            0x95, 0x01,   # Report Count (1)
            0x81, 0x02,   # Input (00000010)
            0x05, 0x01,   # Usage Page (Generic Desktop Controls)
            0x09, 0x31,   # Usage (Y)
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
            0x09, 0x32,   # Usage (Z)
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
            0x09, 0x33,   # Usage (Rx)
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
            0x09, 0x34,   # Usage (Ry)
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
            0x09, 0x35,   # Usage (Rz)
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
            0x09, 0x30,   # Usage (X)
            0x19, 0x01,   # Usage Minimum (1)
            0x29, 0x01,   # Usage Maximum (1)
            0x15, 0x00,   # Logical Minimum (0)
            0x25, 0xFF,   # Logical Maximum (255)
            0x35, 0x00,   # Physical Minimum (0)
            0x45, 0xFF,   # Physical Maximum (255)
            0x75, 0x08,   # Report Size (8)
            0x95, 0x01,   # Report Count (1)
            0x81, 0x02,   # Input (00000010)
            0xC0,   # End Collection
        ])
