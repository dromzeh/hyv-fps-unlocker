use serde_json::Value;
use std::error::Error;
use std::result::Result;

pub fn parse_raw_value(raw_value: &winreg::RegValue) -> Result<Value, Box<dyn Error>> {
    let json_value: Value = serde_json::from_slice(&raw_value.bytes)?;
    Ok(json_value)
}

pub fn write_new_raw_value(
    reg_key_path: &str,
    value_name_contains: &str,
    new_raw_value: &winreg::RegValue,
) -> Result<(), Box<dyn Error>> {
    let hkcu = winreg::RegKey::predef(winreg::enums::HKEY_CURRENT_USER);
    let reg_key = hkcu.open_subkey_with_flags(reg_key_path, winreg::enums::KEY_ALL_ACCESS)?;
    let values = reg_key
        .enum_values()
        .map(|x| x.unwrap().0)
        .collect::<Vec<_>>();
    let value_name = values
        .iter()
        .find(|&x| x.contains(value_name_contains))
        .unwrap();
    Ok(reg_key.set_raw_value(value_name, new_raw_value)?)
}
