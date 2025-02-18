namespace HidReportMapCreator.Definition;

public enum InputType
{
    /// <summary>
    /// A button or switch
    /// </summary>
    Button,
    
    /// <summary>
    /// A joystick control with 2 axes
    /// </summary>
    Joystick,
    
    /// <summary>
    /// A linear control that can be moved up or down
    /// </summary>
    Slider,
    
    /// <summary>
    /// A rotary control for generating a variable value, normally in the form of a knob
    /// spun by the index finger and thumb. Report values should increase as controls are
    /// spun clockwise. This usage does not follow the HID orientation conventions.
    /// </summary>
    Dial,
    
    /// <summary>
    /// A rotary control for generating a variable value, normally rolled, unlike a dial.
    /// Report values should increase as controls are rolled forward, away from the user.
    /// This usage does not follow the HID orientation conventions.
    /// </summary>
    Wheel,
    
    /// <summary>
    /// A hat switch. A hat switch is a switch that can be in one of several positions, such as up, down, left, right, or centered.
    /// </summary>
    HatSwitch,
}