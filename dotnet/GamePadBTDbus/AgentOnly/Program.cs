using Tmds.DBus;

namespace AgentOnly;
// -------------------------------
// Define the D-Bus interfaces
// -------------------------------
    
[DBusInterface("org.bluez.Agent1")]
public interface IAgent1 : IDBusObject
{
    // Called when the agent is unregistered.
    Task ReleaseAsync();

    // Called to request a numeric passkey for pairing.
    // (Returns a uint between 0 and 999999.)
    Task<uint> RequestPasskeyAsync(ObjectPath device);

    // Called to display a passkey; expects a uint passkey.
    Task DisplayPasskeyAsync(ObjectPath device, uint passkey);

    // Called to request confirmation of a passkey.
    // (Here, we expect the passkey to be passed as a string, matching the Python "os" signature.)
    Task RequestConfirmationAsync(ObjectPath device, uint passkey);

    // Called to request a PIN code; returns a string PIN.
    Task<string> RequestPinCodeAsync(ObjectPath device);

    // Called to request authorization for pairing.
    Task RequestAuthorizationAsync(ObjectPath device);

    // Called to authorize a service request; takes a device and a service UUID.
    Task AuthorizeServiceAsync(ObjectPath device, string uuid);
    
    Task CancelAsync();
}

// For agent registration
[DBusInterface("org.bluez.AgentManager1")]
public interface IAgentManager1 : IDBusObject
{
    Task RegisterAgentAsync(ObjectPath agentPath, string capability);
    Task RequestDefaultAgentAsync(ObjectPath agentPath);
}

[DBusInterface("org.bluez.Adapter1")]
public interface IAdapter1 : IDBusObject
{
    Task SetAsync(string propertyName, object value);
}

// -------------------------------
// Implement the service classes
// -------------------------------

// Agent for pairing
[DBusInterface("org.bluez.Agent1")]
public class Agent1 : IAgent1
{
    public ObjectPath ObjectPath { get; }
    public Agent1(ObjectPath path) { ObjectPath = path; }

    public Task ReleaseAsync()
    {
        Console.WriteLine("Agent Released");
        return Task.CompletedTask;
    }

    public Task<string> RequestPinCodeAsync(ObjectPath device)
    {
        Console.WriteLine("Requesting PIN code for device: " + device);
        return Task.FromResult("0000");
    }

    public Task DisplayPinCodeAsync(ObjectPath device, string pincode)
    {
        Console.WriteLine("Displaying PIN code for device: " + device + " PIN: " + pincode);
        return Task.CompletedTask;
    }

    public Task<uint> RequestPasskeyAsync(ObjectPath device)
    {
        Console.WriteLine("Requesting passkey for device: " + device);
        return Task.FromResult((uint)123456);
    }

    public Task DisplayPasskeyAsync(ObjectPath device, uint passkey)
    {
        Console.WriteLine("Displaying passkey for device: " + device + " Passkey: " + passkey);
        return Task.CompletedTask;
    }

    public Task RequestConfirmationAsync(ObjectPath device, uint passkey)
    {
        Console.WriteLine("Requesting confirmation for device: " + device + " Passkey: " + passkey);
        return Task.CompletedTask;
    }

    public Task RequestAuthorizationAsync(ObjectPath device)
    {
        Console.WriteLine("Requesting authorization for device: " + device);
        return Task.CompletedTask;
    }

    public Task AuthorizeServiceAsync(ObjectPath device, string uuid)
    {
        Console.WriteLine("Authorizing service for device: " + device + " UUID: " + uuid);
        return Task.CompletedTask;
    }

    public Task CancelAsync()
    {
        Console.WriteLine("Agent request cancelled");
        return Task.CompletedTask;
    }
}

// -------------------------------
// Main program and registration helpers
// -------------------------------
public class Program
{
    static async Task Main(string[] args)
    {
        Console.WriteLine("Starting BlueZ Agent...");
        var connection = new Connection(Address.System);
        await connection.ConnectAsync();

        var agent = new Agent1(new ObjectPath("/com/kevinisabelle/agent"));
        var capability = "KeyboardDisplay";

        await connection.RegisterObjectAsync(agent);

        if (args.Length > 0 && args[0] == "register")
        {
            var agentManager = connection.CreateProxy<IAgentManager1>("org.bluez", new ObjectPath("/org/bluez"));
                
            await agentManager.RegisterAgentAsync(agent.ObjectPath, capability);
            Console.WriteLine("Agent registered with capability: " + capability);
                
            await Task.Delay(1000);
                
            await agentManager.RequestDefaultAgentAsync(agent.ObjectPath);
            Console.WriteLine($"Agent registered as default with {capability} capability");
        }

        try
        {
            var adapterProps = connection.CreateProxy<IAdapter1>("org.bluez", new ObjectPath("/org/bluez/hci0"));
                
            await adapterProps.SetAsync("Powered", true);
            await adapterProps.SetAsync("Discoverable", true);
            await adapterProps.SetAsync("Pairable", true);
                
            Console.WriteLine("Adapter set to Powered, Discoverable, and Pairable");
        }
        catch (Exception ex)
        {
            Console.WriteLine("Error setting adapter properties: " + ex.Message);
            throw;
        }
            
        // Run forever
        await Task.Delay(-1);
    }
}