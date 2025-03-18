use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use zbus::interface;
use crate::bluez::BaseGattCharacteristic;
use crate::utils::ObjectPathTrait;

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

#[interface(name = "org.bluez.GattCharacteristic1")]
impl PnpIdChrcInterface {
    fn read_value(&self, _options: HashMap<String, String>) -> zbus::fdo::Result<Vec<u8>> {
        Ok(vec![
            0x0, 0x2, 0x6, 0xD, 0x0, 0x4, 0x7, 0x8, 0x5, 0x6, 0x0, 0x0, 0x0, 0x1,
        ])
    }

    #[zbus(property)]
    fn get_flags(&self) -> Vec<String> {
        self.base.flags.clone()
    }

    #[zbus(property)]
    fn get_uuid(&self) -> String {
        self.base.uuid.clone()
    }

    #[zbus(property)]
    fn get_service(&self) -> String {
        self.base.service.clone()
    }

    #[zbus(property)]
    fn get_descriptors(&self) -> Vec<String> {
        self.base.descriptors.clone()
    }
}