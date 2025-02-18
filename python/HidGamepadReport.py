from enum import Enum, auto
from dataclasses import dataclass, field
from typing import List

class HIDControlType(Enum):
    BUTTON = auto()
    AXIS = auto()
    HAT_SWITCH = auto()
    SLIDER = auto()
    ROTARY_ENCODER = auto()
    POT = auto()
    JOYSTICK = auto()

@dataclass
class GamepadControl:
    """
    Represents a single control element using human-readable parameters.
    """
    type: HIDControlType  # The type of control (e.g., BUTTON, AXIS, etc.)
    name: str             # A human-friendly name for the control
    count: int            # Number of items (e.g., number of buttons or axes)
    logical_min: int      # Minimum value (e.g., 0 for buttons, -127 for axes)
    logical_max: int      # Maximum value (e.g., 1 for buttons, 127 for axes)
    value: int = 0x00     # Current value of the control (for multi-button, bits are used)

    def set_value(self, value: int):
        """
        Sets the value of the control.
        For a multi-button control, if index is provided, it sets the bit at that index.
        For single-value controls, index is ignored.
        """
        if self.type == HIDControlType.BUTTON:
           self.value = value

    def get_report_bytes(self) -> List[int]:
        """
        Returns the report bytes for this control.
        This is a simplified example:
          - BUTTON: Pack up to 8 buttons into one byte.
          - AXIS, SLIDER, POT, ROTARY_ENCODER: Return a single byte (assuming value fits).
          - HAT_SWITCH: Return a nibble (packed in one byte).
          - JOYSTICK_WITH_BUTTON: Assume two axes and one button; here we return three bytes.
        """
        if self.type == HIDControlType.BUTTON:
            # For up to 8 buttons, return one byte.
            return [self.value & 0xFF]
        elif self.type in (HIDControlType.AXIS, HIDControlType.SLIDER, HIDControlType.POT, HIDControlType.ROTARY_ENCODER):
            # For these, we assume a single value fits in one byte.
        
            return [self.value & 0xFF, self.value & 0xFF, self.value & 0xFF, self.value & 0xFF]
        elif self.type == HIDControlType.HAT_SWITCH:
            # Hat switch: use the lower 4 bits (0-15).
            return [self.value & 0xFF]
        elif self.type == HIDControlType.JOYSTICK:
            # For a joystick with button, assume two axes and one button.
            # In a real scenario, you'd probably store each axis separately.
            # For demonstration, we return two dummy axis bytes and one button byte.
            axis1 = (self.value >> 24) & 0xFF
            axis2 = (self.value >> 16) & 0xFF
            axis3 = (self.value >> 8) & 0xFF
            axis4 = (self.value >> 0) & 0xFF
            return [axis1, axis2, axis3, axis4]
        else:
            return [0]

    def get_report_map_bytes(self) -> List[int]:
        """
        Returns the report map bytes for this control.
        This is a placeholder.
        A full implementation would generate a proper HID report descriptor snippet
        based on the control type, count, and logical range.
        """

        if self.type == HIDControlType.BUTTON:
            padding = 8 - self.count  # calculate how many bits to pad to reach a full byte
            return [
                0x05, 0x09,             # Usage Page (Button)
                0x19, 0x01,             # Usage Minimum (Button 1)
                0x29, 0x00 + self.count, # Usage Maximum (Button self.count)
                0x15, 0x00,             # Logical Minimum (0)
                0x25, 0x01,             # Logical Maximum (1)
                0x75, 0x01,             # Report Size (1 bit per button)
                0x95, self.count,       # Report Count (actual number of buttons)
                0x81, 0x02,             # Input (Data, Variable, Absolute) for button bits
                0x75, 0x01,             # Report Size (1 bit for padding)
                0x95, padding,          # Report Count (padding bits to fill one byte)
                0x81, 0x03              # Input (Constant) for padding
            ]
        elif self.type == HIDControlType.SLIDER:
            # Example: For a single axis or slider.
            return [
                0x05, 0x01,             # Usage Page (Generic Desktop)
                0x09, 0x36,             # Usage (Slider)
                0x15, self.logical_min & 0xFF, # Logical Minimum
                0x25, self.logical_max & 0xFF, # Logical Maximum
                0x75, 0x08,             # Report Size (4 bits)
                0x95, self.count,       # Report Count (number of axes)
                0x81, 0x02              # Input (Data, Variable, Absolute)
            ]
        elif self.type == HIDControlType.POT:
            # Example: For a single potentiometer.
            return [
                0x05, 0x01,             # Usage Page (Generic Desktop)
                0x09, 0x37,             # Usage (Dial)
                0x15, self.logical_min & 0xFF, # Logical Minimum
                0x25, self.logical_max & 0xFF, # Logical Maximum
                0x75, 0x08,             # Report Size (8 bits)
                0x95, self.count,       # Report Count (number of pots)
                0x81, 0x02              # Input (Data, Variable, Absolute)
            ]
        elif self.type == HIDControlType.ROTARY_ENCODER:
            # Example: Same as axis but intended as relative.
            return [
                0x05, 0x01,             # Usage Page (Generic Desktop)
                0x09, 0x38,             # Usage (Wheel)
                0x15, self.logical_min & 0xFF, # Logical Minimum
                0x25, self.logical_max & 0xFF, # Logical Maximum
                0x75, 0x08,             # Report Size (8 bits)
                0x95, self.count,       # Report Count (number of encoders)
                0x81, 0x06              # Input (Data, Variable, Relative)
            ]
        elif self.type == HIDControlType.HAT_SWITCH:
            # Example: 4-bit hat switch.
            return [
                0x05, 0x01,             # Usage Page (Generic Desktop)
                0x09, 0x39,             # Usage (Hat switch)
                0x15, 0x01,             # Logical Minimum (1)
                0x25, 0x08,             # Logical Maximum (8)
                0x35, 0x00,             # Physical Minimum (0)
                0x46, 0x3B, 0x01,       # Physical Maximum (315)
                0x65, 0x14,             # Unit (degrees)
                0x75, 0x04,             # Report Size (4 bits)
                0x95, 0x01,             # Report Count (1)
                0x81, 0x02              # Input (Data, Variable, Absolute)
            ]
        
        elif self.type == HIDControlType.JOYSTICK:
            # Example: Two axes and one button.
            return [
                0x05, 0x01, # Usage Page (Generic Desktop)
                0x09, 0x04, # Usage (Joystick)
                0xA1, 0x01, # Collection (Application)
                0x09, 0x30, # Usage (X)
                0x09, 0x31, # Usage (Y)
                0x09, 0x33, # Usage (Rx)
                0x09, 0x34, # Usage (Ry)
                0x15, 0x81, # Logical Min (–127)
                0x25, 0x7F, # Logical Max (127)
                0x75, 0x08, # Report Size (8 bits)
                0x95, 0x04, # Report Count (4 axes)
                0x81, 0x02, # Input (Data, Variable, Absolute)
                0xC0        # End Collection
            ]
                    
        else:
            return []

@dataclass
class GamepadDefinition:
    """
    Holds the complete gamepad configuration.
    """
    name: str                           # Name of the gamepad
    controls: List[GamepadControl] = field(default_factory=list)

    def get_report_map_bytes(self) -> List[int]:
        """
        Returns the complete HID report descriptor bytes for all controls.
        This includes the header, all control descriptors, and the ending collection.
        """
        report_map = []
        # Header: Usage Page (Generic Desktop), Usage (Gamepad), Collection (Application)
        report_map += [0x05, 0x01,    # Usage Page (Generic Desktop)
                       0x09, 0x05,    # Usage (Gamepad)
                       0xA1, 0x01]    # Collection (Application)
        
        report_map += [0x85, 0x01]    # Report ID 1

        # Append each control's report map bytes
        for control in self.controls:
            report_map += control.get_report_map_bytes()

        # End Collection
        report_map += [0xC0]
        return report_map
    
    def get_report_bytes(self) -> List[int]:
        """
        Returns the report bytes for all controls
        """
        report = []
        for control in self.controls:
            report += control.get_report_bytes()
        return report