use simple_redis;
use std::collections::HashMap;  
use std::fs::File;  
use std::io::Write;
use serde::Deserialize; 
use serde::Serialize;  
use serde_json::to_string_pretty;  
use clap::{App as ClapApp, Arg};

  
fn main() -> Result<(), Box<dyn std::error::Error>> {  
    let matches = ClapApp::new("macjson")
        .version("1.0.0")
        .author("h13317136163@163.com")
        .about("MAC地址Redis格式化工具")
        .arg(
            Arg::with_name("ip")
                .short("i")
                .long("ip")
                .value_name("IP_ADDRESS")
                .help("Redis数据库地址")
                .default_value("redis://127.0.0.1:6379/0"),
        )
        .get_matches();
    let ip_address = matches.value_of("ip").unwrap();
    let mut redis = simple_redis::create(ip_address)?;
    let keys: Vec<String> = redis.keys("*")?;  
    let mut key_value_pairs: HashMap<String, DeviceInfo> = HashMap::new();   
    for key in keys {  
        let value: String = redis.get(&key)?;
        let device_info: DeviceInfo = serde_json::from_str(&value)?;  
        key_value_pairs.insert(key, device_info);  
    }  
    let json_string = to_string_pretty(&key_value_pairs)?;  
    let mut file = File::create("mac.json")?;  
    file.write_all(json_string.as_bytes())?;  
    Ok(())  
}  

#[derive(Serialize, Deserialize, Debug)]  
struct DeviceInfo {  
    bluetooth_mac: String,  
    wired_mac: String,  
    wireless_mac: String,  
}  