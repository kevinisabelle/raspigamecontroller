using Windows.Gaming.Input;

class Program
{
    static void Main(string[] args)
    {
        // Subscribe to events
        Gamepad.GamepadAdded += Gamepad_GamepadAdded;
        Gamepad.GamepadRemoved += Gamepad_GamepadRemoved;

        // Check initial list (may be empty if the controller is added later)
        Console.WriteLine("Initial connected gamepads: " + Gamepad.Gamepads.Count);

        if (Gamepad.Gamepads.Count > 0)
        {
            Console.WriteLine("LeftThmbx: " + Gamepad.Gamepads[0].GetCurrentReading().LeftThumbstickX);
        }
        
        Console.WriteLine("Waiting for gamepad events. Press any key to exit...");
        Console.ReadKey();
    }

    private static void Gamepad_GamepadAdded(object sender, Gamepad e)
    {
        Console.WriteLine("Gamepad added: " + e);
        Console.WriteLine("LeftThmbx: " + e.GetCurrentReading().LeftThumbstickX);
        Console.WriteLine("Current connected gamepads: " + Gamepad.Gamepads.Count);
    }

    private static void Gamepad_GamepadRemoved(object sender, Gamepad e)
    {
        Console.WriteLine("Gamepad removed: " + e);
        Console.WriteLine("Current connected gamepads: " + Gamepad.Gamepads.Count);
    }
}