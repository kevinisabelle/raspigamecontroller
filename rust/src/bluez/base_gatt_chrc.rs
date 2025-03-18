use std::collections::HashMap;
use zbus::zvariant::Value;
use zbus::interface;

#[derive(Debug)]
pub struct BaseGattCharacteristic {
    pub path: String,
    pub uuid: String,
    pub flags: Vec<String>,
    pub service: String,
    pub descriptors: Vec<String>,
}

impl BaseGattCharacteristic {
    pub fn new(
        path: String,
        uuid: String,
        flags: Vec<String>,
        service: String,
        descriptors: Vec<String>,
    ) -> Self {
        Self {
            path,
            uuid,
            flags,
            service,
            descriptors,
        }
    }

    pub fn get_properties(&self) -> HashMap<String, Value> {
        let mut properties: HashMap<String, Value> = HashMap::new();
        properties.insert("Flags".to_string(), Value::from(self.flags.clone()));
        properties.insert("UUID".to_string(), Value::from(self.uuid.clone()));
        properties.insert("Service".to_string(), Value::from(self.service.clone()));
        properties.insert(
            "Descriptors".to_string(),
            Value::from(self.descriptors.clone()),
        );
        properties
    }
}

#[macro_export]
macro_rules! gatt_chrc_properties {
    () => {
        paste! {
            #[zbus(property)]
            fn get_flags(&self) -> Vec<String> {
                self.0.lock().unwrap().base.flags.clone()
            }

            #[zbus(property)]
            fn get_uuid(&self) -> String {
                self.0.lock().unwrap().base.uuid.clone()
            }

            #[zbus(property)]
            fn get_service(&self) -> String {
                self.0.lock().unwrap().base.service.clone()
            }

            #[zbus(property)]
            fn get_descriptors(&self) -> Vec<String> {
                self.0.lock().unwrap().base.descriptors.clone()
            }
        }
    };
}
