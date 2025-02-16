using Tmds.DBus;

namespace GamePadBTDbus.Bluetooth;

public class GattDescriptor : IGattDescriptor1
{
    public ObjectPath ObjectPath { get; }
    private string _uuid;
    private byte[] _value;
    private ObjectPath _characteristic;
    public GattDescriptor(ObjectPath path, ObjectPath characteristic, string uuid, byte[] value)
    {
        ObjectPath = path;
        _characteristic = characteristic;
        _uuid = uuid;
        _value = value;
    }
    public string UUID => _uuid;
    public ObjectPath Characteristic => _characteristic;
    public Task<byte[]> ReadValueAsync(IDictionary<string, object> options)
    {
        return Task.FromResult(_value);
    }
}