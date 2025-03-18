use crate::bluez::BaseGattCharacteristic;
use crate::utils::ObjectPathTrait;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use zbus::interface;

#[derive(Debug)]
pub struct ModelNumberChrc {
    pub base: BaseGattCharacteristic,
    pub value: Vec<u8>,
}

impl ObjectPathTrait for ModelNumberChrc {
    fn object_path(&self) -> String {
        self.base.path.to_string()
    }
}

impl ModelNumberChrc {
    pub fn new(path: String, service: String) -> Self {
        let uuid = "2a24".to_string();
        let flags = vec!["read".to_string()];
        Self {
            base: BaseGattCharacteristic::new(path, uuid, flags, service, vec![]),
            value: "GP".as_bytes().to_vec(),
        }
    }
}

pub(crate) struct ModelNumberChrcInterface(pub Arc<Mutex<ModelNumberChrc>>);

#[interface(name = "org.bluez.GattCharacteristic1")]
impl ModelNumberChrcInterface {
    fn read_value(&self, _options: HashMap<String, String>) -> zbus::fdo::Result<Vec<u8>> {
        Ok(self.value.clone())
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
