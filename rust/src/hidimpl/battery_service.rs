use crate::bluez::base_gatt_service::BaseGattService;
use crate::constants::{BATTERY_SERVICE_UUID, GATT_SERVICE_IFACE};
use crate::hidimpl::battery_level_chrc::BatteryLevelChrc;
use crate::utils::{InterfaceProperties, ObjectInterfaces, ObjectPathTrait, ObjectProperties};
use crate::{extend_option_prop, extend_service_props, object_path};
use macros::gatt_service;
use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::{Arc, Mutex};
use zbus::interface;

#[derive(Debug)]
pub struct BatteryService {
    pub base: BaseGattService,
    pub battery_level_chrc: Option<Arc<Mutex<BatteryLevelChrc>>>,
}

object_path! {
    impl BatteryService {
        pub fn new(path: String) -> Self {
            Self {
                base: BaseGattService::new(path, BATTERY_SERVICE_UUID.to_string(), true, vec![]),
                battery_level_chrc: None
            }
        }

        pub fn add_characteristic_path(&mut self, path: String) {
            self.base.characteristics.push(path);
        }
        
        pub fn get_properties (&self) -> ObjectInterfaces {
            
            let mut properties: ObjectInterfaces = HashMap::new();
            extend_service_props!(&self, properties);
            extend_option_prop!(&self.battery_level_chrc, properties);

            properties
        }
    }
}

pub(crate) struct BatteryServiceInterface(pub Arc<Mutex<BatteryService>>);

#[gatt_service()]
impl BatteryServiceInterface {}
