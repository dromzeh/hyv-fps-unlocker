use inquire::InquireError;
use inquire::Select;
use std::error::Error;
use std::result::Result;
use std::string::String;

/// Prompts the user for a game selection and returns the game abbreviation.
pub fn get_game_selection() -> Result<String, Box<dyn Error>> {
    let games: Vec<&str> = vec!["Honkai: Star Rail", "Honkai Impact 3rd"];
    let game_selection: Result<&str, InquireError> =
        Select::new("What game to modify?", games).prompt();

    let full_game_name = match game_selection {
        Ok(g) => g,
        Err(e) => panic!("Error: {}", e),
    };

    let game = match full_game_name {
        "Honkai: Star Rail" => "hsr",
        "Honkai Impact 3rd" => "hi3",
        _ => panic!("Invalid game selection"),
    };

    Ok(game.to_string())
}
