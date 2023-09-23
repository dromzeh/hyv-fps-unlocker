use inquire::InquireError;
use inquire::Select;
use inquire::Text;
use serde_json::json;
use serde_json::Value;
use std::error::Error;
use std::result::Result;
use std::string::String;

/// Prints the current FPS settings for a specified game.
pub fn print_current_values(game: &str, json_value: &Value) {
    match game {
        "hsr" => println!("Current FPS Value: {}", json_value["FPS"]),
        "hi3" => println!("Current FPS: {}", json_value["TargetFrameRateForInLevel"]),
        _ => panic!("Invalid game selection"),
    }
}

/// Prompts the user for a new FPS value and returns a new JSON value with the updated FPS value.
pub fn get_new_fps_settings(game: &str, json_value: &mut Value) -> Result<Value, Box<dyn Error>> {
    let input: String = if game != "hsr" {
        let fps_options = Text::new("What FPS to set?").prompt();
        match fps_options {
            Ok(g) => g,
            Err(e) => panic!("Error: {}", e),
        }
    } else {
        let fps_options: Vec<&str> = vec!["30", "60", "120"];
        let fps_selection: Result<&str, InquireError> =
            Select::new("What FPS to set?", fps_options).prompt();

        match fps_selection {
            Ok(g) => g.to_string(),
            Err(e) => panic!("Error: {}", e),
        }
    };

    let fps: u32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("No input detected, setting FPS to 120");
            120
        }
    };

    match game {
        "hsr" => {
            json_value["FPS"] = json!(fps);
        }
        "hi3" => {
            json_value["TargetFrameRateForInLevel"] = json!(fps);
            json_value["TargetFrameRateForOthers"] = json!(fps);
        }
        _ => panic!("Invalid game selection"),
    }

    Ok(json_value.clone())
}
