use crate::bluez::base_gatt_chrc::BaseGattCharacteristic;
use crate::object_path;
use crate::utils::ObjectPathTrait;
use macros::gatt_characteristic;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use zbus::interface;

#[derive(Debug)]
pub struct HardwareRevisionChrc {
    pub base: BaseGattCharacteristic,
    pub value: Vec<u8>,
}

object_path! {
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
}

pub(crate) struct HardwareRevisionChrcInterface(pub Arc<Mutex<HardwareRevisionChrc>>);

#[gatt_characteristic()]
impl HardwareRevisionChrcInterface {
    fn read_value(&self, _options: HashMap<String, String>) -> zbus::fdo::Result<Vec<u8>> {
        Ok(self.0.lock().unwrap().value.clone())
    }
}
