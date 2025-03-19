use crate::utils::ObjectPathTrait;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use macros::gatt_chrc_properties;
use zbus::interface;
use crate::bluez::base_gatt_chrc::BaseGattCharacteristic;

#[derive(Debug)]
pub struct HardwareRevisionChrc {
    pub base: BaseGattCharacteristic,
    pub value: Vec<u8>,
}

impl ObjectPathTrait for HardwareRevisionChrc {
    fn object_path(&self) -> String {
        self.base.path.to_string()
    }
}

impl HardwareRevisionChrc {
    pub fn new(path: String, service: String) -> Self {
        let uuid = "2a27".to_string();
        let flags = vec!["read".to_string()];
        Self {
            base: BaseGattCharacteristic::new(path, uuid, flags, service, vec![]),
            value: "1.0".as_bytes().to_vec(),
        }
    }
}

pub(crate) struct HardwareRevisionChrcInterface(pub Arc<Mutex<HardwareRevisionChrc>>);

#[gatt_chrc_properties()]
#[interface(name = "org.bluez.GattCharacteristic1")]
impl HardwareRevisionChrcInterface {
    fn read_value(&self, _options: HashMap<String, String>) -> zbus::fdo::Result<Vec<u8>> {
        Ok(self.0.lock().unwrap().value.clone())
    }
}
