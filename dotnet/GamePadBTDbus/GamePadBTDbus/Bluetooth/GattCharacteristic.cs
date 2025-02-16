using Tmds.DBus.Protocol;

namespace GamePadBTDbus.Bluetooth;

public class GattCharacteristic : IGattCharacteristic1
{
    public ObjectPath ObjectPath { get; }
    private string _uuid;
    private ObjectPath _service;
    private string[] _flags;
    private List<ObjectPath> _descriptors = new List<ObjectPath>();
    private Func<byte[]> _readHandler;
    private bool _notifying = false;
    private byte[] _value;
    public GattCharacteristic(ObjectPath path, string uuid, string[] flags, ObjectPath service, Func<byte[]> readHandler = null)
    {
        ObjectPath = path;
        _uuid = uuid;
        _flags = flags;
        _service = service;
        _readHandler = readHandler;
    }
    public string UUID => _uuid;
    public ObjectPath Service => _service;
    public string[] Flags => _flags;
    public ObjectPath[] Descriptors => _descriptors.ToArray();

    public Task<byte[]> ReadValueAsync(IDictionary<string, object> options)
    {
        Console.WriteLine("ManualCharacteristic ReadValue called");
        if (_readHandler != null)
        {
            return Task.FromResult(_readHandler());
        }
        Console.WriteLine("Default ReadValue called");
        return Task.FromResult(new byte[] { 0 });
    }

    public Task WriteValueAsync(byte[] value, IDictionary<string, object> options)
    {
        try
        {
            Console.WriteLine("WriteValue called with value: " + BitConverter.ToString(value));
            _value = value;
            if (value.Length > 0)
            {
                var reportId = value[0];
                if (reportId == 0x02)
                {
                    var ledState = value[1] & 0x01;
                    Console.WriteLine($"Output report received: LED state = {ledState}");
                }
            }
        }
        catch (Exception ex)
        {
            Console.WriteLine("Error in WriteValue: " + ex);
            throw;
        }
        return Task.CompletedTask;
    }

    public Task StartNotifyAsync()
    {
        Console.WriteLine("StartNotify called for " + ObjectPath);
        if (!_notifying)
        {
            _notifying = true;
            if (_readHandler != null)
            {
                _value = _readHandler();
                // In a full implementation, you would emit the PropertiesChanged signal.
            }
        }
        return Task.CompletedTask;
    }

    public Task StopNotifyAsync()
    {
        Console.WriteLine("StopNotify called for " + ObjectPath);
        _notifying = false;
        return Task.CompletedTask;
    }

    public void SendNotification(byte[] value)
    {
        if (_notifying)
        {
            _value = value;
            // Emit PropertiesChanged signal here.
        }
    }

    public void AddDescriptor(ObjectPath descriptor)
    {
        _descriptors.Add(descriptor);
    }
}