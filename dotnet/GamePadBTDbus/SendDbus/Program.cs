using Tmds.DBus;

[DBusInterface("com.kevinisabelle.Hello")]
public interface IHello : IDBusObject
{
    Task<string> SayHelloAsync(ObjectPath name, uint passkey);
    
    Task SayHelloNoReplyAsync(ObjectPath name, uint passkey);
}

public class HelloService : IHello
{
    // Set the object path where this service is available.
    public ObjectPath ObjectPath { get; } = new ObjectPath("/com/kevinisabelle/Hello");

    public Task<string> SayHelloAsync(ObjectPath name, uint passkey)
    {
        Console.WriteLine("Hello called with name: " + name + " Passkey: " + passkey);
        return Task.FromResult($"Hello, {name}! with passkey {passkey}");
    }
    
    public Task SayHelloNoReplyAsync(ObjectPath name, uint passkey)
    {
        Console.WriteLine("Hello called with name: " + name + " Passkey: " + passkey);
        return Task.CompletedTask;
    }
}

public class Program
{
    public static async Task Main(string[] args)
    {
        var isServer = args.Length > 0 && args[0] == "server";
        var isNoreply = args.Length > 1 && args[1] == "noreply";

        if (isNoreply)
        {
            await  MakeClientNoReply();
            return;
        }
        
        if (isServer)
        {
            await MakeServer();
        }
        else
        {
            await MakeClient();
        }
    }

    private static async Task MakeServer()
    {
        // Connect to the session bus.
        var connection = new Connection(Address.System);
        await connection.ConnectAsync();

        // Create and register the service object.
        var helloService = new HelloService();
        await connection.RegisterObjectAsync(helloService);

        // var agent = new TestAgent();
        // await connection.RegisterObjectAsync(agent);
        
        // Request a well-known bus name.
        var serviceName = "org.example.HelloService";
        await connection.RegisterServiceAsync(serviceName);

        Console.WriteLine("Service is running. Press Ctrl+C to exit.");
        await Task.Delay(-1); // Keep the service alive.
        
    }
    
    private static async Task MakeClient()
    {
        // Connect to the session bus.
        var connection = new Connection(Address.System);
        await connection.ConnectAsync();

        // Create a proxy to the service object.
        var hello = connection.CreateProxy<IHello>("org.example.HelloService", new ObjectPath("/com/kevinisabelle/Hello"));

        // Call the remote method.
        var response = await hello.SayHelloAsync(new ObjectPath("/org/bluez/hci0/dev_B0_A4_60_E7_88_52"), 123456);
        Console.WriteLine("Say Hello called: " + response);
    }
    
    private static async Task MakeClientNoReply()
    {
        // Connect to the session bus.
        var connection = new Connection(Address.System);
        await connection.ConnectAsync();

        // Create a proxy to the service object.
        var hello = connection.CreateProxy<IHello>("org.example.HelloService", new ObjectPath("/com/kevinisabelle/Hello"));

        // Call the remote method.
        await hello.SayHelloNoReplyAsync(new ObjectPath("/org/bluez/hci0/dev_B0_A4_60_E7_88_52"), 123456);
        Console.WriteLine("Say Hello No Reply called");
    }
}