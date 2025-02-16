import sys
import asyncio
from dbus_next.aio import MessageBus
from dbus_next.service import ServiceInterface, method
from dbus_next.constants import BusType

class HelloService(ServiceInterface):
    def __init__(self):
        super().__init__('com.kevinisabelle.Hello')

    @method()
    def SayHello(self, name: 'o', passkey: 'u') -> 's': # type: ignore
        print("Hello called with name:", name, "Passkey:", passkey)
        return f"Hello, {name}! with passkey {passkey}"
    
    @method()
    def SayHelloNoReply(self, name: 'o', passkey: 'u') -> None: # type: ignore
        print("Hello called with name:", name, "Passkey:", passkey)

async def run_server():
    bus = await MessageBus(bus_type=BusType.SYSTEM).connect()
    hello_service = HelloService()
    bus.export('/com/kevinisabelle/Hello', hello_service)
    await bus.request_name('org.example.HelloService')
    print("Service is running. Press Ctrl+C to exit.")
    await asyncio.get_running_loop().create_future()  # Run forever

async def run_client():
    bus = await MessageBus(bus_type=BusType.SYSTEM).connect()
    introspection = await bus.introspect('org.example.HelloService', '/com/kevinisabelle/Hello')
    proxy_object = bus.get_proxy_object('org.example.HelloService', '/com/kevinisabelle/Hello', introspection)
    hello_interface = proxy_object.get_interface('com.kevinisabelle.Hello')
    response = await hello_interface.call_say_hello("/org/bluez/hci0/dev_B0_A4_60_E7_88_52", 123456)
    print("Say Hello called:", response)

async def run_client_no_reply():
    bus = await MessageBus(bus_type=BusType.SYSTEM).connect()
    introspection = await bus.introspect('org.example.HelloService', '/com/kevinisabelle/Hello')
    proxy_object = bus.get_proxy_object('org.example.HelloService', '/com/kevinisabelle/Hello', introspection)
    hello_interface = proxy_object.get_interface('com.kevinisabelle.Hello')
    await hello_interface.call_say_hello_no_reply("/org/bluez/hci0/dev_B0_A4_60_E7_88_52", 123456)
    print("Say Hello No Reply called")

if __name__ == '__main__':
    if len(sys.argv) > 1 and sys.argv[1] == "server":
        asyncio.run(run_server())
    elif len(sys.argv) > 1 and sys.argv[1] == "noreply":
        asyncio.run(run_client_no_reply())
    else:
        asyncio.run(run_client())
