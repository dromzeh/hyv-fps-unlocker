use crate::errors::{FpsUnlockerError, Result};
use serde_json::Value;

#[derive(Debug, Clone)]
pub struct GameConfig {
    pub name: &'static str,
    pub registry_paths: Vec<RegistryPath>,
    pub fps_field: &'static str,
    pub additional_fps_fields: Vec<&'static str>,
    pub supported_fps_values: Option<Vec<u32>>,
}

#[derive(Debug, Clone)]
pub struct RegistryPath {
    pub path: &'static str,
    pub value_patterns: Vec<&'static str>,
}

impl GameConfig {
    pub fn get_games() -> Vec<GameConfig> {
        vec![
            GameConfig {
                name: "Honkai: Star Rail",
                registry_paths: vec![RegistryPath {
                    path: "Software\\Cognosphere\\Star Rail",
                    value_patterns: vec!["GraphicsSettings_Model_h", "GraphicsSettings_Model"],
                }],
                fps_field: "FPS",
                additional_fps_fields: vec![],
                supported_fps_values: Some(vec![30, 60, 120]),
            },
            GameConfig {
                name: "Honkai Impact 3rd",
                registry_paths: vec![RegistryPath {
                    path: "Software\\miHoYo\\Honkai Impact 3rd",
                    value_patterns: vec![
                        "PersonalGraphicsSettingV2_h",
                        "PersonalGraphicsSettingV2",
                    ],
                }],
                fps_field: "TargetFrameRateForInLevel",
                additional_fps_fields: vec!["TargetFrameRateForOthers"],
                supported_fps_values: None,
            },
        ]
    }

    pub fn get_current_fps(&self, json_value: &Value) -> Option<u64> {
        json_value.get(self.fps_field)?.as_u64()
    }

    pub fn set_fps(&self, json_value: &mut Value, fps: u32) -> Result<()> {
        if let Some(supported_fps) = &self.supported_fps_values {
            if !supported_fps.contains(&fps) {
                return Err(FpsUnlockerError::InvalidFpsValue(format!(
                    "Supported FPS values for {}: {:?}",
                    self.name, supported_fps
                )));
            }
        }

        json_value[self.fps_field] = serde_json::Value::Number(serde_json::Number::from(fps));

        for field in &self.additional_fps_fields {
            json_value[field] = serde_json::Value::Number(serde_json::Number::from(fps));
        }

        Ok(())
    }
}
