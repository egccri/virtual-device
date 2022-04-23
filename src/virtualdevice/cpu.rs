use crate::resource::Runnable;
use crate::virtualdevice::flash::Flash;
use std::sync::Arc;
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;

pub struct CPU {}

impl CPU {
    pub fn run(shared_flash: Arc<Flash>, tasks: Vec<Box<dyn Runnable>>) -> JoinHandle<()> {
        let handle = thread::spawn(move || {
            loop {
                for task in &tasks {
                    task.run(shared_flash.clone());
                }
                thread::sleep(Duration::from_secs(1));
                println!("{:?}", shared_flash.to_json());
            }
        });
        handle
    }
}
