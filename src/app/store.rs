use std::collections::HashMap;

pub struct Store {
    pub data: HashMap<String, String>,
}

impl Store {
    pub fn new() -> Store {
        Store {
            data: HashMap::new(),
        }
    }
}
