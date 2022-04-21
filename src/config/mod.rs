mod device_profile;


mod test {
    use std::fs::File;
    use crate::config::device_profile::DeviceList;
    use serde_json::Value as Json;
    use toml::Value as Toml;

    #[test]
    fn profile_to_toml() {
        let json_file = File::open("config/device.json").unwrap();
        let u: DeviceList = serde_json::from_reader(json_file).unwrap();
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
