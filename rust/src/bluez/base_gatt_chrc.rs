use std::collections::HashMap;
use zbus::zvariant::{ObjectPath, OwnedObjectPath, OwnedValue, Value};

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

    pub fn get_properties(&self) -> HashMap<String, OwnedValue> {
        let mut properties: HashMap<String, OwnedValue> = HashMap::new();
        properties.insert(
            "Flags".to_string(),
            OwnedValue::try_from(Value::from(&self.flags.clone())).unwrap(),
        );
        properties.insert(
            "UUID".to_string(),
            OwnedValue::try_from(Value::from(&self.uuid.clone())).unwrap(),
        );
        properties.insert(
            "Service".to_string(),
            OwnedValue::try_from(
                ObjectPath::try_from(self.service.clone()).unwrap(),
            ).unwrap(),
        );
        properties.insert(
            "Descriptors".to_string(),
            OwnedValue::try_from(Value::from(&self.descriptors.clone())).unwrap(),
        );
        properties
    }
}
