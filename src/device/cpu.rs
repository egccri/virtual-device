use crate::device::flash::Flash;
use crate::resource::Runnable;
use std::sync::Arc;
use std::thread;

pub struct CPU {
    shared_flash: Arc<Flash>,
}

impl<T> CPU {
    fn run(&self, tasks: Vec<Box<dyn Runnable>>) {
        thread::spawn(move || {
            for task in tasks {
                task.run(self.shared_flash.clone());
            }
        });
    }
}
