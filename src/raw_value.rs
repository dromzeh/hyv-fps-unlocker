use serde_json::Value;
use std::error::Error;
use std::result::Result;
use winreg::enums::*;
use winreg::{RegKey, RegValue};

pub fn parse_raw_value(raw_value: &RegValue) -> Result<Value, Box<dyn Error>> {
    let json_value: Value = serde_json::from_slice(&raw_value.bytes)?;
    Ok(json_value)
}

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
