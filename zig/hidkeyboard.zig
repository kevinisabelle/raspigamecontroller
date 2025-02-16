const c = @cImport({
    @cInclude("sys/socket.h");
    @cInclude("bluetooth/bluetooth.h");
    @cInclude("bluetooth/l2cap.h");
});

const std = @import("std");
pub fn main() !void {

    // Open a connection to the system D-Bus
    const dbus = try std.fs.cwd().openFile("/run/dbus/system_bus_socket", .{ .mode = .read_write });
    defer dbus.close();

    var buf: [1024]u8 = undefined;
    // Create the HID profile XML for D-Bus registration by reading it from a file
    const xml = try std.fs.cwd().readFile("hidprofile.xml", &buf);

    // Write the XML to register the HID profile via D-Bus
    try dbus.writeAll(xml);

    std.debug.print("HID Profile registered. Waiting for connections...\n", .{});

    const sock = c.socket(c.AF_BLUETOOTH, c.SOCK_SEQPACKET, 0);
    if (sock < 0) {
        return error.SocketError;
    }
    // defer c.close(sock);

    // Define the target address and port
    var addr: c.struct_sockaddr = .{};
    addr.sa_family = c.AF_BLUETOOTH;
    var bdaddrvar: c.bdaddr_t = .{};
    _ = c.str2ba("01:23:45:67:89:AB", &bdaddrvar); // Replace with the host device's Bluetooth MAC
    // addr.sa_data = 0x11; // Protocol/Service Multiplexer for HID

    // const converted_addr = c.struct_sockaddr{ .l2_family = c.AF_BLUETOOTH, .l2_bdaddr = addr.l2_bdaddr, .l2_psm = addr.l2_psm };

    // Connect to the remote Bluetooth device
    if (c.connect(sock, &addr, @sizeOf(c.struct_sockaddr_l2)) < 0) {
        return error.ConnectionError;
    }
}
