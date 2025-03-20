use crate::constants::GATT_SERVICE_HID_UUID;
use crate::utils::ObjectPathTrait;
use std::fmt::Debug;
use std::sync::{Arc, Mutex};
use macros::{gatt_service};
use zbus::interface;
use crate::bluez::base_gatt_service::BaseGattService;
use crate::hidimpl::report_chrc::ReportChrc;
use crate::object_path;

#[derive(Debug)]
pub struct HidService {
    pub base: BaseGattService,
    pub report_chrc: Option<Arc<Mutex<ReportChrc>>>
}

object_path! {
    impl HidService {
        pub fn new(path: String) -> Self {
            Self {
                base: BaseGattService::new(path, GATT_SERVICE_HID_UUID.to_string(), true, vec![]),
                report_chrc: None,
            }
        }
    
        pub fn add_characteristic_path(&mut self, path: String) {
            self.base.characteristics.push(path);
        }
    }
}
pub(crate) struct HidServiceInterface(pub Arc<Mutex<HidService>>);

#[gatt_service()]
impl HidServiceInterface {
    
}
