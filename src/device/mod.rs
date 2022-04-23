use crate::device::device_profile::DeviceProfile;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::fs::File;

pub mod device_profile;
pub mod device_profile_impl;

lazy_static! {
    pub static ref DEVICE_PROFILES: HashMap<String, DeviceProfile> = parse_profiles();
}

pub fn parse_profiles() -> HashMap<String, DeviceProfile> {
    let mut profiles = HashMap::<String, DeviceProfile>::new();
    let profile_json = File::open("config/profile.json").expect("Profile config file open error!");
    let profile: DeviceProfile = serde_json::from_reader(profile_json).unwrap();
    profiles.insert(profile.profile_name.clone(), profile);
    profiles
}

pub fn get_profiles(profile_name: String) -> &'static DeviceProfile {
    DEVICE_PROFILES.get::<String>(&profile_name).unwrap()
}

#[cfg(test)]
mod test {
    use crate::device::device_profile::DeviceList;
    use std::fs::File;
    use std::io::Read;
    use std::string::String;
    use crate::device::parse_profiles;

    #[test]
    fn parse_device_test() {
        parse_profiles();
        let device_json =
            File::open("config/device.json").expect("Device config file open error!");
        let u: DeviceList = serde_json::from_reader(device_json).unwrap();
        println!("{:?}", u);

        let mut device_toml =
            File::open("config/device.toml").expect("Device config file open error!");
        let mut toml_string = String::new();
        device_toml.read_to_string(&mut toml_string);
        let u: DeviceList = toml::from_slice(toml_string.as_bytes()).unwrap();
        println!("{:?}", u);

        let device_yaml =
            File::open("config/device.yaml").expect("Device config file open error!");
        let u: DeviceList = serde_yaml::from_reader(device_yaml).unwrap();
        println!("{:?}", u);
    }
}
