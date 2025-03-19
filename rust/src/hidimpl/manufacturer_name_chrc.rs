use crate::utils::ObjectPathTrait;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use macros::gatt_chrc_properties;
use zbus::interface;
use crate::bluez::base_gatt_chrc::BaseGattCharacteristic;

#[derive(Debug)]
pub struct ManufacturerNameChrc {
    pub base: BaseGattCharacteristic,
    pub value: Vec<u8>,
}

impl ObjectPathTrait for ManufacturerNameChrc {
    fn object_path(&self) -> String {
        self.base.path.to_string()
    }
}

impl ManufacturerNameChrc {
    pub fn new(path: String, service: String) -> Self {
        let uuid = "2a29".to_string();
        let flags = vec!["read".to_string()];
        Self {
            base: BaseGattCharacteristic::new(path.clone(), uuid, flags, service, vec![]),
            value: "Ki".as_bytes().to_vec(),
        }
    }
}

pub(crate) struct ManufacturerNameChrcInterface(pub Arc<Mutex<ManufacturerNameChrc>>);

#[gatt_chrc_properties()]
#[interface(name = "org.bluez.GattCharacteristic1")]
impl ManufacturerNameChrcInterface {
    fn read_value(&self, _options: HashMap<String, String>) -> zbus::fdo::Result<Vec<u8>> {
        Ok(self.0.lock().unwrap().value.clone())
    }
}
