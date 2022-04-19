use crate::device::flash::Flash;
use crate::resource::resource_int::ResourceInt;
use std::any::Any;
use std::sync::Arc;

pub mod resource_float;
pub mod resource_int;
pub mod types;

pub trait ResourceNeed {
    fn value(&self) -> Box<dyn Any>;

    fn write(&self, value: Box<dyn Any>, shared_flash: Arc<Flash>);
}

pub trait Runnable: ResourceNeed + Send + Sync + 'static {
    fn run(&self, shared_flash: Arc<Flash>);
}
