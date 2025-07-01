use crate::errors::{FpsUnlockerError, Result};
use serde_json::Value;
use winreg::RegValue;

pub fn parse_raw_value(raw_value: &RegValue) -> Result<Value> {
    match serde_json::from_slice(&raw_value.bytes) {
        Ok(value) => Ok(value),
        Err(e) => {
            let cleaned_value = clean_raw_value(raw_value);
            match serde_json::from_slice(&cleaned_value.bytes) {
                Ok(value) => Ok(value),
                Err(e2) => {
                    let ultra_cleaned = ultra_clean_raw_value(raw_value);
                    match serde_json::from_slice(&ultra_cleaned.bytes) {
                        Ok(value) => Ok(value),
                        Err(_) => {
                            let hex_dump = hex::encode(&raw_value.bytes);
                            Err(FpsUnlockerError::JsonParseError(format!(
                                "Failed to parse JSON even after cleaning attempts.\nOriginal error: {}\nCleaned error: {}\nHex dump (first 100 bytes): {}",
                                e, e2, &hex_dump[..std::cmp::min(200, hex_dump.len())]
                            )))
                        }
                    }
                }
            }
        }
    }
}

fn clean_raw_value(raw_value: &RegValue) -> RegValue {
    let cleaned_bytes: Vec<u8> = raw_value
        .bytes
        .iter()
        .filter(|&&b| b != 0)
        .copied()
        .collect();

    RegValue {
        bytes: cleaned_bytes,
        vtype: raw_value.vtype.clone(),
    }
}

fn ultra_clean_raw_value(raw_value: &RegValue) -> RegValue {
    let cleaned_bytes: Vec<u8> = raw_value
        .bytes
        .iter()
        .filter(|&&b| b >= 32 || b == 9 || b == 10 || b == 13)
        .copied()
        .collect();

    RegValue {
        bytes: cleaned_bytes,
        vtype: raw_value.vtype.clone(),
    }
}

pub fn create_raw_value_from_json(
    json_value: &Value,
    original_raw_value: &RegValue,
) -> Result<RegValue> {
    let json_bytes = serde_json::to_vec(json_value)?;

    Ok(RegValue {
        bytes: json_bytes,
        vtype: original_raw_value.vtype.clone(),
    })
}
