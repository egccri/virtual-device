use std::collections::HashMap;
use std::ops::Deref;
use std::sync::Mutex;

#[derive(Debug)]
pub struct Flash {
    store: Mutex<HashMap<String, String>>,
}

impl Flash {
    pub fn new() -> Self {
        Flash {
            store: Mutex::new(HashMap::<String, String>::new()),
        }
    }

    pub fn update_resource_value(&self, resource_name: String, value: String) {
        self.store.lock().unwrap().insert(resource_name, value);
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(self.store.lock().unwrap().deref()).unwrap()
    }

    fn to_hex(&self) {
        todo!()
    }
}
