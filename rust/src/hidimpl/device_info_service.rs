use crate::bluez::base_gatt_service::BaseGattService;
use crate::constants::DEVICE_INFORMATION_SERVICE_UUID;
use crate::object_path;
use crate::utils::ObjectPathTrait;
use macros::gatt_service;
use std::sync::{Arc, Mutex};
use zbus::interface;

#[derive(Debug)]
pub struct DeviceInfoService {
    pub base: BaseGattService,
}

object_path! {
    impl DeviceInfoService {
        pub fn new(path: String) -> Self {
            Self {
                base: BaseGattService::new(
                    path,
                    DEVICE_INFORMATION_SERVICE_UUID.to_string(),
                    true,
                    vec![],
                ),
            }
        }

        pub fn add_characteristic_path(&mut self, path: String) {
            self.base.characteristics.push(path);
        }
    }
}

pub(crate) struct DeviceInfoServiceInterface(pub Arc<Mutex<DeviceInfoService>>);

#[gatt_service()]
impl DeviceInfoServiceInterface {}
