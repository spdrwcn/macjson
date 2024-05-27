use redis::{Client, Commands};
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use serde::Deserialize;
use serde::Serialize;
use serde_json::to_string_pretty;
use clap::{App as ClapApp, Arg};
use std::time::Instant;
#[derive(Serialize, Deserialize, Debug)]
struct DeviceInfo {
    bluetooth_mac: String,
    wired_mac: String,
    wireless_mac: String,
}

fn get_matches() -> clap::ArgMatches<'static> {
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
    matches
}

fn get_client(ip_address: &str) -> Result<Client, Box<dyn std::error::Error>> {
    let client = Client::open(ip_address)?;
    Ok(client)
}

fn get_keys(con: &mut redis::Connection) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let keys: Vec<String> = con.keys("*")?;
    Ok(keys)
}

fn get_device_infos(con: &mut redis::Connection, keys: &[String]) -> Result<HashMap<String, DeviceInfo>, Box<dyn std::error::Error>> {
    let mut key_value_pairs: HashMap<String, DeviceInfo> = HashMap::new();
    let mut pipe = redis::pipe();

    for key in keys {
        pipe.get(key);
    }

    let values: Vec<String> = pipe.query(con)?;

    for (key, value) in keys.iter().zip(values) {
        let device_info: DeviceInfo = serde_json::from_str(&value)?;
        key_value_pairs.insert(key.to_owned(), device_info);
    }

    Ok(key_value_pairs)
}

fn write_to_file(json_string: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::create("mac.json")?;
    file.write_all(json_string.as_bytes())?;
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = get_matches();
    let ip_address = matches.value_of("ip").unwrap();
    let client = get_client(ip_address)?;
    let mut con = client.get_connection()?;
    let start_time = Instant::now();
    let keys = get_keys(&mut con)?;
    let key_value_pairs = get_device_infos(&mut con, &keys)?;
    let json_string = to_string_pretty(&key_value_pairs)?;
    write_to_file(&json_string)?;
    let elapsed_time = start_time.elapsed();
    println!("Execution time: {:?}", elapsed_time);
    println!("Press Enter to exit...");  
    std::io::stdin().read_line(&mut String::new()).unwrap();  
    Ok(())
}