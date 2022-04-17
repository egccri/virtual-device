use crate::device::cpu::CPU;
use crate::device::flash::Flash;
use crate::resource::resource_int::ResourceInt;
use crate::resource::{Resource, Runnable};
use std::sync::Arc;

mod config;
mod device;
mod resource;
mod upstream;

fn main() {
    // initial one device
    let flash = Flash::new();
    let shared_flash = Arc::new(flash);

    let mut resource_int = ResourceInt::new();

    let mut vec = Vec::<Box<dyn Runnable<ValueType = i32>>>::new();
    vec.push(Box::new(resource_int));

    CPU::run(shared_flash, vec);
}
