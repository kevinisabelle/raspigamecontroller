use crate::hid::HidService;
use crate::utils::ObjectPathTrait;
use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use std::sync::{Arc, Mutex};
use zbus::interface;
use zbus::zvariant::Value;

pub struct GattApplication {
    pub path: String,
    pub hid_service: Option<&'static HidService>,
}

impl ObjectPathTrait for GattApplication {
    fn object_path(&self) -> String {
        self.path.to_string()
    }
}

impl Debug for GattApplication {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "GattApplication {{ path: {} }}", self.path)
    }
}

impl GattApplication {
    pub fn new(path: String) -> Self {
        Self {
            path,
            hid_service: None,
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
