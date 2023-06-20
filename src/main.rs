extern crate winreg;
use serde_json::json;
use serde_json::Value;
use std::error::Error;
use std::io;
use std::result::Result;
use std::string::String;
use std::vec::Vec;
use winreg::enums::*;
use winreg::RegKey;
use winreg::RegValue;

fn main() -> Result<(), Box<dyn Error>> {
    // get raw_value from registry
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let hi3_regkey =
        hkcu.open_subkey_with_flags("Software\\miHoYo\\Honkai Impact 3rd", KEY_ALL_ACCESS)?;
    let values = hi3_regkey
        .enum_values()
        .map(|x| x.unwrap().0)
        .collect::<Vec<_>>();
    let value_name = values
        .iter()
        .find(|&x| x.contains("PersonalGraphicsSettingV2"))
        .unwrap();
    println!("Found PersonalGraphicsSettingV2 at {:?}", value_name);
    let raw_value = hi3_regkey.get_raw_value(value_name)?;

    // println!("{:?}", raw_value.bytes);

    // parse raw_value binary data into json, then print it
    let json_value: Value = serde_json::from_slice(&raw_value.bytes)?;
    // println!("{:?}", json_value);

    // ask user for input, to modify TargetFrameRateForInLevel & TargetFrameRateForOthers
    let mut input = String::new();
    println!("Enter desired FPS in level: ");
    io::stdin().read_line(&mut input)?;
    let target_frame_rate_for_in_level: u32 = input.trim().parse()?;
    input.clear();
    println!("");
    println!("Enter desired FPS out of level: ");
    io::stdin().read_line(&mut input)?;
    let target_frame_rate_for_others: u32 = input.trim().parse()?;
    input.clear();

    // modify json_value
    let mut json_value = json_value;
    json_value["TargetFrameRateForInLevel"] = json!(target_frame_rate_for_in_level);
    json_value["TargetFrameRateForOthers"] = json!(target_frame_rate_for_others);
    // println!("{:?}", json_value);

    // convert json_value back to binary data
    let new_raw_value = RegValue {
        bytes: serde_json::to_vec(&json_value)?,
        vtype: raw_value.vtype,
    };

    // write new_raw_value to registry
    hi3_regkey.set_raw_value(value_name, &new_raw_value)?;

    // print new_raw_value
    // let new_raw_value = hi3_regkey.get_raw_value(value_name)?;
    // println!("{:?}", new_raw_value.bytes);

    println!("");
    println!(
        "Updated FPS settings to {} in level, {} out of level",
        target_frame_rate_for_in_level, target_frame_rate_for_others
    );
    println!("Press any key to exit");

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(())
}
