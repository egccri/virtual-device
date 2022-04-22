use crate::{CPU, Flash};
use crate::resource::VirtualResource;

pub mod cpu;
pub mod flash;

pub struct VirtualDeviceTwin {
    cpu: CPU,
    flash: Flash,
    virtual_resources: Vec<VirtualResource>
}