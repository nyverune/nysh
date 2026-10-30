use directories::ProjectDirs;
use std::fs;

use ratatui::style::Color;
use serde::{Deserialize, Deserializer};

fn deserialize_hex_color<'de, D>(deserializer: D) -> Result<Color, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = String::deserialize(deserializer)?;
    let hex = s.trim_start_matches('#');

    if hex.len() != 6 {
        return Err(serde::de::Error::custom("Color must be in #RRGGBB format"));
    }

    let r = u8::from_str_radix(&hex[0..2], 16).map_err(serde::de::Error::custom)?;
    let g = u8::from_str_radix(&hex[2..4], 16).map_err(serde::de::Error::custom)?;
    let b = u8::from_str_radix(&hex[4..6], 16).map_err(serde::de::Error::custom)?;

    Ok(Color::Rgb(r, g, b))
}

#[derive(Debug, Clone, Deserialize)]
pub struct ThemePalette {
    #[serde(deserialize_with = "deserialize_hex_color")]
    pub bg: Color,

    #[serde(deserialize_with = "deserialize_hex_color")]
    pub fg: Color,

    #[serde(deserialize_with = "deserialize_hex_color")]
    pub accent: Color,

    #[serde(deserialize_with = "deserialize_hex_color")]
    pub secondary: Color,

    #[serde(deserialize_with = "deserialize_hex_color")]
    pub muted: Color,

    #[serde(deserialize_with = "deserialize_hex_color")]
    pub selection: Color,

    #[serde(deserialize_with = "deserialize_hex_color")]
    pub error: Color,

    #[serde(deserialize_with = "deserialize_hex_color")]
    pub warning: Color,

    #[serde(deserialize_with = "deserialize_hex_color")]
    pub success: Color,

    #[serde(deserialize_with = "deserialize_hex_color")]
    pub info: Color,
}

impl Default for ThemePalette {
    fn default() -> Self {
        Self {
            bg: Color::Rgb(16, 13, 22),          // #100D16
            fg: Color::Rgb(246, 240, 252),       // #F6F0FC
            accent: Color::Rgb(168, 85, 247),    // #A855F7
            secondary: Color::Rgb(255, 92, 184), // #FF5CB8
            muted: Color::Rgb(139, 130, 151),    // #8B8297
            selection: Color::Rgb(76, 50, 104),  // #4C3268
            error: Color::Rgb(255, 92, 114),     // #FF5C72
            warning: Color::Rgb(255, 195, 107),  // #FFC36B
            success: Color::Rgb(117, 214, 163),  // #75D6A3
            info: Color::Rgb(114, 183, 255),     // #72B7FF
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct AppConfig {
    #[serde(skip)]
    pub app_name: String,

    pub theme: ThemePalette,

    #[serde(skip)]
    pub version: String,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            app_name: "Nysh".to_string(),
            theme: ThemePalette::default(),
            version: "0.0.0".to_string(),
        }
    }
}

pub fn load_config() -> AppConfig {
    let mut config = AppConfig::default();

    if let Some(proj_dirs) = ProjectDirs::from("com", "nyverune", "nysh") {
        let config_dir = proj_dirs.config_dir();
        let config_path = config_dir.join("config.toml");

        if let Ok(content) = fs::read_to_string(&config_path) {
            match toml::from_str::<AppConfig>(&content) {
                Ok(load_config) => {config.theme = load_config.theme},
                Err(e) => eprintln!(
                    "Config parse error {}: {}. Using defaults.",
                    config_path.display(),
                    e
                ),
            }
        }
    } else {
        eprintln!("No system config directory found.");
    }

    config
}
