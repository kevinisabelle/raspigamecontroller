using System.Runtime.InteropServices;

namespace HciExample.Bluetooth;

public class HclLayer
{
    // Import the socket(), ioctl(), bind(), setsockopt(), and close() functions from libc.
    [DllImport("libc", SetLastError = true)]
    public static extern int socket(int domain, int type, int protocol);

    [DllImport("libc", SetLastError = true)]
    public static extern int ioctl(int fd, uint request, ref hci_dev_info info);

    [DllImport("libc", SetLastError = true)]
    public static extern int close(int fd);

    [DllImport("libc", SetLastError = true)]
    public static extern int bind(int sockfd, ref sockaddr_hci addr, uint addrlen);

    [DllImport("libc", SetLastError = true)]
    public static extern int setsockopt(int socket, int level, int option_name, ref hci_filter option_value,
        uint option_len);

    // bdaddr_t: Represents a 6-byte Bluetooth device address.
    [StructLayout(LayoutKind.Sequential)]
    public struct bdaddr_t
    {
        [MarshalAs(UnmanagedType.ByValArray, SizeConst = 6)]
        public byte[] b;
    }

    // hci_dev_info: Holds basic info about an HCI device.
    // (This is a simplified version. The real structure has more fields.)
    [StructLayout(LayoutKind.Sequential, CharSet = CharSet.Ansi)]
    public struct hci_dev_info
    {
        public ushort dev_id;

        [MarshalAs(UnmanagedType.ByValTStr, SizeConst = 8)]
        public string name;

        public bdaddr_t bdaddr;
        public uint flags;
        public byte type;

        [MarshalAs(UnmanagedType.ByValArray, SizeConst = 8)]
        public byte[] features;

        public uint pkt_type;
        public uint link_policy;
        public uint link_mode;
        public ushort acl_len;
        public byte sco_len;

        [MarshalAs(UnmanagedType.ByValArray, SizeConst = 10)]
        public uint[] stat;
    }

    // sockaddr_hci: Address structure for binding to an HCI device.
    [StructLayout(LayoutKind.Sequential)]
    public struct sockaddr_hci
    {
        public ushort hci_family; // Should be AF_BLUETOOTH
        public ushort hci_dev; // HCI device index (e.g. 0 for hci0)
        public ushort hci_channel; // HCI channel (RAW or USER)
    }

    // hci_filter: Used with setsockopt() to filter HCI events.
    [StructLayout(LayoutKind.Sequential)]
    public struct hci_filter
    {
        public uint type_mask;

        [MarshalAs(UnmanagedType.ByValArray, SizeConst = 2)]
        public uint[] event_mask;

        public ushort opcode;
    }

    // Opens a raw Bluetooth HCI socket.
    private static int GetSocket()
    {
        int sock = socket(HciConstants.AF_BLUETOOTH, HciConstants.SOCK_RAW, HciConstants.BTPROTO_HCI);
        if (sock < 0)
        {
            Console.WriteLine("Failed to open socket. Are you running as root?");
            throw new Exception("Failed to open socket. Are you running as root?");
        }

        return sock;
    }

    // Wrapper for ioctl call.
    private static void IoCtl(int sock, uint request, ref hci_dev_info info)
    {
        int ret = ioctl(sock, request, ref info);
        if (ret < 0)
        {
            Console.WriteLine($"ioctl failed. Request: {request:X8}");
            throw new Exception($"ioctl failed. Request: {request:X8}");
        }
    }

    // Retrieves the HCI device information for hci0.
    public static hci_dev_info GetDeviceInfo()
    {
        hci_dev_info devInfo = new hci_dev_info();

        int sock = GetSocket();
        try
        {
            // Prepare a hci_dev_info structure for hci0 (dev_id = 0)
            devInfo.dev_id = 0; // hci0
            devInfo.features = new byte[8];
            devInfo.stat = new uint[10];
            devInfo.bdaddr = new bdaddr_t { b = new byte[6] };

            // Call ioctl to fill the structure with adapter information.
            IoCtl(sock, HciConstants.HCIGETDEVINFO, ref devInfo);
        }
        catch (Exception ex)
        {
            Console.WriteLine(ex.Message);
            throw;
        }
        finally
        {
            int result = close(sock);
            if (result < 0)
            {
                Console.WriteLine("Failed to close socket.");
            }
        }

        return devInfo;
    }

    // Create and bind a socket to a specified HCI device index and channel.
    private static int CreateSocket(ushort index, ushort channel)
    {
        int fd = socket(HciConstants.AF_BLUETOOTH, HciConstants.SOCK_RAW, HciConstants.BTPROTO_HCI);
        if (fd < 0)
        {
            Console.WriteLine("Failed to create socket.");
            return -1;
        }

        sockaddr_hci addr = new sockaddr_hci
        {
            hci_family = (ushort)HciConstants.AF_BLUETOOTH,
            hci_dev = index,
            hci_channel = channel
        };

        int ret = bind(fd, ref addr, (uint)Marshal.SizeOf(typeof(sockaddr_hci)));
        if (ret < 0)
        {
            Console.WriteLine("Bind failed.");
            close(fd);
            return -1;
        }

        return fd;
    }

    // Creates a socket bound to the HCI user channel.
    public static int NewUserChannel(ushort index)
    {
        int fd = CreateSocket(index, HciConstants.HCI_CHANNEL_USER);
        if (fd < 0)
        {
            Console.WriteLine("Unable to create user channel socket.");
            throw new Exception("Unable to create user channel socket.");
        }

        return fd;
    }

    // Creates a socket bound to the HCI raw channel and sets an HCI filter.
    public static int NewRawDevice(ushort index)
    {
        int fd = CreateSocket(index, HciConstants.HCI_CHANNEL_RAW);
        if (fd < 0)
        {
            Console.WriteLine("Unable to create raw channel socket.");
            throw new Exception("Unable to create raw channel socket.");
        }

        // Set up an HCI filter
        hci_filter filter = new hci_filter
        {
            // Set type_mask to only receive HCI event packets.
            type_mask = 1U << HciConstants.BT_H4_EVT_PKT,
            event_mask = [0xFFFFFFFF, 0xFFFFFFFF],
            opcode = 0
        };

        int ret = setsockopt(fd, HciConstants.SOL_HCI, HciConstants.HCI_FILTER, ref filter,
            (uint)Marshal.SizeOf(typeof(hci_filter)));
        if (ret < 0)
        {
            Console.WriteLine("setsockopt failed.");
            close(fd);
            throw new Exception("setsockopt failed.");
        }

        return fd;
    }
}
