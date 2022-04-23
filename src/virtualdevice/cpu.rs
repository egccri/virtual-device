use crate::resource::Runnable;
use crate::virtualdevice::flash::Flash;
use std::sync::Arc;
use std::thread;

pub struct CPU {}

impl CPU {
    pub fn run(shared_flash: Arc<Flash>, tasks: Vec<Box<dyn Runnable>>) {
        let handle = thread::spawn(move || {
            for task in tasks {
                task.run(shared_flash.clone());
            }
        });
        handle.join().unwrap();
    }
}
