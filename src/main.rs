use crate::device::cpu::CPU;
use crate::device::flash::Flash;
use crate::resource::resource_float::ResourceFloat;
use crate::resource::resource_int::ResourceInt;
use crate::resource::Runnable;
use std::sync::Arc;

mod config;
mod device;
mod resource;
mod upstream;

#[tokio::main]
async fn main() {
    // initial one device
    let flash = Flash::new();
    let shared_flash = Arc::new(flash);

    let resource_int = ResourceInt::new();
    let resource_float = ResourceFloat::new();

    let mut vec = Vec::<Box<dyn Runnable>>::new();
    vec.push(Box::new(resource_int));
    vec.push(Box::new(resource_float));

    CPU::run(shared_flash, vec);

    match upstream::kafka::kafka_upstream::push().await {
        Ok(_) => {}
        Err(err) => {
            println!("{}", err)
        }
    };
}
