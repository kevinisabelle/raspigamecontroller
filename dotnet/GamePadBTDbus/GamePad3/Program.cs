using HciExample.Bluetooth;

namespace HciExample
{
    class Program
    {
        static void Main(string[] args)
        {
            var deviceInfo = HclLayer.GetDeviceInfo();
            Console.WriteLine($"Device Addr: {deviceInfo.bdaddr.b[0]:X2}:{deviceInfo.bdaddr.b[1]:X2}:{deviceInfo.bdaddr.b[2]:X2}:{deviceInfo.bdaddr.b[3]:X2}:{deviceInfo.bdaddr.b[4]:X2}:{deviceInfo.bdaddr.b[5]:X2}");
            
        }
    }
}
