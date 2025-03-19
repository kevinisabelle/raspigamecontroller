use crate::constants::BATTERY_SERVICE_UUID;
use crate::utils::ObjectPathTrait;
use std::fmt::Debug;
use std::sync::{Arc, Mutex};
use macros::gatt_service_properties;
use zbus::interface;
use crate::bluez::base_gatt_service::BaseGattService;

#[derive(Debug)]
pub struct BatteryService {
    pub base: BaseGattService,
}

impl ObjectPathTrait for BatteryService {
    fn object_path(&self) -> String {
        self.base.path.to_string()
    }
}

impl BatteryService {
    pub fn new(path: String) -> Self {
        Self {
            base: BaseGattService::new(path, BATTERY_SERVICE_UUID.to_string(), true, vec![]),
        }
    }

    pub fn add_characteristic_path(&mut self, path: String) {
        self.base.characteristics.push(path);
    }
}

pub(crate) struct BatteryServiceInterface(pub Arc<Mutex<BatteryService>>);

#[gatt_service_properties()]
#[interface(name = "org.bluez.GattService1")]
impl BatteryServiceInterface {
   
}
