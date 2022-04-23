use crate::device::device_profile::DeviceResource;
use crate::resource::{ResourceNeed, Runnable};
use crate::virtualdevice::flash::Flash;
use rand::Rng;
use std::any::Any;
use std::sync::Arc;

pub struct ResourceInt {
    // must defined in the profile
    device_resource: &'static DeviceResource,
    speed: i32,
}

impl ResourceInt {
    pub fn new(device_resource: &'static DeviceResource, speed: i32) -> Self {
        ResourceInt { device_resource, speed }
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
            self.device_resource.get_resource_name(),
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
