use crate::errors::{FpsUnlockerError, Result};
use crate::game_config::{GameConfig, RegistryPath};
use winreg::enums::*;
use winreg::{RegKey, RegValue};

pub struct FoundRegistry {
    pub path: String,
    pub value_name: String,
}

pub fn detect_installed_games() -> Vec<GameConfig> {
    let all_games = GameConfig::get_games();
    let mut installed_games = Vec::new();
    let mut not_detected_games = Vec::new();

    println!("🔍 Detecting installed games...");

    for game in all_games {
        if is_game_installed(&game) {
            println!("  ✓ {} found", game.name);
            installed_games.push(game);
        } else {
            println!("  ✗ {} not detected", game.name);
            not_detected_games.push(game.name);
        }
    }

    if installed_games.is_empty() {
        println!("  ✗ No supported games found");
    } else if !not_detected_games.is_empty() {
        println!(
            "\n💡 Note: If you have {} installed but it's not detected,",
            not_detected_games.join(" or ")
        );
        println!("   try opening the game's graphics settings first, change a value, then save, to generate registry values.");
    }

    println!();
    installed_games
}

fn is_game_installed(game_config: &GameConfig) -> bool {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);

    for registry_path in &game_config.registry_paths {
        if try_registry_path(&hkcu, registry_path).is_ok() {
            return true;
        }
    }
    false
}

pub fn find_registry_info(game_config: &GameConfig) -> Result<FoundRegistry> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);

    for registry_path in &game_config.registry_paths {
        if let Ok(found) = try_registry_path(&hkcu, registry_path) {
            return Ok(FoundRegistry {
                path: registry_path.path.to_string(),
                value_name: found,
            });
        }
    }

    let attempted_paths: Vec<String> = game_config
        .registry_paths
        .iter()
        .map(|p| format!("  - {}", p.path))
        .collect();

    Err(FpsUnlockerError::RegistryKeyNotFound(format!(
        "Could not find registry information for {}.\n\nAttempted paths:\n{}",
        game_config.name,
        attempted_paths.join("\n")
    )))
}

fn try_registry_path(hkcu: &RegKey, registry_path: &RegistryPath) -> Result<String> {
    let reg_key = hkcu
        .open_subkey_with_flags(registry_path.path, KEY_READ)
        .map_err(|e| match e.kind() {
            std::io::ErrorKind::NotFound => FpsUnlockerError::RegistryKeyNotFound(format!(
                "Registry path '{}' not found",
                registry_path.path
            )),
            std::io::ErrorKind::PermissionDenied => FpsUnlockerError::RegistryAccessDenied(
                format!("Access denied to registry path '{}'", registry_path.path),
            ),
            _ => FpsUnlockerError::WinregError(e),
        })?;

    let available_values: Vec<String> = reg_key
        .enum_values()
        .filter_map(|result| result.ok().map(|(name, _)| name))
        .collect();

    for pattern in &registry_path.value_patterns {
        if let Some(found_value) = find_matching_value(&available_values, pattern) {
            return Ok(found_value);
        }
    }

    Err(FpsUnlockerError::RegistryValueNotFound(format!(
        "None of the expected patterns {:?} found in {}. Available values: {:?}",
        registry_path.value_patterns, registry_path.path, available_values
    )))
}

fn find_matching_value(available_values: &[String], pattern: &str) -> Option<String> {
    for value in available_values {
        let value_lower = value.to_lowercase();
        let pattern_lower = pattern.to_lowercase();

        if matches_pattern(&value_lower, &pattern_lower) {
            return Some(value.clone());
        }
    }
    None
}

fn matches_pattern(value: &str, pattern: &str) -> bool {
    if value == pattern {
        return true;
    }

    if value.starts_with(pattern) {
        return true;
    }

    if let Some(base_pattern) = pattern.strip_suffix("_h") {
        if let Some(pattern_index) = value.find(base_pattern) {
            let after_pattern = &value[pattern_index + base_pattern.len()..];
            if let Some(after_h) = after_pattern.strip_prefix("_h") {
                if after_h.chars().all(|c| c.is_ascii_digit()) {
                    return true;
                }
            }
        }
    }

    if value.contains(pattern) && pattern.len() > 5 {
        return true;
    }

    false
}

pub fn get_raw_value(registry_info: &FoundRegistry) -> Result<RegValue> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let reg_key = hkcu
        .open_subkey_with_flags(&registry_info.path, KEY_READ)
        .map_err(FpsUnlockerError::WinregError)?;

    reg_key
        .get_raw_value(&registry_info.value_name)
        .map_err(FpsUnlockerError::WinregError)
}

pub fn write_raw_value(registry_info: &FoundRegistry, new_raw_value: &RegValue) -> Result<()> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let reg_key = hkcu
        .open_subkey_with_flags(&registry_info.path, KEY_SET_VALUE)
        .map_err(|e| match e.kind() {
            std::io::ErrorKind::PermissionDenied => FpsUnlockerError::RegistryAccessDenied(
                "Permission denied writing to registry. Try running as administrator.".to_string(),
            ),
            _ => FpsUnlockerError::WinregError(e),
        })?;

    reg_key
        .set_raw_value(&registry_info.value_name, new_raw_value)
        .map_err(FpsUnlockerError::WinregError)
}
