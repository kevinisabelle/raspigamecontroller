use crate::bluez::base_gatt_chrc::BaseGattCharacteristic;
use crate::constants::SERIAL_NUMBER_CHARACTERISTIC_UUID;
use crate::utils::ObjectPathTrait;
use macros::gatt_chrc_properties;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use zbus::interface;

#[derive(Debug)]
pub struct SerialNumberChrc {
    pub base: BaseGattCharacteristic,
    pub value: Vec<u8>,
}

impl ObjectPathTrait for SerialNumberChrc {
    fn object_path(&self) -> String {
        self.base.path.to_string()
    }
}

impl SerialNumberChrc {
    pub fn new(path: String, service: String) -> Self {
        let uuid = SERIAL_NUMBER_CHARACTERISTIC_UUID.to_string();
        let flags = vec!["read".to_string()];
        Self {
            base: BaseGattCharacteristic::new(path, uuid, flags, service, vec![]),
            value: "1.0".as_bytes().to_vec(),
        }
    }
}

pub(crate) struct SerialNumberChrcInterface(pub Arc<Mutex<SerialNumberChrc>>);

#[gatt_chrc_properties()]
#[interface(name = "org.bluez.GattCharacteristic1")]
impl SerialNumberChrcInterface {
    fn read_value(&self, _options: HashMap<String, String>) -> zbus::fdo::Result<Vec<u8>> {
        Ok(self.0.lock().unwrap().value.clone())
    }
}
