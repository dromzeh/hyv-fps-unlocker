extern crate winreg;

mod errors;
mod fps_settings;
mod game_config;
mod game_selection;
mod message;
mod raw_value;
mod registry_info;

use errors::Result;

fn main() {
    println!("🎮 hyv-fps-unlocker @ git.dromzeh.dev/hyv-fps-unlocker");
    println!("This program is provided as-is, without any warranty. Use at your own risk.");
    println!(
        "Do not use this program if you are not comfortable with modifying your registry files."
    );
    println!("\nIMPORTANT: Please close your games before running this program!");
    println!("Modifying registry values while the game is running will prevent the program from working as intended.\n");

    if let Err(err) = run_program() {
        eprintln!("\n❌ Error: {}", err);
        eprintln!("\nIf this error persists, please:");
        eprintln!("1. Make sure the game is installed and has been launched at least once");
        eprintln!("2. Try running this program as administrator");
        eprintln!("3. Check if the game is running (close it if it is)");
        eprintln!("4. Open a GitHub issue if the problem continues");
        message::wait_for_user_input();
    }
}

fn run_program() -> Result<()> {
    // Auto-detect installed games and get selection
    let game_config = game_selection::get_game_selection()?;

    // Find registry information
    let registry_info = registry_info::find_registry_info(&game_config)?;

    // Read and parse current settings
    let raw_value = registry_info::get_raw_value(&registry_info)?;
    let mut json_value = raw_value::parse_raw_value(&raw_value)?;

    // Show current FPS and get new settings
    fps_settings::print_current_values(&game_config, &json_value);
    let new_json_value = fps_settings::get_new_fps_settings(&game_config, &mut json_value)?;

    // Save new settings
    let new_raw_value = raw_value::create_raw_value_from_json(&new_json_value, &raw_value)?;
    registry_info::write_raw_value(&registry_info, &new_raw_value)?;

    // Confirm success
    message::print_success_message(&game_config, &new_json_value);

    // Important notes
    println!("\n📝 Important notes:");
    println!("- Changes will take effect the next time you start the game");
    println!("- If you change graphics settings in-game, you'll need to rerun this program");
    println!("- If you experience issues, you can reset graphics settings in-game");

    message::wait_for_user_input();
    Ok(())
}
