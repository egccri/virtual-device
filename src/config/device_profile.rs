use crate::config::device_profile::AccessMode::{ReadOnly, ReadWrite};
use serde::{Deserialize, Deserializer, Serialize};
use std::fmt::{self, Display, Formatter};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct DeviceList {
    devices: Vec<Device>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Device {
    device_id: String,
    device_name: String,
    device_description: Option<String>,
    resources: Vec<DeviceResource>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct DeviceResource {
    resource_name: String,
    resource_description: String,
    value: Value,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Value {
    // value_type: Type,
    access_mode: AccessMode,
    minimum: String,
    maximum: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum Type {
    STRING(String),
    INT(i32),
    DOUBLE(f32),
}

// impl<'de> Deserialize<'de> for Type {
//     fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
//         where D: Deserializer<'de>
//     {
//         let s = String::deserialize(deserializer)?;
//         if s == "INT" {
//             Ok(Type::INT(0))
//         } else {
//             Ok(Type::INT(0))
//         }
//     }
// }

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
