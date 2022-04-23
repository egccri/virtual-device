use crate::device::device_profile::DeviceResource;
use crate::device::DEVICE_PROFILES;
use crate::resource::VirtualResource::{ResourceFloat, ResourceInt};
use crate::virtualdevice::flash::Flash;
use serde::{Deserialize, Deserializer, Serialize};
use std::any::Any;
use std::sync::Arc;

pub mod resource_float;
pub mod resource_int;
pub mod types;

// 产生值的速度（时间间隔）
pub type Speed = i32;

// 放入构建必须的属性
#[derive(Debug, PartialEq, Serialize, Copy, Clone)]
pub enum VirtualResource {
    ResourceFloat(&'static DeviceResource, Speed),
    ResourceInt(&'static DeviceResource, Speed),
}

impl<'de> Deserialize<'de> for VirtualResource {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let a: Vec<&str> = s.split(":").collect();
        let profile_name = a[0];
        let resource_name = a[1];
        let virtual_resource = a[2];
        if virtual_resource == "ResourceFloat" {
            Ok(ResourceFloat(
                DEVICE_PROFILES
                    .get(profile_name)
                    .unwrap()
                    .get_resource(resource_name),
                1,
            ))
        } else if virtual_resource == "ResourceInt" {
            Ok(ResourceInt(
                DEVICE_PROFILES
                    .get(profile_name)
                    .unwrap()
                    .get_resource(resource_name),
                1,
            ))
        } else {
            Ok(ResourceInt(
                DEVICE_PROFILES
                    .get(profile_name)
                    .unwrap()
                    .get_resource(resource_name),
                1,
            ))
        }
    }
}

impl VirtualResource {
    pub fn gen(self) -> Box<dyn Runnable> {
        match self {
            VirtualResource::ResourceFloat(device_resource, _) => {
                Box::new(resource_float::ResourceFloat::new(device_resource))
            }
            VirtualResource::ResourceInt(device_resource, _) => {
                Box::new(resource_int::ResourceInt::new(device_resource))
            }
        }
    }
}

pub trait ResourceNeed {
    fn value(&self) -> Box<dyn Any>;

    fn write(&self, value: Box<dyn Any>, shared_flash: Arc<Flash>);
}

pub trait Runnable: ResourceNeed + Send + Sync + 'static {
    fn run(&self, shared_flash: Arc<Flash>);
}
