use crate::bluez::base_gatt_service::BaseGattService;
use crate::constants::{GATT_SERVICE_HID_UUID, GATT_SERVICE_IFACE};
use crate::hidimpl::hid_control_point_chrc::HidControlPointChrc;
use crate::hidimpl::hid_info_chrc::HidInfoChrc;
use crate::hidimpl::protocol_mode_chrc::ProtocolModeChrc;
use crate::hidimpl::report_chrc::ReportChrc;
use crate::hidimpl::report_map_chrc::ReportMapChrc;
use crate::utils::{ObjectInterfaces, ObjectPathTrait};
use crate::{extend_option_prop, extend_service_props, object_path};
use macros::gatt_service;
use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::{Arc, Mutex};
use zbus::interface;
use zbus::zvariant::OwnedValue;

#[derive(Debug)]
pub struct HidService {
    pub base: BaseGattService,
    pub report_chrc: Option<Arc<Mutex<ReportChrc>>>,
    pub reportmap_chrc: Option<Arc<Mutex<ReportMapChrc>>>,
    pub protocol_mode_chrc: Option<Arc<Mutex<ProtocolModeChrc>>>,
    pub hid_info_chrc: Option<Arc<Mutex<HidInfoChrc>>>,
    pub hid_control_point_chrc: Option<Arc<Mutex<HidControlPointChrc>>>,
}

object_path! {
    impl HidService {
        pub fn new(path: String) -> Self {
            Self {
                base: BaseGattService::new(path, GATT_SERVICE_HID_UUID.to_string(), true, vec![]),
                report_chrc: None,
                reportmap_chrc: None,
                protocol_mode_chrc: None,
                hid_info_chrc: None,
                hid_control_point_chrc: None,
            }
        }

        pub fn add_characteristic_path(&mut self, path: String) {
            self.base.characteristics.push(path);
        }

        pub fn get_properties (&self) -> ObjectInterfaces {
            let mut properties = HashMap::new();
            extend_service_props!(&self, properties);

            extend_option_prop!(&self.report_chrc, properties);
            extend_option_prop!(&self.reportmap_chrc, properties);
            extend_option_prop!(&self.protocol_mode_chrc, properties);
            extend_option_prop!(&self.hid_info_chrc, properties);
            extend_option_prop!(&self.hid_control_point_chrc, properties);

            properties
        }
    }
}
pub(crate) struct HidServiceInterface(pub Arc<Mutex<HidService>>);

#[gatt_service()]
impl HidServiceInterface {}
