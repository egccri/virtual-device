use crate::device::device_profile::{DeviceProfile, DeviceResource};

impl DeviceProfile {
    pub fn get_resource(&'static self, resource_name: &str) -> &'static DeviceResource {
        let device_resource =
            self.device_resources.iter().find(|resource| resource.get_resource_name() == resource_name);
        &device_resource.unwrap()
    }
}

impl DeviceResource {
    pub fn get_resource_name(&self) -> String {
        self.resource_name.clone()
    }
}
