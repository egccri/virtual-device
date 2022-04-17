use crate::config::device_profile::AccessMode::{ReadOnly, ReadWrite};
use serde::{Deserialize, Serialize};
use std::fmt::{self, Display, Formatter};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Devices {
    devices: Vec<Device>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Device {
    resources: Vec<DeviceResource>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct DeviceResource {
    resource_name: String,
    description: String,
    value: Value,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Value {
    value_type: Type,
    access_mode: AccessMode,
    minimum: String,
    maximum: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum Type {
    STRING(String),
    INI(i32),
    DOUBLE(f32),
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
