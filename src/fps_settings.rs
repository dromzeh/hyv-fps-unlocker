use crate::errors::{FpsUnlockerError, Result};
use crate::game_config::GameConfig;
use inquire::{Select, Text};
use serde_json::Value;

pub fn print_current_values(game_config: &GameConfig, json_value: &Value) {
    if let Some(fps) = game_config.get_current_fps(json_value) {
        println!("Current FPS for {}: {}\n", game_config.name, fps);
    } else {
        println!("Current FPS for {}: Unable to read\n", game_config.name);
    }
}

pub fn get_new_fps_settings(game_config: &GameConfig, json_value: &mut Value) -> Result<Value> {
    let fps = if let Some(supported_fps) = &game_config.supported_fps_values {
        let fps_options: Vec<String> = supported_fps.iter().map(|f| f.to_string()).collect();
        let fps_selection = Select::new(
            &format!("What FPS to set for {}?", game_config.name),
            fps_options,
        )
        .prompt()
        .map_err(|e| FpsUnlockerError::UserInputError(format!("FPS selection failed: {}", e)))?;

        fps_selection
            .parse::<u32>()
            .map_err(|e| FpsUnlockerError::InvalidFpsValue(format!("Parse error: {}", e)))?
    } else {
        let fps_input = Text::new(&format!("What FPS to set for {}?", game_config.name))
            .with_help_message("Enter a number (e.g., 144 for 144 FPS)")
            .prompt()
            .map_err(|e| FpsUnlockerError::UserInputError(format!("FPS input failed: {}", e)))?;

        if fps_input.trim().is_empty() {
            120
        } else {
            fps_input.trim().parse::<u32>().map_err(|_| {
                FpsUnlockerError::InvalidFpsValue(format!(
                    "Invalid FPS value: '{}'. Please enter a valid number.",
                    fps_input
                ))
            })?
        }
    };

    if fps < 10 || fps > 1000 {
        return Err(FpsUnlockerError::InvalidFpsValue(format!(
            "FPS value {} is out of reasonable range (10-1000)",
            fps
        )));
    }

    game_config.set_fps(json_value, fps)?;

    Ok(json_value.clone())
}
