use std::env;
use rayon::prelude::*;
use redis::{Client, Commands};
use serde::Deserialize;
use serde::Serialize;
use serde_json::to_string_pretty;
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::time::Instant;

#[derive(Serialize, Deserialize, Debug)]
struct DeviceInfo {
    bluetooth_mac: String,
    wired_mac: String,
    wireless_mac: String,
}

fn get_client(ip_address: &str) -> Result<Client, Box<dyn std::error::Error>> {
    let client = Client::open(ip_address)?;
    Ok(client)
}

fn get_keys(con: &mut redis::Connection) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let keys: Vec<String> = con.keys("*")?;
    Ok(keys)
}

fn get_device_infos(
    con: &mut redis::Connection,
    keys: &[String],
) -> Result<HashMap<String, DeviceInfo>, Box<dyn std::error::Error>> {
    let mut key_value_pairs: HashMap<String, DeviceInfo> = HashMap::new();
    let mut pipe = redis::pipe();

    for key in keys {
        pipe.get(key);
    }

    let values: Vec<String> = pipe.query(con)?;

    let results: Vec<(String, DeviceInfo)> = keys
        .par_iter()
        .zip(values.par_iter())
        .map(|(key, value)| {
            let device_info: DeviceInfo = serde_json::from_str(&value).unwrap();
            (key.to_owned(), device_info)
        })
        .collect();

    for (key, value) in results {
        key_value_pairs.insert(key, value);
    }

    Ok(key_value_pairs)
}

fn write_to_file(json_string: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::create("mac.json")?;
    file.write_all(json_string.as_bytes())?;
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let ip_address = if args.len() > 1 {
        &args[1]
    } else {
        "redis://127.0.0.1:6379/0"
    };
    let start_time = Instant::now();
    let client = get_client(ip_address)?;
    let mut con = client.get_connection()?;
    let keys = get_keys(&mut con)?;
    let key_value_pairs = get_device_infos(&mut con, &keys)?;
    let json_string = to_string_pretty(&key_value_pairs)?;
    write_to_file(&json_string)?;
    let elapsed_time = start_time.elapsed();
    println!("获取数据: {}组", keys.len());
    println!("用时: {:?}", elapsed_time);
    Ok(())
}
