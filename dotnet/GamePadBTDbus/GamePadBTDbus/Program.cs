using GamePadBTDbus.Bluetooth;
// using Tmds.DBus;
using Tmds.DBus.Protocol;

namespace GamePadBTDbus;

public static class Handlers
{
    public static byte[] ReportMapReadHandler()
    {
        return
        [
            0x05, 0x01,       // Usage Page (Generic Desktop)
            0x09, 0x05,       // Usage (Game Pad)
            0xA1, 0x01,       // Collection (Application)
            0x85, 0x01,       // Report ID (1)
            0x05, 0x09,       // Usage Page (Button)
            0x19, 0x01,       // Usage Minimum (Button 1)
            0x29, 0x01,       // Usage Maximum (Button 1)
            0x15, 0x00,       // Logical Minimum (0)
            0x25, 0x01,       // Logical Maximum (1)
            0x75, 0x01,       // Report Size (1)
            0x95, 0x01,       // Report Count (1)
            0x81, 0x02,       // Input (Data, Variable, Absolute)
            0x75, 0x01,       // Report Size (1)
            0x95, 0x07,       // Report Count (7)
            0x81, 0x03,       // Input (Constant)
            0x05, 0x01,       // Usage Page (Generic Desktop)
            0x09, 0x30,       // Usage (X)
            0x15, 0x81,       // Logical Minimum (-127)
            0x25, 0x7F,       // Logical Maximum (127)
            0x75, 0x08,       // Report Size (8)
            0x95, 0x01,       // Report Count (1)
            0x81, 0x02,       // Input (Data, Variable, Absolute)
            0x85, 0x02,       // Report ID (2)
            0x09, 0x48,       // Usage (LED)
            0x15, 0x00,       // Logical Minimum (0)
            0x25, 0x01,       // Logical Maximum (1)
            0x75, 0x01,       // Report Size (1)
            0x95, 0x01,       // Report Count (1)
            0x91, 0x02,       // Output (Data, Variable, Absolute)
            0x75, 0x07,       // Report Size (7)
            0x95, 0x01,       // Report Count (1)
            0x91, 0x03,       // Output (Constant)
            0xC0              // End Collection
        ];
    }

    public static byte[] GamepadReportReadHandler() => [0x00, 0x00];

    public static byte[] HidInfoReadHandler() => [0x11, 0x01, 0x00, 0x03];

    public static byte[] ProtocolModeReadHandler() => [0x01];
}

public class Program
{
    static async Task Main(string[] args)
    {
        Console.WriteLine("Starting GamepadKi...");
            Tmds.DBus.Protocol.
        var connection = new Connection(Address.System);
        await connection.ConnectAsync();

        var characteristics = new List<ObjectPath>();
        var hidService = new GattService(Constants.SERVICE_PATH, characteristics, Constants.GATT_SERVICE_HID_UUID);
        await connection.RegisterObjectAsync(hidService);

        // Register characteristics:
        var reportMapChar = new GattCharacteristic(
            $"{Constants.SERVICE_PATH}/char0",
            Constants.GATT_REPORT_MAP_UUID,
            ["read"],
            Constants.SERVICE_PATH,
            Handlers.ReportMapReadHandler
        );
        await connection.RegisterObjectAsync(reportMapChar);

        var gamepadReportChar = new GattCharacteristic(
            $"{Constants.SERVICE_PATH}/char1",
            Constants.GATT_REPORT_UUID,
            ["read", "notify", "write-without-response"],
            Constants.SERVICE_PATH,
            Handlers.GamepadReportReadHandler
        );
        await connection.RegisterObjectAsync(gamepadReportChar);

        var hidInfoChar = new GattCharacteristic(
            $"{Constants.SERVICE_PATH}/char2",
            Constants.GATT_HID_INFORMATION_UUID,
            [ "read" ],
            Constants.SERVICE_PATH,
            Handlers.HidInfoReadHandler
        );
        await connection.RegisterObjectAsync(hidInfoChar);

        var protocolModeChar = new GattCharacteristic(
            $"{Constants.SERVICE_PATH}/char3",
            Constants.GATT_PROTOCOL_MODE_UUID,
            new string[] { "read", "write", "write-without-response" },
            Constants.SERVICE_PATH,
            Handlers.ProtocolModeReadHandler
        );
        await connection.RegisterObjectAsync(protocolModeChar);

        // Add descriptors to gamepad report characteristic
        var reportRefDesc = new GattDescriptor(
            $"{Constants.SERVICE_PATH}/char1/desc0",
            $"{Constants.SERVICE_PATH}/char1",
            Constants.GATT_DESC_REPORT_REFERENCE_UUID,
            [0x01, 0x01]
        );
        await connection.RegisterObjectAsync(reportRefDesc);

        var cccDesc = new GattDescriptor(
            $"{Constants.SERVICE_PATH}/char1/desc1",
            $"{Constants.SERVICE_PATH}/char1",
            Constants.GATT_DESC_CLIENT_DESCRIPTOR_UUID,
            [0x00, 0x01]
        );
        await connection.RegisterObjectAsync(cccDesc);

        var outputReportRef = new GattDescriptor(
            "{Constants.SERVICE_PATH}/char1/desc2",
            $"{Constants.SERVICE_PATH}/char1",
            Constants.GATT_DESC_REPORT_REFERENCE_UUID,
            [0x00, 0x02]
        );
        await connection.RegisterObjectAsync(outputReportRef);

        // Add descriptor object paths to gamepad report characteristic
        gamepadReportChar.AddDescriptor($"{Constants.SERVICE_PATH}/char1/desc0");
        gamepadReportChar.AddDescriptor($"{Constants.SERVICE_PATH}/char1/desc1");
        gamepadReportChar.AddDescriptor($"{Constants.SERVICE_PATH}/char1/desc2");

        // Add characteristic object paths to HID service
        characteristics.Add(reportMapChar.ObjectPath);
        characteristics.Add(gamepadReportChar.ObjectPath);
        characteristics.Add(hidInfoChar.ObjectPath);
        characteristics.Add(protocolModeChar.ObjectPath);

        Console.WriteLine("Manual GATT server running with:");
        Console.WriteLine($"  - Report Map Characteristic at {Constants.SERVICE_PATH}/char0");
        Console.WriteLine($"  - Gamepad Report Characteristic at {Constants.SERVICE_PATH}/char1");
        Console.WriteLine($"  - HID Information Characteristic at {Constants.SERVICE_PATH}/char2");
        Console.WriteLine($"  - Protocol Mode Characteristic at {Constants.SERVICE_PATH}/char3");

        // Create and register BLE Advertisement
        var advertisement = new LEAdvertisement(Constants.ADVERTISEMENT_PATH, "peripheral")
        {
            ServiceUUIDs = [Constants.GATT_SERVICE_HID_UUID],
            IncludeTxPower = true
        };
        await connection.RegisterObjectAsync(advertisement);
        await RegisterAdvertisement(connection, advertisement);

        // Run forever
        await Task.Delay(-1);
    }

    static async Task RegisterAdvertisement(Connection connection, LEAdvertisement leAdvertisement)
    {
        try
        {
            var adManager = connection.CreateProxy<ILEAdvertisingManager1>(Constants.BLUEZ_SERVICE, Constants.ADAPTER_PATH);
            await adManager.RegisterAdvertisementAsync(leAdvertisement.ObjectPath, new Dictionary<string, object>());
            Console.WriteLine("Advertisement registered");
        }
        catch (Exception ex)
        {
            Console.WriteLine("Failed to register advertisement: " + ex.Message);
            throw;
        }
    }
}