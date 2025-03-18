#[derive(Debug)]
pub struct BaseGattApplication {
    pub path: String,
    pub services: Vec<String>,
}

impl BaseGattApplication {
    pub fn new(path: String, services: Vec<String>) -> Self {
        Self { path, services }
    }
}
