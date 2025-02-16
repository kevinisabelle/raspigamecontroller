using Tmds.DBus;

namespace GamePadBTDbus.Bluetooth;

public class GattService : IGattService1
{
    public ObjectPath ObjectPath { get; }
    private string _uuid = "00001812-0000-1000-8000-00805f9b34fb";
    private bool _primary = true;
    private List<ObjectPath> _characteristics;
    public GattService(ObjectPath path, List<ObjectPath> characteristics, string serviceUUID)
    {
        ObjectPath = path;
        _characteristics = characteristics;
        _uuid = serviceUUID;
    }
    public string UUID => _uuid;
    public bool Primary => _primary;
    public ObjectPath[] Characteristics => _characteristics.ToArray();
}