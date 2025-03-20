use crate::hidimpl::battery_service::BatteryService;
use crate::hidimpl::device_info_service::DeviceInfoService;
use crate::hidimpl::hid_service::HidService;
use crate::utils::ObjectPathTrait;
use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::{Arc, Mutex};
use zbus::interface;
use zbus::zvariant::Value;

#[derive(Debug)]
pub struct GattApplication {
    pub path: String,
    pub hid_service: Option<Arc<Mutex<HidService>>>,
    pub battery_service: Option<Arc<Mutex<BatteryService>>>,
    pub device_info_service: Option<Arc<Mutex<DeviceInfoService>>>,
}

impl ObjectPathTrait for GattApplication {
    fn object_path(&self) -> String {
        self.path.to_string()
    }
}

impl GattApplication {
    pub fn new(path: String) -> Self {
        Self {
            path,
            hid_service: None,
            battery_service: None,
            device_info_service: None,
        }
    }
    
    pub fn notify_hid_report (&self) {
        if let Some(hid_service) = &self.hid_service {
            let hid_service = hid_service.lock().unwrap();
            if let Some(report_chrc) = &hid_service.report_chrc {
                let report_chrc = report_chrc.lock().unwrap();
                report_chrc.notify_value_changed();
            }
        }
    }
}

pub(crate) struct GattApplicationInterface(pub Arc<Mutex<GattApplication>>);

#[interface(name = "org.bluez.GattApplication1")]
impl GattApplicationInterface {
    // out_signature='a{oa{sa{sv}}}')
    pub fn get_managed_objects(&self) -> HashMap<String, HashMap<String, HashMap<String, Value>>> {
        HashMap::new()
    }
}
