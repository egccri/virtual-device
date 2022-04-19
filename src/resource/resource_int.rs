use crate::device::flash::Flash;
use crate::resource::{ResourceNeed, Runnable};
use rand::Rng;
use std::any::Any;
use std::sync::Arc;

pub struct ResourceInt {}

impl ResourceInt {
    pub fn new() -> Self {
        ResourceInt {}
    }
}

impl ResourceNeed for ResourceInt {
    fn value(&self) -> Box<dyn Any> {
        let mut thread_rng = rand::thread_rng();
        let value: i32 = thread_rng.gen_range(0..10);
        Box::new(value)
    }

    fn write(&self, value: Box<dyn Any>, shared_flash: Arc<Flash>) {
        shared_flash.update_resource_value(
            "int".to_string(),
            value.downcast::<i32>().unwrap().to_string(),
        )
    }
}

impl Runnable for ResourceInt {
    fn run(&self, shared_flash: Arc<Flash>) {
        let value = self.value();
        self.write(value, shared_flash);
    }
}
