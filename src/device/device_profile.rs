use crate::device::device_profile::AccessMode::{ReadOnly, ReadWrite};
use crate::resource::VirtualResource;
use serde::{Deserialize, Serialize};
use std::fmt::{self, Display, Formatter};

/// There is a small different with kubeedge and edgex virtualdevice model or virtualdevice profile.
/// This profile doesn't support report single resource, so resource's attrs are unsupported,
/// if need like timestamp attr, define it as a single resource in virtualdevice resource level.

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct DeviceList {
    pub device_num: Option<i32>,
    pub devices: Vec<Device>,
    pub virtual_resources: Vec<VirtualResource>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Device {
    pub device_id: String,
    pub device_name: String,
    pub device_description: Option<String>,
    pub device_profile: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct DeviceProfile {
    pub profile_name: String,
    pub profile_description: String,
    pub device_resources: Vec<DeviceResource>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct DeviceResource {
    pub(crate) resource_name: String,
    resource_description: String,
    value: Value,
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
