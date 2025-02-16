using HashtagChris.DotNetBlueZ;

IAdapter1 adapter = (await BlueZManager.GetAdaptersAsync()).FirstOrDefault();

Console.WriteLine("Adapter Name: " + adapter.ObjectPath);