use std::collections::HashMap;
use zbus::zvariant::Value;

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
