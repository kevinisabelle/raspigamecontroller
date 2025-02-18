class GamepadValues1:
    def __init__(self, Joystick0, Joystick1, Joystick2, Joystick3, Slider0, Slider1, Slider2, Slider3, Rotary0, Rotary1, Rotary2, Rotary3, Pot0, Pot1, Pot2, Pot3, Button0, Button1, Button2, Button3, Button4, Button5):
        self.Joystick0 = Joystick0 & 0xFF
        self.Joystick1 = Joystick1 & 0xFF
        self.Joystick2 = Joystick2 & 0xFF
        self.Joystick3 = Joystick3 & 0xFF
        self.Slider0 = Slider0 & 0xFF
        self.Slider1 = Slider1 & 0xFF
        self.Slider2 = Slider2 & 0xFF
        self.Slider3 = Slider3 & 0xFF
        self.Rotary0 = Rotary0 & 0xFF
        self.Rotary1 = Rotary1 & 0xFF
        self.Rotary2 = Rotary2 & 0xFF
        self.Rotary3 = Rotary3 & 0xFF
        self.Pot0 = Pot0 & 0xFF
        self.Pot1 = Pot1 & 0xFF
        self.Pot2 = Pot2 & 0xFF
        self.Pot3 = Pot3 & 0xFF
        self.Button0 = Button0 & 0x1
        self.Button1 = Button1 & 0x1
        self.Button2 = Button2 & 0x1
        self.Button3 = Button3 & 0x1
        self.Button4 = Button4 & 0x1
        self.Button5 = Button5 & 0x1
        self.padding1 = 0  # Dedicated padding field

    def set_Joystick0(self, value):
        self.Joystick0 = value & 0xFF

    def set_Joystick1(self, value):
        self.Joystick1 = value & 0xFF

    def set_Joystick2(self, value):
        self.Joystick2 = value & 0xFF

    def set_Joystick3(self, value):
        self.Joystick3 = value & 0xFF

    def set_Slider0(self, value):
        self.Slider0 = value & 0xFF

    def set_Slider1(self, value):
        self.Slider1 = value & 0xFF

    def set_Slider2(self, value):
        self.Slider2 = value & 0xFF

    def set_Slider3(self, value):
        self.Slider3 = value & 0xFF

    def set_Rotary0(self, value):
        self.Rotary0 = value & 0xFF

    def set_Rotary1(self, value):
        self.Rotary1 = value & 0xFF

    def set_Rotary2(self, value):
        self.Rotary2 = value & 0xFF

    def set_Rotary3(self, value):
        self.Rotary3 = value & 0xFF

    def set_Pot0(self, value):
        self.Pot0 = value & 0xFF

    def set_Pot1(self, value):
        self.Pot1 = value & 0xFF

    def set_Pot2(self, value):
        self.Pot2 = value & 0xFF

    def set_Pot3(self, value):
        self.Pot3 = value & 0xFF

    def set_Button0(self, value):
        self.Button0 = value & 0x1

    def set_Button1(self, value):
        self.Button1 = value & 0x1

    def set_Button2(self, value):
        self.Button2 = value & 0x1

    def set_Button3(self, value):
        self.Button3 = value & 0x1

    def set_Button4(self, value):
        self.Button4 = value & 0x1

    def set_Button5(self, value):
        self.Button5 = value & 0x1

    def get_report(self):
        total = 0
        total = (total << 8) | self.Joystick0
        total = (total << 8) | self.Joystick1
        total = (total << 8) | self.Joystick2
        total = (total << 8) | self.Joystick3
        total = (total << 8) | self.Slider0
        total = (total << 8) | self.Slider1
        total = (total << 8) | self.Slider2
        total = (total << 8) | self.Slider3
        total = (total << 8) | self.Rotary0
        total = (total << 8) | self.Rotary1
        total = (total << 8) | self.Rotary2
        total = (total << 8) | self.Rotary3
        total = (total << 8) | self.Pot0
        total = (total << 8) | self.Pot1
        total = (total << 8) | self.Pot2
        total = (total << 8) | self.Pot3
        total = (total << 1) | self.Button0
        total = (total << 1) | self.Button1
        total = (total << 1) | self.Button2
        total = (total << 1) | self.Button3
        total = (total << 1) | self.Button4
        total = (total << 1) | self.Button5
        total = total << 0  # Padding field padding1
        return total.to_bytes(17, byteorder='big')

    def get_report_map():
        return bytes([
            0x05, 0x01,   # Usage Page (Generic Desktop Controls)
            0x09, 0x05,   # Usage (Gamepad)
            0xA1, 0x01,   # Collection (Application)
            0x85, 0x01,   # Report ID 1
            0x05, 0x01,   # Usage Page (Generic Desktop Controls)
            0x09, 0x30,   # Usage (X)
            0x09, 0x31,   # Usage (Y)
            0x09, 0x33,   # Usage (Rx)
            0x09, 0x34,   # Usage (Ry)
            0x19, 0x01,   # Usage Minimum (1)
            0x29, 0x04,   # Usage Maximum (4)
            0x15, 0x81,   # Logical Minimum (-127)
            0x25, 0x7F,   # Logical Maximum (127)
            0x75, 0x08,   # Report Size (8)
            0x95, 0x04,   # Report Count (4)
            0x81, 0x02,   # Input (Variable, Absolute, No Wrap, Linear, Preferred State, No Null Position, Bit Field)
            0x05, 0x01,   # Usage Page (Generic Desktop Controls)
            0x09, 0x36,   # Usage (Slider)
            0x19, 0x01,   # Usage Minimum (1)
            0x29, 0x04,   # Usage Maximum (4)
            0x15, 0x00,   # Logical Minimum (0)
            0x25, 0xFF,   # Logical Maximum (255)
            0x75, 0x08,   # Report Size (8)
            0x95, 0x04,   # Report Count (4)
            0x81, 0x02,   # Input (Variable, Absolute, No Wrap, Linear, Preferred State, No Null Position, Bit Field)
            0x05, 0x01,   # Usage Page (Generic Desktop Controls)
            0x09, 0x38,   # Usage (Wheel)
            0x19, 0x01,   # Usage Minimum (1)
            0x29, 0x04,   # Usage Maximum (4)
            0x15, 0x81,   # Logical Minimum (-127)
            0x25, 0x7F,   # Logical Maximum (127)
            0x75, 0x08,   # Report Size (8)
            0x95, 0x04,   # Report Count (4)
            0x81, 0x02,   # Input (Variable, Absolute, No Wrap, Linear, Preferred State, No Null Position, Bit Field)
            0x05, 0x01,   # Usage Page (Generic Desktop Controls)
            0x09, 0x37,   # Usage (Dial)
            0x19, 0x01,   # Usage Minimum (1)
            0x29, 0x04,   # Usage Maximum (4)
            0x15, 0x00,   # Logical Minimum (0)
            0x25, 0xFF,   # Logical Maximum (255)
            0x75, 0x08,   # Report Size (8)
            0x95, 0x04,   # Report Count (4)
            0x81, 0x02,   # Input (Variable, Absolute, No Wrap, Linear, Preferred State, No Null Position, Bit Field)
            0x05, 0x09,   # Usage Page (Button)
            0x19, 0x01,   # Usage Minimum (1)
            0x29, 0x06,   # Usage Maximum (6)
            0x15, 0x00,   # Logical Minimum (0)
            0x25, 0x01,   # Logical Maximum (1)
            0x75, 0x01,   # Report Size (1)
            0x95, 0x06,   # Report Count (6)
            0x81, 0x02,   # Input (Variable, Absolute, No Wrap, Linear, Preferred State, No Null Position, Bit Field)
            0x75, 0x02,   # Padding (2)
            0x95, 0x01,   # Padding Count (1)
            0x81, 0x00,   # Padding (Constant, Array, Absolute)
            0xC0,   # End Collection
        ])