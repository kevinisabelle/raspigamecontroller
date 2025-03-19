use crate::constants::GATT_SERVICE_HID_UUID;
use crate::utils::ObjectPathTrait;
use std::fmt::Debug;
use std::sync::{Arc, Mutex};
use macros::gatt_service_properties;
use zbus::interface;
use crate::bluez::base_gatt_service::BaseGattService;

#[derive(Debug)]
pub struct HidService {
    pub base: BaseGattService,
}

impl ObjectPathTrait for HidService {
    fn object_path(&self) -> String {
        self.base.path.to_string()
    }
}

impl HidService {
    pub fn new(path: String) -> Self {
        Self {
            base: BaseGattService::new(path, GATT_SERVICE_HID_UUID.to_string(), true, vec![]),
        }
    }

    pub fn add_characteristic_path(&mut self, path: String) {
        self.base.characteristics.push(path);
    }
}

pub(crate) struct HidServiceInterface(pub Arc<Mutex<HidService>>);

#[gatt_service_properties()]
#[interface(name = "org.bluez.GattService1")]
impl HidServiceInterface {
    
}
