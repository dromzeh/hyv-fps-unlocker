use std::error::Error;
use std::result::Result;
use std::string::String;
use winreg::enums::*;
use winreg::RegKey;

pub fn get_registry_info(game: &str) -> Result<(String, String), Box<dyn Error>> {
    match game {
        "hi3" => Ok((
            "Software\\miHoYo\\Honkai Impact 3rd".to_string(),
            "PersonalGraphicsSettingV2".to_string(),
        )),
        "hsr" => Ok((
            "Software\\Cognosphere\\Star Rail".to_string(),
            "GraphicsSettings_Model".to_string(),
        )),
        _ => Err("Invalid game selection".into()),
    }
}

pub fn get_raw_value(
    reg_key_path: &str,
    value_name_contains: &str,
) -> Result<winreg::RegValue, Box<dyn Error>> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let reg_key = hkcu.open_subkey_with_flags(reg_key_path, KEY_ALL_ACCESS)?;
    let values = reg_key
        .enum_values()
        .map(|x| x.unwrap().0)
        .collect::<Vec<_>>();
    let value_name = values
        .iter()
        .find(|&x| x.contains(value_name_contains))
        .ok_or_else(|| format!("Value {} not found", value_name_contains))?;
    println!("Found {} at {:?} \n", value_name_contains, value_name);
    reg_key
        .get_raw_value(value_name)
        .map_err(|e| format!("Failed to get raw value: {}", e).into())
}
