use serde_json::Value;
use std::io;
use std::string::String;

pub fn print_success_message(game: &str, json_value: &Value) {
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
}

pub fn wait_for_user_input() {
    let mut input = String::new();
    println!("\nPress any key to exit");
    io::stdin().read_line(&mut input).unwrap();
}
