use crate::hidimpl::battery_service::BatteryService;
use crate::hidimpl::device_info_service::DeviceInfoService;
use crate::hidimpl::hid_service::HidService;
use crate::utils::{ObjectInterfaces, ObjectPathTrait};
use crate::{extend_option_prop, extend_service_props};
use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::{Arc, Mutex};
use zbus::interface;
use zbus::zvariant::OwnedValue;

#[derive(Debug, Clone)]
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

    pub fn notify_hid_report(&self) {
        if let Some(hid_service) = &self.hid_service {
            let hid_service = hid_service.lock().unwrap();
            if let Some(report_chrc) = &hid_service.report_chrc {
                let report_chrc = report_chrc.lock().unwrap();
                report_chrc.notify_value_changed();
            }
        }
    }

    pub fn get_properties(&self) -> ObjectInterfaces {
        let mut properties: ObjectInterfaces = HashMap::new();

        // extend_service_props!(&self, properties);

        extend_option_prop!(&self.hid_service, properties);
        extend_option_prop!(&self.battery_service, properties);
        extend_option_prop!(&self.device_info_service, properties);

        properties
    }
}

pub(crate) struct GattApplicationInterface(pub Arc<Mutex<GattApplication>>);

#[interface(name = "org.bluez.GattApplication1")]
impl GattApplicationInterface {}

// Add this new interface for ObjectManager
pub(crate) struct ObjectManagerInterface(pub Arc<Mutex<GattApplication>>);

#[interface(name = "org.freedesktop.DBus.ObjectManager")]
impl ObjectManagerInterface {
    // This is the method BlueZ will call to discover your GATT structure
    pub fn get_managed_objects(
        &self,
    ) -> zbus::fdo::Result<ObjectInterfaces> {
        let locked_app = &self.0.lock().unwrap();
        let mut objects = HashMap::new();

        // Get the properties of the GattApplication
        let app_properties = locked_app.get_properties().clone();

        // Insert the properties of the GattApplication
        objects.extend(app_properties);

        Ok(objects)
    }
}
