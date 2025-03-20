use crate::constants::DEVICE_INFORMATION_SERVICE_UUID;
use crate::utils::ObjectPathTrait;
use std::sync::{Arc, Mutex};
use macros::gatt_service;
use zbus::interface;
use crate::bluez::base_gatt_service::BaseGattService;

#[derive(Debug)]
pub struct DeviceInfoService {
    pub base: BaseGattService,
}

impl ObjectPathTrait for DeviceInfoService {
    fn object_path(&self) -> String {
        self.base.path.to_string()
    }
}

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

pub(crate) struct DeviceInfoServiceInterface(pub Arc<Mutex<DeviceInfoService>>);

#[gatt_service()]
impl DeviceInfoServiceInterface {
   
}
