use crate::device::device_profile::Device;
use crate::resource::VirtualResource;
use crate::{Flash, Runnable, CPU};
use std::sync::Arc;
use std::thread::JoinHandle;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::sync::mpsc::Sender;

pub mod cpu;
pub mod flash;

#[derive(Debug)]
pub struct VirtualDeviceTwin {
    flash: Arc<Flash>,
    device_id: String,
    virtual_resources: Vec<VirtualResource>,
    sender: Sender<String>,
}

impl VirtualDeviceTwin {
    pub fn new(
        device: Device,
        virtual_resources: Vec<VirtualResource>,
        sender: Sender<String>,
    ) -> Self {
        let flash = Flash::new();
        let shared_flash = Arc::new(flash);
        VirtualDeviceTwin {
            flash: shared_flash,
            device_id: device.device_id,
            virtual_resources,
            sender,
        }
    }

    pub fn run(self) -> JoinHandle<()> {
        let mut vec = Vec::<Box<dyn Runnable>>::new();
        for virtual_resource in self.virtual_resources {
            vec.push(virtual_resource.gen());
        }
        self.flash
            .update_resource_value("device_id".to_string(), self.device_id);
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();
        self.flash
            .update_resource_value("event_time".to_string(), now.to_string());
        CPU::run(self.flash, vec, self.sender)
    }
}
