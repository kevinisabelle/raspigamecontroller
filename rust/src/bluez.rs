use crate::constants::{
    ADAPTER_IFACE, ADAPTER_PATH, AGENT_MANAGER_IFACE, BLUEZ_SERVICE, BLUEZ_SERVICE_PATH,
    DBUS_PROPERTIES_IFACE, LE_ADVERTISING_MANAGER_IFACE,
};
use std::collections::HashMap;
use std::error::Error;
use zbus::fdo;
use zbus::zvariant::Value;
use zbus::{interface, zvariant::ObjectPath};
use zbus::{Connection, Proxy};

pub type Properties<'a> = HashMap<String, zbus::zvariant::Value<'a>>;

pub trait DBusProperties {
    fn get_all(&self, interface: &str) -> Properties;

    fn properties_changed(
        &self,
        _changed: HashMap<String, Value>,
        _invalidated: Vec<String>,
    ) -> fdo::Result<()> {
        Ok(())
    }
}

#[derive(Default)]
pub struct Agent {
    pub path: String,
}

impl Agent {
    pub fn new(path: String) -> Self {
        Self { path }
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

#[derive(Default)]
pub struct Advertisement {
    pub path: String,
    ad_type: String,
    service_uuids: Option<Vec<String>>,
    manufacturer_data: Option<HashMap<u16, Vec<u8>>>,
    solicit_uuids: Option<Vec<String>>,
    service_data: Option<HashMap<String, Vec<u8>>>,
    local_name: Option<String>,
    include_tx_power: bool,
    data: Option<HashMap<u8, Vec<u8>>>,
    appearance: Option<u16>,
}

impl Advertisement {
    pub fn new(path: &str, advertising_type: String) -> Self {
        Self {
            path: path.to_string(),
            ad_type: advertising_type,
            service_uuids: None,
            manufacturer_data: None,
            solicit_uuids: None,
            service_data: None,
            local_name: None,
            include_tx_power: false,
            data: None,
            appearance: None,
        }
    }

    pub fn get_path(&self) -> ObjectPath {
        ObjectPath::try_from(self.path.clone()).unwrap()
    }

    pub fn set_appearance(&mut self, appearance: u16) {
        self.appearance = Some(appearance);
    }

    pub fn set_include_tx_power(&mut self, include: bool) {
        self.include_tx_power = include;
    }

    pub fn add_service_uuid(&mut self, uuid: String) {
        if self.service_uuids.is_none() {
            self.service_uuids = Some(Vec::new());
        }
        self.service_uuids.as_mut().unwrap().push(uuid);
    }

    pub fn add_solicit_uuid(&mut self, uuid: String) {
        if self.solicit_uuids.is_none() {
            self.solicit_uuids = Some(Vec::new());
        }
        self.solicit_uuids.as_mut().unwrap().push(uuid);
    }

    pub fn add_manufacturer_data(&mut self, manuf_code: u16, data: Vec<u8>) {
        if self.manufacturer_data.is_none() {
            self.manufacturer_data = Some(HashMap::new());
        }
        self.manufacturer_data
            .as_mut()
            .unwrap()
            .insert(manuf_code, data);
    }

    pub fn add_service_data(&mut self, uuid: String, data: Vec<u8>) {
        if self.service_data.is_none() {
            self.service_data = Some(HashMap::new());
        }
        self.service_data.as_mut().unwrap().insert(uuid, data);
    }

    pub fn add_local_name(&mut self, name: String) {
        self.local_name = Some(name);
    }

    pub fn add_data(&mut self, ad_type: u8, data: Vec<u8>) {
        if self.data.is_none() {
            self.data = Some(HashMap::new());
        }
        self.data.as_mut().unwrap().insert(ad_type, data);
    }
}

#[interface(name = "org.bluez.LEAdvertisement1")]
impl Advertisement {
    pub async fn release(&self) -> fdo::Result<()> {
        println!("{}: Released!", self.path);
        Ok(())
    }

    #[zbus(property, name = "Type")]
    pub fn get_type(&self) -> &str {
        &self.ad_type
    }

    #[zbus(property, name = "ServiceUUIDs")]
    pub fn get_service_uuids(&self) -> Vec<String> {
        self.service_uuids.clone().unwrap_or_default()
    }

    #[zbus(property, name = "ManufacturerData")]
    pub fn get_manufacturer_data(&self) -> HashMap<u16, Vec<u8>> {
        self.manufacturer_data.clone().unwrap_or_default()
    }

    #[zbus(property, name = "SolicitUUIDs")]
    pub fn get_solicit_uuids(&self) -> Vec<String> {
        self.solicit_uuids.clone().unwrap_or_default()
    }

    #[zbus(property, name = "ServiceData")]
    pub fn get_service_data(&self) -> HashMap<String, Vec<u8>> {
        self.service_data.clone().unwrap_or_default()
    }

    #[zbus(property, name = "LocalName")]
    pub fn get_local_name(&self) -> String {
        self.local_name.clone().unwrap_or_default()
    }

    #[zbus(property, name = "Includes")]
    pub fn get_includes(&self) -> Vec<String> {
        if self.include_tx_power {
            vec!["tx-power".to_string()]
        } else {
            Vec::new()
        }
    }
}

/// Registers an advertisement by calling the RegisterAdvertisement method on the
/// LEAdvertisementManager1 interface.
///
/// # Arguments
///
/// * `connection` - An established zbus Connection.
/// * `advertisement_path` - A string slice holding the advertisement's object path.
pub async fn register_advertisement(
    connection: &Connection,
    advertisement_path: String,
) -> Result<(), zbus::Error> {
    // Obtain the unique name of the BlueZ service.

    // Create a proxy to the adapter's LEAdvertisementManager1 interface using the unique destination.
    let ad_manager: Proxy = Proxy::new(
        connection,
        BLUEZ_SERVICE,
        ADAPTER_PATH,
        LE_ADVERTISING_MANAGER_IFACE,
    )
    .await?;

    // Create an empty dictionary for the options.
    let options: HashMap<String, zbus::zvariant::Value> = HashMap::new();

    // Call the RegisterAdvertisement method.
    ad_manager
        .call_method(
            "RegisterAdvertisement",
            &(
                zbus::zvariant::ObjectPath::try_from(advertisement_path)?,
                options,
            ),
        )
        .await?;

    Ok(())
}

// A type alias for simplicity

// GattCharacteristic trait defines common methods with defaults.
pub trait GattCharacteristic {
    // Default read_value returns a not supported error.
    fn read_value(&self, _options: HashMap<String, String>) -> fdo::Result<Vec<u8>> {
        Err(fdo::Error::Failed("Not supported".into()))
    }

    fn write_value(&self, _value: Vec<u8>, _options: HashMap<String, String>) -> fdo::Result<()> {
        Err(fdo::Error::Failed("Not supported".into()))
    }

    fn start_notify(&self) -> fdo::Result<()> {
        Err(fdo::Error::Failed("Not supported".into()))
    }

    fn stop_notify(&self) -> fdo::Result<()> {
        Err(fdo::Error::Failed("Not supported".into()))
    }
}

pub struct BasicCharacteristic {
    pub path: String,
    pub uuid: String,
    pub flags: Vec<String>,
    // For simplicity, service represented by its path.
    pub service: String,
    pub descriptors: Vec<String>,
}

impl BasicCharacteristic {
    pub fn new(path: String, uuid: String, flags: Vec<String>, service: String) -> Self {
        Self {
            path,
            uuid,
            flags,
            service,
            descriptors: Vec::new(),
        }
    }

    pub fn add_descriptor(&mut self, descriptor_path: String) {
        self.descriptors.push(descriptor_path);
    }
}
