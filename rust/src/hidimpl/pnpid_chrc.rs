use crate::bluez::base_gatt_chrc::BaseGattCharacteristic;
use crate::utils::ObjectPathTrait;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use macros::gatt_characteristic;
use zbus::interface;

#[derive(Debug)]
pub struct PnpIdChrc {
    pub base: BaseGattCharacteristic,
}

impl ObjectPathTrait for PnpIdChrc {
    fn object_path(&self) -> String {
        self.base.path.to_string()
    }
}

impl PnpIdChrc {
    pub fn new(path: String, service: String) -> Self {
        let uuid = "2a50".to_string();
        let flags = vec!["read".to_string()];
        Self {
            base: BaseGattCharacteristic::new(path, uuid, flags, service, vec![]),
        }
    }
}

pub(crate) struct PnpIdChrcInterface(pub Arc<Mutex<PnpIdChrc>>);

#[gatt_characteristic()]
impl PnpIdChrcInterface {
    fn read_value(&self, _options: HashMap<String, String>) -> zbus::fdo::Result<Vec<u8>> {
        Ok(vec![
            0x0, 0x2, 0x6, 0xD, 0x0, 0x4, 0x7, 0x8, 0x5, 0x6, 0x0, 0x0, 0x0, 0x1,
        ])
    }
}
