use crate::constants::{ADAPTER_PATH, BLUEZ_SERVICE, LE_ADVERTISING_MANAGER_IFACE};
use crate::utils::ObjectPathTrait;
use std::collections::HashMap;
use zbus::{fdo, interface, Connection, Proxy};

#[derive(Default, Debug)]
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
    pub fn new(
        path: String,
        ad_type: String,
        service_uuids: Option<Vec<String>>,
        manufacturer_data: Option<HashMap<u16, Vec<u8>>>,
        solicit_uuids: Option<Vec<String>>,
        service_data: Option<HashMap<String, Vec<u8>>>,
        local_name: Option<String>,
        include_tx_power: bool,
        data: Option<HashMap<u8, Vec<u8>>>,
        appearance: Option<u16>,
    ) -> Self {
        Self {
            path: path.to_string(),
            ad_type,
            service_uuids,
            manufacturer_data,
            solicit_uuids,
            service_data,
            local_name,
            include_tx_power,
            data,
            appearance,
        }
    }
}

impl ObjectPathTrait for Advertisement {
    fn object_path(&self) -> String {
        self.path.clone()
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
        println!("advertisment get_type: {:?}", self.ad_type);
        &self.ad_type
    }

    #[zbus(property, name = "ServiceUUIDs")]
    pub fn get_service_uuids(&self) -> Vec<String> {
        println!(
            "advertisment get_service_uuids: {:?}",
            self.service_uuids.clone().unwrap_or_default()
        );
        self.service_uuids.clone().unwrap_or_default()
    }

    /*#[zbus(property, name = "ManufacturerData")]
    pub fn get_manufacturer_data(&self) -> HashMap<u16, Vec<u8>> {
        println!(
            "advertisment get_manufacturer_data: {:?}",
            self.manufacturer_data.clone().unwrap_or_default()
        );
        self.manufacturer_data.clone().unwrap_or_default()
    }*/

    /*#[zbus(property, name = "SolicitUUIDs")]
    pub fn get_solicit_uuids(&self) -> Vec<String> {
        println!(
            "advertisment get_solicit_uuids: {:?}",
            self.solicit_uuids.clone().unwrap_or_default()
        );
        self.solicit_uuids.clone().unwrap_or_default()
    }*/

    /*#[zbus(property, name = "ServiceData")]
    pub fn get_service_data(&self) -> HashMap<String, Vec<u8>> {
        println!(
            "advertisment get_service_data: {:?}",
            self.service_data.clone().unwrap_or_default()
        );
        self.service_data.clone().unwrap_or_default()
    }*/

    #[zbus(property, name = "LocalName")]
    pub fn get_local_name(&self) -> String {
        println!(
            "advertisment get_local_name: {:?}",
            self.local_name.clone().unwrap_or_default()
        );
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

    /*#[zbus(property, name = "Data")]
    pub fn get_data(&self) -> HashMap<u8, Vec<u8>> {
        println!(
            "advertisment get_data: {:?}",
            self.data.clone().unwrap_or_default()
        );
        self.data.clone().unwrap_or_default()
    }*/

    #[zbus(property, name = "Appearance")]
    pub fn get_appearance(&self) -> u16 {
        println!(
            "advertisment get_appearance: {:#X}",
            self.appearance.clone().unwrap_or_default()
        );
        self.appearance.unwrap_or_default()
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
