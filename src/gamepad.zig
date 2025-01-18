const std = @import("std");

// We'll use pigpio via C interop.
const c = @cImport({
    @cInclude("pigpio.h");
});

pub fn main() !void {

    // Output a message to the console.
    std.debug.print("Hello, gamepad!\n", .{});

    // Initialize pigpio (returns >= 0 if successful).
    std.debug.print("Initializing pigpio...\n", .{});
    if (c.gpioInitialise() < 0) {
        std.debug.print("Failed to initialize pigpio\n", .{});
        return;
    }
    defer c.gpioTerminate();
    std.debug.print("pigpio initialized\n", .{});

    // Use GPIO 17 as input, with internal pull-up.
    const buttonPin = 17;
    _ = c.gpioSetMode(buttonPin, c.PI_INPUT);
    _ = c.gpioSetPullUpDown(buttonPin, c.PI_PUD_UP);

    // Open the HID device file.
    const hidFile: std.fs.File = std.fs.cwd().openFile("/dev/hidg0", .{}) catch |err| {
        std.debug.print("An error occurred while opening file: {}\n", .{err});
        return;
    };

    defer hidFile.close();

    // We'll repeatedly write 1 byte: 0x01 or 0x00.
    var buffer: [1]u8 = .{0};

    while (true) {
        // If the physical button is pressed (connects pin to GND),
        // gpioRead() returns 0. Otherwise, it's 1.
        const isPressed = c.gpioRead(buttonPin);
        if (isPressed == 0) {
            // pressed
            buffer[0] = 0x01;
        } else {
            // not pressed
            buffer[0] = 0x00;
        }

        // Write the single-byte report to the HID device.
        try hidFile.writeAll(&buffer);

        // Sleep a bit to avoid hammering the CPU constantly.
        std.time.sleep(50 * std.time.ms_per_s);
    }
}
