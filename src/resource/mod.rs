use crate::device::flash::Flash;
use crate::resource::resource_int::ResourceInt;
use std::sync::Arc;

mod resource_int;

pub enum Resource {
    Int(ResourceInt),
}

pub trait ResourceNeed {

    type ValueType;

    fn value(&self) -> Self::ValueType;

    fn write(&self, value: Self::ValueType, shared_flash: Arc<Flash>);
}

pub trait Runnable: ResourceNeed + Send + Sync + 'static {
    fn run(&self, shared_flash: Arc<Flash>);
}
