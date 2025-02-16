const c = @cImport({
    @cInclude("bluetooth/bluetooth.h");
    @cInclude("bluetooth/l2cap.h");
});

const std = @import("std");

const e = error{SocketError};

pub fn main() !void {
    const AF_BLUETOOTH = c.AF_BLUETOOTH;
    const SOCK_SEQPACKET = c.SOCK_SEQPACKET;
    const BTPROTO_L2CAP = c.BTPROTO_L2CAP;

    // Open a socket
    const sock = c.socket(AF_BLUETOOTH, SOCK_SEQPACKET, BTPROTO_L2CAP);
    if (sock < 0) {
        std.debug.print("Failed to open socket: {}\n", .{sock});
        return e.SocketError;
    }
    defer _ = c.close(sock);

    std.debug.print("Socket opened successfully: {}\n", .{sock});
}
