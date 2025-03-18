use std::error::Error;
use zbus::{fdo, interface, Connection, Proxy};
use zbus::zvariant::{ObjectPath, Value};
use crate::constants::{ADAPTER_IFACE, ADAPTER_PATH, AGENT_MANAGER_IFACE, BLUEZ_SERVICE, BLUEZ_SERVICE_PATH, DBUS_PROPERTIES_IFACE};
use crate::utils::ObjectPathTrait;

#[derive(Default, Debug)]
pub struct Agent {
    pub path: String,
}

impl Agent {
    pub fn new(path: String) -> Self {
        Self { path }
    }
}

impl ObjectPathTrait for Agent {
    fn object_path(&self) -> String {
        self.path.clone()
    }
}

#[interface(name = "org.bluez.Agent1")]
impl Agent {
    // The Release method: no parameters, no return.
    async fn release(&self) -> fdo::Result<()> {
        println!("Agent Released");
        Ok(())
    }

    // The RequestPasskey method receives a device object path and returns an u32.
    async fn request_passkey(&self, device: ObjectPath<'_>) -> fdo::Result<u32> {
        println!("RequestPasskey for device: {}", device);
        Ok(0)
    }

    // The DisplayPasskey method receives a device path and a passkey.
    async fn display_passkey(&self, device: ObjectPath<'_>, passkey: u32) -> fdo::Result<()> {
        println!(
            "DisplayPasskey for device: {}, passkey: {}",
            device, passkey
        );
        Ok(())
    }

    // The RequestConfirmation method receives a device path and passkey.
    async fn request_confirmation(&self, device: ObjectPath<'_>, passkey: u32) -> fdo::Result<()> {
        println!(
            "Auto-confirming pairing for device: {} with passkey: {}",
            device, passkey
        );
        Ok(())
    }

    // The RequestPinCode method receives a device path and returns a string.
    async fn request_pin_code(&self, device: ObjectPath<'_>) -> fdo::Result<String> {
        println!("RequestPinCode for device: {}", device);
        Ok("0000".into())
    }

    // The RequestAuthorization method receives a device path.
    async fn request_authorization(&self, device: ObjectPath<'_>) -> fdo::Result<()> {
        println!("RequestAuthorization for device: {}", device);
        Ok(())
    }

    // The AuthorizeService method receives a device path and a service UUID.
    async fn authorize_service(&self, device: ObjectPath<'_>, uuid: &str) -> fdo::Result<()> {
        println!(
            "AuthorizeService called for device: {} and service UUID: {}",
            device, uuid
        );
        Ok(())
    }
}

pub async fn register_agent(
    connection: &Connection,
    agent_object_path: &str,
    capability: &str,
) -> Result<(), zbus::Error> {
    // Create a proxy for the AgentManager interface on /org/bluez.
    let agent_manager = Proxy::new(
        &connection,
        BLUEZ_SERVICE,
        BLUEZ_SERVICE_PATH,
        AGENT_MANAGER_IFACE,
    )
    .await?;

    println!("Agent manager proxy created");

    let result_registering = agent_manager
        .call_method(
            "RegisterAgent",
            &(ObjectPath::try_from(agent_object_path)?, capability),
        )
        .await?;

    println!("Agent registered: {:?}", result_registering);

    // Call RequestDefaultAgent(agent_object_path)
    agent_manager
        .call_method(
            "RequestDefaultAgent",
            &(ObjectPath::try_from(agent_object_path)?),
        )
        .await?;
    println!("Agent registered as default with {} capability", capability);

    // Attempt to set adapter properties using the Properties interface.
    match Proxy::new(
        &connection,
        BLUEZ_SERVICE,
        ADAPTER_PATH,
        DBUS_PROPERTIES_IFACE,
    )
    .await
    {
        Ok(adapter_proxy) => {
            async fn set_property(
                proxy: &Proxy<'_>,
                property: &str,
                value: bool,
            ) -> Result<(), Box<dyn Error>> {
                proxy
                    .call_method("Set", &(ADAPTER_IFACE, property, Value::from(value)))
                    .await?;
                Ok(())
            }
            if let Err(e) = set_property(&adapter_proxy, "Powered", true).await {
                eprintln!("Error setting Powered: {}", e);
            }
            if let Err(e) = set_property(&adapter_proxy, "Discoverable", true).await {
                eprintln!("Error setting Discoverable: {}", e);
            }
            if let Err(e) = set_property(&adapter_proxy, "Pairable", true).await {
                eprintln!("Error setting Pairable: {}", e);
            }
        }
        Err(e) => {
            eprintln!("Error obtaining adapter proxy: {}", e);
        }
    }

    Ok(())
}
