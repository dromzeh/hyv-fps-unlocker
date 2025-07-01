use crate::errors::{FpsUnlockerError, Result};
use crate::game_config::GameConfig;
use crate::registry_info;
use inquire::Select;

pub fn get_game_selection() -> Result<GameConfig> {
    let installed_games = registry_info::detect_installed_games();

    if installed_games.is_empty() {
        return Err(FpsUnlockerError::GameNotSupported(
            "No supported games found on this system.\n\nSupported games:\n- Honkai: Star Rail\n- Honkai Impact 3rd\n\nThis could happen if:\n1. The games are not installed\n2. The games haven't been launched yet\n3. You haven't opened graphics settings in-game yet (required to generate registry values)\n\nSolution: Launch your game, go to Settings > Graphics, change any setting (like resolution), apply, then try this program again.".to_string()
        ));
    }

    if installed_games.len() == 1 {
        let game = &installed_games[0];
        println!("✓ Only one game detected: {}", game.name);
        println!("Automatically selecting {}...\n", game.name);
        return Ok(game.clone());
    }

    println!("Found {} installed games:", installed_games.len());
    for game in &installed_games {
        println!("  • {}", game.name);
    }
    println!();

    let game_names: Vec<&str> = installed_games.iter().map(|g| g.name).collect();

    let game_selection = Select::new("Which game would you like to modify?", game_names)
        .prompt()
        .map_err(|e| FpsUnlockerError::UserInputError(format!("Game selection failed: {}", e)))?;

    installed_games
        .into_iter()
        .find(|game| game.name == game_selection)
        .ok_or_else(|| FpsUnlockerError::GameNotSupported(game_selection.to_string()))
}
