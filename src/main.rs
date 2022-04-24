use std::fs::File;
use std::thread;
use std::thread::JoinHandle;
use tokio::runtime::Builder;

use crate::device::device_profile::DeviceList;
use crate::reporter::kafka::kafka_reporter::push;
use crate::resource::Runnable;
use crate::virtualdevice::cpu::CPU;
use crate::virtualdevice::flash::Flash;
use crate::virtualdevice::VirtualDeviceTwin;

pub mod device;
mod reporter;
pub mod resource;
mod virtualdevice;

fn main() {
    let device_json = File::open("config/device.json").expect("Device config file open error!");
    let device_list: DeviceList = serde_json::from_reader(device_json).unwrap();

    let (send, mut recv) = tokio::sync::mpsc::channel(32);

    thread::spawn(move || {
        Builder::new_multi_thread()
            .worker_threads(2)
            .enable_all()
            .build()
            .unwrap()
            .block_on(async {
                loop {
                    while let Some(message) = recv.recv().await {
                        tokio::spawn(push(message, None));
                    }
                }
            });
    });

    let mut join_handles = Vec::<JoinHandle<()>>::new();

    for device in device_list.devices {
        let virtual_device_twin =
            VirtualDeviceTwin::new(device, device_list.virtual_resources.clone(), send.clone());
        join_handles.push(virtual_device_twin.run());
    }

    for join_handle in join_handles {
        join_handle.join().unwrap();
    }
}
