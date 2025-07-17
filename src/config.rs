use anyhow::{Result, Context};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use directories::ProjectDirs;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub font: FontConfig,
    pub colors: ColorScheme,
    pub keybindings: Vec<KeyBinding>,
    pub features: Features,
    pub performance: Performance,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FontConfig {
    pub family: String,
    pub size: f32,
    pub auto_size: bool,
    pub ligatures: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorScheme {
    pub background: String,
    pub foreground: String,
    pub cursor: String,
    pub selection: String,
    pub black: String,
    pub red: String,
    pub green: String,
    pub yellow: String,
    pub blue: String,
    pub magenta: String,
    pub cyan: String,
    pub white: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyBinding {
    pub key: String,
    pub modifiers: Vec<String>,
    pub action: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Features {
    pub command_palette: bool,
    pub activity_indicators: bool,
    pub ai_suggestions: bool,
    pub collaborative_sessions: bool,
    pub session_recording: bool,
    pub plugins: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Performance {
    pub gpu_acceleration: bool,
    pub render_fps: u32,
    pub cache_size_mb: u32,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            font: FontConfig {
                family: "JetBrains Mono".to_string(),
                size: 14.0,
                auto_size: true,
                ligatures: true,
            },
            colors: ColorScheme {
                background: "#1e1e2e".to_string(),
                foreground: "#cdd6f4".to_string(),
                cursor: "#f5e0dc".to_string(),
                selection: "#585b70".to_string(),
                black: "#45475a".to_string(),
                red: "#f38ba8".to_string(),
                green: "#a6e3a1".to_string(),
                yellow: "#f9e2af".to_string(),
                blue: "#89b4fa".to_string(),
                magenta: "#f5c2e7".to_string(),
                cyan: "#94e2d5".to_string(),
                white: "#bac2de".to_string(),
            },
            keybindings: vec![
                KeyBinding {
                    key: "p".to_string(),
                    modifiers: vec!["ctrl".to_string(), "shift".to_string()],
                    action: "command_palette".to_string(),
                },
                KeyBinding {
                    key: "n".to_string(),
                    modifiers: vec!["ctrl".to_string(), "shift".to_string()],
                    action: "new_tab".to_string(),
                },
            ],
            features: Features {
                command_palette: true,
                activity_indicators: true,
                ai_suggestions: true,
                collaborative_sessions: false,
                session_recording: true,
                plugins: true,
            },
            performance: Performance {
                gpu_acceleration: true,
                render_fps: 120,
                cache_size_mb: 256,
            },
        }
    }
}

impl Config {
    pub fn load() -> Result<Self> {
        if let Some(proj_dirs) = ProjectDirs::from("com", "kitty-plus", "kitty-plus") {
            let config_path = proj_dirs.config_dir().join("config.toml");
            
            if config_path.exists() {
                let contents = std::fs::read_to_string(&config_path)
                    .context("Failed to read config file")?;
                let config: Config = toml::from_str(&contents)
                    .context("Failed to parse config file")?;
                Ok(config)
            } else {
                Ok(Config::default())
            }
        } else {
            Ok(Config::default())
        }
    }
    
    pub fn save(&self) -> Result<()> {
        if let Some(proj_dirs) = ProjectDirs::from("com", "kitty-plus", "kitty-plus") {
            let config_dir = proj_dirs.config_dir();
            std::fs::create_dir_all(config_dir)?;
            
            let config_path = config_dir.join("config.toml");
            let contents = toml::to_string_pretty(self)?;
            std::fs::write(config_path, contents)?;
        }
        Ok(())
    }
}