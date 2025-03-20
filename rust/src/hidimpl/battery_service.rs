use crate::bluez::base_gatt_service::BaseGattService;
use crate::constants::BATTERY_SERVICE_UUID;
use crate::object_path;
use crate::utils::ObjectPathTrait;
use macros::gatt_service;
use std::fmt::Debug;
use std::sync::{Arc, Mutex};
use zbus::interface;

#[derive(Debug)]
pub struct BatteryService {
    pub base: BaseGattService,
}

object_path! {
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
}

pub(crate) struct BatteryServiceInterface(pub Arc<Mutex<BatteryService>>);

#[gatt_service()]
impl BatteryServiceInterface {}
