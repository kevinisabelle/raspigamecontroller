use crate::bluez::base_gatt_chrc::BaseGattCharacteristic;
use crate::object_path;
use crate::utils::ObjectPathTrait;
use macros::{gatt_characteristic};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use zbus::interface;

#[derive(Debug)]
pub struct ManufacturerNameChrc {
    pub base: BaseGattCharacteristic,
    pub value: Vec<u8>,
}

object_path! {
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
}

pub(crate) struct ManufacturerNameChrcInterface(pub Arc<Mutex<ManufacturerNameChrc>>);

#[gatt_characteristic()]
impl ManufacturerNameChrcInterface {
    fn read_value(&self, _options: HashMap<String, String>) -> zbus::fdo::Result<Vec<u8>> {
        Ok(self.0.lock().unwrap().value.clone())
    }
}
