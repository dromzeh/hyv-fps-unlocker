use crate::game_config::GameConfig;
use serde_json::Value;
use std::io;
use std::string::String;

pub fn print_success_message(game_config: &GameConfig, json_value: &Value) {
    if let Some(fps) = game_config.get_current_fps(json_value) {
        println!(
            "\n✓ Successfully set FPS to {} for {}",
            fps, game_config.name
        );

        if !game_config.additional_fps_fields.is_empty() {
            println!("  Additional fields updated:");
            for field in &game_config.additional_fps_fields {
                if let Some(value) = json_value.get(field) {
                    println!("  - {}: {}", field, value);
                }
            }
        }
    } else {
        println!("✓ FPS settings have been updated for {}", game_config.name);
    }
}

pub fn wait_for_user_input() {
    let mut input = String::new();
    println!("\nPress Enter to exit...");
    let _ = io::stdin().read_line(&mut input);
}
