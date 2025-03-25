use crate::bluez::base_gatt_service::BaseGattService;
use crate::constants::DEVICE_INFORMATION_SERVICE_UUID;
use crate::hidimpl::hardware_revision_chrc::HardwareRevisionChrc;
use crate::hidimpl::manufacturer_name_chrc::ManufacturerNameChrc;
use crate::hidimpl::model_number_chrc::ModelNumberChrc;
use crate::hidimpl::pnpid_chrc::PnpIdChrc;
use crate::hidimpl::serial_number_chrc::SerialNumberChrc;
use crate::utils::{ObjectInterfaces, ObjectPathTrait};
use crate::{extend_option_prop, extend_service_props, object_path};
use macros::gatt_service;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use zbus::interface;

#[derive(Debug)]
pub struct DeviceInfoService {
    pub base: BaseGattService,
    pub manufacturer_name_chrc: Option<Arc<Mutex<ManufacturerNameChrc>>>,
    pub serial_number_chrc: Option<Arc<Mutex<SerialNumberChrc>>>,
    pub hardware_revision_chrc: Option<Arc<Mutex<HardwareRevisionChrc>>>,
    pub model_number_chrc: Option<Arc<Mutex<ModelNumberChrc>>>,
    pub pnp_id_chrc: Option<Arc<Mutex<PnpIdChrc>>>,
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
                manufacturer_name_chrc: None,
                serial_number_chrc: None,
                hardware_revision_chrc: None,
                model_number_chrc: None,
                pnp_id_chrc: None,
            }
        }

        pub fn add_characteristic_path(&mut self, path: String) {
            self.base.characteristics.push(path);
        }

        pub fn get_properties(&self) -> ObjectInterfaces {
            let mut properties: ObjectInterfaces = HashMap::new();

            extend_service_props!(&self, properties);

            extend_option_prop!(&self.pnp_id_chrc, properties);
            extend_option_prop!(&self. manufacturer_name_chrc, properties);
            extend_option_prop!(&self.serial_number_chrc, properties);
            extend_option_prop!(&self.hardware_revision_chrc, properties);
            extend_option_prop!(&self.model_number_chrc, properties);

            properties
        }
    }
}

pub(crate) struct DeviceInfoServiceInterface(pub Arc<Mutex<DeviceInfoService>>);

#[gatt_service()]
impl DeviceInfoServiceInterface {}
