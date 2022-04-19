use crate::device::flash::Flash;
use crate::resource::{ResourceNeed, Runnable};
use std::any::Any;
use std::sync::Arc;

pub struct ResourceFloat {}

impl ResourceFloat {
    pub fn new() -> Self {
        ResourceFloat {}
    }
}

impl ResourceNeed for ResourceFloat {
    fn value(&self) -> Box<dyn Any> {
        let value: f32 = 1.5;
        Box::new(value)

        // Box::new(1.5)
    }

    fn write(&self, value: Box<dyn Any>, shared_flash: Arc<Flash>) {
        shared_flash.update_resource_value(
            "float".to_string(),
            value.downcast::<f32>().unwrap().to_string(),
        )
    }
}

impl Runnable for ResourceFloat {
    fn run(&self, shared_flash: Arc<Flash>) {
        let value = self.value();
        self.write(value, shared_flash);
    }
}
