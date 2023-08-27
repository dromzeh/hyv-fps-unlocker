extern crate winreg;
use std::error::Error;
use winreg::RegValue;
use std::result::Result;
mod fps_settings;
mod game_selection;
mod message;
mod raw_value;
mod registry_info;

fn main() {
    if let Err(err) = run_program() {
        eprintln!(
            "An error occurred, please open a GitHub issue if this was unexpected: {}",
            err
        );
        message::wait_for_user_input();
    }
}

fn run_program() -> Result<(), Box<dyn Error>> {
    println!("hyv-fps-unlocker @ git.dromzeh.dev/hyv-fps-unlocker");
    println!("This program is provided as-is, without any warranty. Use at your own risk.");
    println!(
        "Do not use this program if you are not comfortable with modifying your registry files."
    );
    println!("Modifying registry values while the game is running will prevent the program from working as intended.\n");
    let game = game_selection::get_game_selection()?;
    let (reg_key_path, value_name_contains) = registry_info::get_registry_info(&game)?;
    let raw_value = registry_info::get_raw_value(&reg_key_path, &value_name_contains)?;
    let mut json_value = match raw_value::parse_raw_value(&raw_value) {
        Ok(value) => value,
        Err(_) => {
            println!("Failed to parse value, attempting to clean value...");
            match raw_value::parse_raw_value(&clean_raw_value(&raw_value)) {
                Ok(value) => value,
                Err(_) => {
                    return Err("Failed to parse raw value after cleaning".into());
                }
            }
        }
    };
    fps_settings::print_current_values(&game, &json_value);
    let new_json_value = fps_settings::get_new_fps_settings(&game, &mut json_value)?;
    let new_raw_value = winreg::RegValue {
        bytes: serde_json::to_vec(&new_json_value)?,
        vtype: raw_value.vtype,
    };
    raw_value::write_new_raw_value(&reg_key_path, &value_name_contains, &new_raw_value)?;
    message::print_success_message(&game, &new_json_value);
    message::wait_for_user_input();

    Ok(())
}

// remove null bytes from raw value
fn clean_raw_value(raw_value: &RegValue) -> RegValue {
    let mut bytes = raw_value.bytes.clone();
    let mut new_bytes = Vec::new();
    for byte in bytes.iter_mut() {
        if *byte != 00 {
            new_bytes.push(*byte);
        }
    }
    RegValue {
        bytes: new_bytes,
        vtype: raw_value.vtype.clone(),
    }
}