use serde_json::json;
use serde_json::Value;
use std::error::Error;
use std::io;
use std::result::Result;
use std::string::String;

pub fn print_current_values(game: &str, json_value: &Value) {
    println!("Current Values for {}:\n", game);
    if game == "hsr" {
        println!("FPS: {}", json_value["FPS"]);
    } else {
        println!("In Level: {}", json_value["TargetFrameRateForInLevel"]);
        println!("Out of Level: {}", json_value["TargetFrameRateForOthers"]);
    }
}

pub fn get_new_fps_settings(game: &str, json_value: &mut Value) -> Result<Value, Box<dyn Error>> {
    let mut input = String::new();
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

    Ok(json_value.clone())
}
