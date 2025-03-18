
#[derive(Debug)]
pub struct BaseGattDescriptor {
    pub path: String,
    pub uuid: String,
    pub flags: Vec<String>,
    pub characteristic: String,
}

impl BaseGattDescriptor {
    pub fn new(path: String, uuid: String, flags: Vec<String>, characteristic: String) -> Self {
        Self {
            path,
            uuid,
            flags,
            characteristic,
        }
    }
}