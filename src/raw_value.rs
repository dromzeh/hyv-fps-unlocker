use serde_json::Value;
use std::error::Error;
use std::result::Result;
use winreg::enums::*;
use winreg::{RegKey, RegValue};

/// Parses a raw value into a JSON value.
pub fn parse_raw_value(raw_value: &RegValue) -> Result<Value, Box<dyn Error>> {
    let json_value: Value = serde_json::from_slice(&raw_value.bytes)?;
    Ok(json_value)
}

/// Writes a new raw value to the registry.
pub fn write_new_raw_value(
    reg_key_path: &str,
    value_name_contains: &str,
    new_raw_value: &RegValue,
) -> Result<(), Box<dyn Error>> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let reg_key = hkcu.open_subkey_with_flags(reg_key_path, KEY_ALL_ACCESS)?;
    let value_name = reg_key
        .enum_values()
        .filter_map(|x| x.ok().map(|(name, _)| name))
        .find(|name| name.contains(value_name_contains))
        .ok_or_else(|| format!("Value {} not found", value_name_contains))?;
    reg_key.set_raw_value(value_name, new_raw_value)?;
    Ok(())
}

/// Parses a raw value into a JSON value, and attempts to clean the raw value if parsing fails.
pub fn parse_and_clean_raw_value(raw_value: &RegValue) -> Result<Value, Box<dyn Error>> {
    match parse_raw_value(raw_value) {
        Ok(value) => Ok(value),
        Err(_) => {
            println!("Failed to parse raw value, attempting to clean and parse again.");
            match parse_raw_value(&clean_raw_value(raw_value)) {
                Ok(value) => Ok(value),
                Err(_) => {
                    Err("Failed to parse raw value after attempting to clean. You might want to check the registry value manually. If you are sure the value is correct, please open a GitHub issue.".into())
                }
            }
        }
    }
}

/// Removes all null bytes from a raw value to "clean" it if parsing fails.
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
