use crate::device::device_profile::DeviceResource;
use crate::resource::{ResourceNeed, Runnable};
use crate::virtualdevice::flash::Flash;
use std::any::Any;
use std::sync::Arc;

pub struct ResourceFloat {
    // must device in the profile
    device_resource: &'static DeviceResource,
    speed: i32,
}

impl ResourceFloat {
    pub fn new(device_resource: &'static DeviceResource, speed: i32) -> Self {
        ResourceFloat {
            device_resource,
            speed,
        }
    }
}

impl ResourceNeed for ResourceFloat {
    fn value(&self) -> Box<dyn Any> {
        let value: f32 = 1.5;
        Box::new(value)
    }

    fn write(&self, value: Box<dyn Any>, shared_flash: Arc<Flash>) {
        shared_flash.update_resource_value(
            self.device_resource.get_resource_name(),
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
