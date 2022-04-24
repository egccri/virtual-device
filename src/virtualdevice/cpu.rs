use crate::resource::Runnable;
use crate::virtualdevice::flash::Flash;
use std::sync::Arc;
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;
use tokio::sync::mpsc::Sender;

pub struct CPU {}

impl CPU {
    pub fn run(
        shared_flash: Arc<Flash>,
        tasks: Vec<Box<dyn Runnable>>,
        sender: Sender<String>,
    ) -> JoinHandle<()> {
        let handle = thread::spawn(move || loop {
            for task in &tasks {
                task.run(shared_flash.clone());
            }
            thread::sleep(Duration::from_secs(1));
            println!("{:?}", shared_flash.to_json());
            sender.blocking_send(shared_flash.to_json());
        });
        handle
    }
}
