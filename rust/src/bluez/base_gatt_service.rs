#[derive(Debug)]
pub struct BaseGattService {
    pub path: String,
    pub uuid: String,
    pub primary: bool,
    pub characteristics: Vec<String>,
}

impl BaseGattService {
    pub fn new(path: String, uuid: String, primary: bool, characteristics: Vec<String>) -> Self {
        Self {
            path,
            uuid,
            primary,
            characteristics,
        }
    }
}