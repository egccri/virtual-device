use crate::device::device_profile::{DeviceList, DeviceProfile};
use std::collections::HashMap;
use std::fs::File;
use lazy_static::lazy_static;

pub mod device_profile;
pub mod device_profile_impl;

lazy_static!(
    pub static ref DEVICE_PROFILES: HashMap<&'static str, DeviceProfile> = parse_profiles();
);


pub struct Context {
    device_list: DeviceList,
    device_profile: DeviceProfile,
}

pub fn parse_profiles() -> HashMap<&'static str, DeviceProfile> {
    let mut profiles = HashMap::<&'static str, DeviceProfile>::new();
    let profile_json = File::open("config/profile.json").expect("Profile config file open error!");
    let profile: DeviceProfile = serde_json::from_reader(profile_json).unwrap();
    println!("{:?}", profile);
    profiles.insert("crane", profile);
    profiles
}

mod test {
    use std::fs::File;
    use crate::device::device_profile::DeviceList;
    use serde_json::Value as Json;
    use toml::Value as Toml;

    #[test]
    fn profile_to_toml() {
        let device_json = File::open("config/device.json").expect("Device config file open error!");
        let u: DeviceList = serde_json::from_reader(device_json).unwrap();
        println!("{:?}", u);
        let t = toml::to_string(&u).unwrap();
        println!("{}", t);
        let y = serde_yaml::to_string(&u).unwrap();
        println!("{}", y);
    }

    fn convert(toml: Toml) -> Json {
        match toml {
            Toml::String(s) => Json::String(s),
            Toml::Integer(i) => Json::Number(i.into()),
            Toml::Float(f) => {
                let n = serde_json::Number::from_f64(f).expect("float infinite and nan not allowed");
                Json::Number(n)
            }
            Toml::Boolean(b) => Json::Bool(b),
            Toml::Array(arr) => Json::Array(arr.into_iter().map(convert).collect()),
            Toml::Table(table) => {
                Json::Object(table.into_iter().map(|(k, v)| (k, convert(v))).collect())
            }
            Toml::Datetime(dt) => Json::String(dt.to_string()),
        }
    }
}
