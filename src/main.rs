use std::fs::File;

use crate::device::device_profile::DeviceList;
use crate::resource::Runnable;
use crate::virtualdevice::cpu::CPU;
use crate::virtualdevice::flash::Flash;
use crate::virtualdevice::VirtualDeviceTwin;

pub mod device;
mod reporter;
pub mod resource;
mod virtualdevice;

#[tokio::main]
async fn main() {
    let device_json = File::open("config/device.json").expect("Device config file open error!");
    let device_list: DeviceList = serde_json::from_reader(device_json).unwrap();

    for device in device_list.devices {
        let virtual_device_twin = VirtualDeviceTwin::new(device, device_list.virtual_resources.clone());
        virtual_device_twin.run();
    }

    // match reporter::kafka::kafka_upstream::push().await {
    //     Ok(_) => {}
    //     Err(err) => {
    //         println!("{}", err)
    //     }
    // };
}
