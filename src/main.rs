use crate::virtualdevice::cpu::CPU;
use crate::virtualdevice::flash::Flash;
use crate::resource::resource_float::ResourceFloat;
use crate::resource::resource_int::ResourceInt;
use crate::resource::Runnable;
use std::sync::Arc;

pub mod device;
mod virtualdevice;
pub mod resource;
mod reporter;

#[tokio::main]
async fn main() {
    // initial one virtualdevice
    let flash = Flash::new();
    let shared_flash = Arc::new(flash);

    // let resource_int = ResourceInt::new("resource_name".to_string());
    // let resource_float = ResourceFloat::new("resource_name".to_string());

    let mut vec = Vec::<Box<dyn Runnable>>::new();
    // vec.push(Box::new(resource_int));
    // vec.push(Box::new(resource_float));

    CPU::run(shared_flash, vec);

    match reporter::kafka::kafka_upstream::push().await {
        Ok(_) => {}
        Err(err) => {
            println!("{}", err)
        }
    };
}
