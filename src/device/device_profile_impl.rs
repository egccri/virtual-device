use crate::device::device_profile::{DeviceProfile, DeviceResource};

impl DeviceProfile {
    pub fn get_resource(&'static self, _: &str) -> &'static DeviceResource {
        &self.device_resources[0]
    }
}

impl DeviceResource {
    pub fn get_resource_name(&self) -> String {
        self.resource_name.clone()
    }
}