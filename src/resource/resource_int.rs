use crate::device::flash::Flash;
use crate::resource::{ResourceNeed, Runnable};
use std::sync::Arc;

pub struct ResourceInt {}

impl ResourceNeed for ResourceInt {

    type ValueType = i32;

    fn value(&self) -> Self::ValueType {
        12
    }

    fn write(&self, value: Self::ValueType, shared_flash: Arc<Flash>) {
        shared_flash.update_resource_value("temp".to_string(), value.to_string())
    }
}

impl Runnable for ResourceInt {
    fn run(self, shared_flash: Arc<Flash>) {
        let value = self.value();
        self.write(value, shared_flash);
    }
}
