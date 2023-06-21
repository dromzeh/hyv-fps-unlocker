use std::error::Error;
use std::io;
use std::result::Result;
use std::string::String;

pub fn get_game_selection() -> Result<String, Box<dyn Error>> {
    let mut input = String::new();
    println!("Modify FPS for which game? (hsr/hi3):");
    io::stdin().read_line(&mut input)?;
    let game = input.trim().to_lowercase();
    input.clear();

    match game.as_str() {
        "hi3" | "hsr" => Ok(game),
        _ => {
            println!("Invalid game selection");
            Err("Invalid game selection".into())
        }
    }
}
