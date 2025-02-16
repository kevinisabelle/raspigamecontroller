const std = @import("std");

pub fn main() !void {
    const device = "/dev/hidraw0";

    // Open Bluetooth HID socket (example assumes HID channel is 19)
    const report: [8]u8 = [_]u8{ 0x00, 0x00, 0x04, 0x00, 0x00, 0x00, 0x00, 0x00 }; // 'A' key pressed

    // const dir = try std.fs.cwd().openDir("/dev", .{});
    const file = try std.fs.cwd().openFile(device, .{ .mode = .write_only });
    const writer = std.fs.File.writer(file);
    _ = try writer.write(&report);
    defer file.close();
}
