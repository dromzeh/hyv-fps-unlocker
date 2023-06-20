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
    // ask user which game to update FPS settings for
    let mut input = String::new();
    println!("Modify FPS for which game? (hsr/hi3):");
    io::stdin().read_line(&mut input)?;
    let game = input.trim().to_lowercase();
    input.clear();

    // get registry key and value name based on selected game
    let (reg_key_path, value_name_contains) = match game.as_str() {
        "hi3" => (
            "Software\\miHoYo\\Honkai Impact 3rd",
            "PersonalGraphicsSettingV2",
        ),
        "hsr" => ("Software\\Cognosphere\\Star Rail", "GraphicsSettings_Model"),
        _ => {
            println!("Invalid game selection");
            return Ok(());
        }
    };

    // get raw_value from registry
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let reg_key = hkcu.open_subkey_with_flags(reg_key_path, KEY_ALL_ACCESS)?;
    let values = reg_key
        .enum_values()
        .map(|x| x.unwrap().0)
        .collect::<Vec<_>>();
    let value_name = values
        .iter()
        .find(|&x| x.contains(value_name_contains))
        .unwrap();
    println!("Found {} at {:?} \n", value_name_contains, value_name);
    let raw_value = reg_key.get_raw_value(value_name)?;

    // parse raw_value binary data into json, then print it
    let mut json_value: Value = serde_json::from_slice(&raw_value.bytes)?;

    // print json_value
    println!("Current Values for {}:\n", game);
    if game == "hsr" {
        println!("FPS: {}", json_value["FPS"]);
    } else {
        println!("In Level: {}", json_value["TargetFrameRateForInLevel"]);
        println!("Out of Level: {}", json_value["TargetFrameRateForOthers"]);
    }

    // ask user for new FPS settings
    if game == "hsr" {
        println!("\nEnter desired FPS (30/60/120): ");
        io::stdin().read_line(&mut input)?;
        let fps: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("No input detected, setting FPS to 120");
                120
            }
        };
        input.clear();

        json_value["FPS"] = json!(fps);
    } else {
        println!("\nEnter desired FPS in level: ");
        io::stdin().read_line(&mut input)?;
        let target_frame_rate_for_in_level: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("No input detected, setting FPS to 120");
                120
            }
        };
        input.clear();
        println!("Enter desired FPS out of level: ");
        io::stdin().read_line(&mut input)?;
        let target_frame_rate_for_others: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("No input detected, setting FPS to 120");
                120
            }
        };
        input.clear();

        json_value["TargetFrameRateForInLevel"] = json!(target_frame_rate_for_in_level);
        json_value["TargetFrameRateForOthers"] = json!(target_frame_rate_for_others);
    }

    // convert json_value back to binary data
    let new_raw_value = RegValue {
        bytes: serde_json::to_vec(&json_value)?,
        vtype: raw_value.vtype,
    };

    // write new_raw_value to registry
    reg_key.set_raw_value(value_name, &new_raw_value)?;

    if game == "hsr" {
        println!("\nFPS set to {}", json_value["FPS"]);
    } else {
        println!(
            "\nIn Level FPS set to {}",
            json_value["TargetFrameRateForInLevel"]
        );
        println!(
            "Out of Level FPS set to {}",
            json_value["TargetFrameRateForOthers"]
        );
    }

    println!("\nPress any key to exit");

    io::stdin().read_line(&mut input)?;
    Ok(())
}
