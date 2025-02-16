using System;
using System.Threading.Tasks;
using InTheHand.Bluetooth;
using InTheHand.Bluetooth.GenericAttributeProfile;
using InTheHand.Net;
using InTheHand.Net.Sockets;

namespace ConsoleBleClient
{
    class Program
    {
        static async Task Main(string[] args)
        {
            // Your device's MAC address (colon-separated)
            string macAddress = "B8:27:EB:F1:93:3D";
            // Convert to a UInt64. InTheHand expects a numeric address.
            ulong address = ConvertMacToULong(macAddress);

            var client = new BluetoothClient();
            
            client.ConnectAsync(new BluetoothAddress(address), Service
            
            // Connect to the BLE device
            BluetoothDevice device = await BluetoothDevice.FromBluetoothAddressAsync(address);
            if (device == null)in
            {
                Console.WriteLine("Device not found.");
                return;
            }
            Console.WriteLine("Connected to device.");

            // Get GATT services
            var services = device.Gatt;
            foreach (var service in services)
            {
                Console.WriteLine($"Service: {service.Uuid}");
                var characteristics = await service.GetCharacteristicsAsync();
                foreach (var characteristic in characteristics)
                {
                    Console.WriteLine($"\tCharacteristic: {characteristic.Uuid} Properties: {characteristic.CharacteristicProperties}");

                    // Read value if possible
                    if (characteristic.CharacteristicProperties.HasFlag(GattCharacteristicProperties.Read))
                    {
                        var result = await characteristic.ReadValueAsync();
                        if (result.Status == GattCommunicationStatus.Success)
                        {
                            Console.WriteLine($"\t\tValue: {BitConverter.ToString(result.Value)}");
                        }
                        else
                        {
                            Console.WriteLine("\t\tRead failed.");
                        }
                    }
                }
            }
            Console.WriteLine("Done. Press any key to exit...");
            Console.ReadKey();
        }

        static ulong ConvertMacToULong(string mac)
        {
            // Remove colons and parse as hexadecimal.
            string hex = mac.Replace(":", "");
            return ulong.Parse(hex, System.Globalization.NumberStyles.HexNumber);
        }
    }
}
