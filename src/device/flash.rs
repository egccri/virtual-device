use std::collections::HashMap;
use std::sync::Mutex;

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

    fn to_hex(&self) {
        todo!()
    }
}
