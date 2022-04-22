use crate::device::device_profile::AccessMode::{ReadOnly, ReadWrite};
use serde::{Deserialize, Serialize};
use std::fmt::{self, Display, Formatter};
use crate::resource::VirtualResource;

/// There is a small different with kubeedge and edgex virtualdevice model or virtualdevice profile.
/// This profile doesn't support report single resource, so resource's attrs are unsupported,
/// if need like timestamp attr, define it as a single resource in virtualdevice resource level.

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct DeviceList {
    device_num: Option<i32>,
    devices: Vec<Device>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Device {
    device_id: String,
    device_name: String,
    device_description: Option<String>,
    device_profile: String,
    virtual_resources: Vec<VirtualResource>
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct DeviceProfile {
    profile_name: String,
    profile_description: String,
    pub(crate) device_resources: Vec<DeviceResource>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct DeviceResource {
    pub(crate) resource_name: String,
    resource_description: String,
    value: Value,
    // virtual_resource: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Value {
    value_type: Type,
    access_mode: AccessMode,
    minimum: String,
    maximum: String,
    unit: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum Type {
    STRING,
    INT,
    FLOAT,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum AccessMode {
    ReadOnly = 0,
    ReadWrite = 1,
}

impl Display for AccessMode {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match *self {
            ReadOnly => write!(f, "0 (access mode is read only)."),
            ReadWrite => write!(f, "1 (access mode is read write)."),
        }
    }
}

impl AccessMode {
    pub fn from_bits(bits: u8) -> AccessMode {
        match bits {
            0 => ReadOnly,
            1 => ReadWrite,
            _ => panic!("Invalid access mode."),
        }
    }
}
