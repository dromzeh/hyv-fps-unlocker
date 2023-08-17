use serde_json::Value;
use std::io;
use std::string::String;

pub fn print_success_message(game: &str, json_value: &Value) {
    match game {
        "hsr" => println!("FPS set to {}", json_value["FPS"]),
        "hi3" => println!("FPS set to {}", json_value["TargetFrameRateForInLevel"]),
        _ => panic!("Invalid game selection"),
    }
}

pub fn wait_for_user_input() {
    let mut input = String::new();
    println!("Press any key to exit");
    io::stdin().read_line(&mut input).unwrap();
}
