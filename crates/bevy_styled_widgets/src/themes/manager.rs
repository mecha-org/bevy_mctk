use bevy::prelude::*;
use serde::{Deserialize, Deserializer, Serialize};
use std::{collections::HashMap, fs::File};

use super::styles::ThemeStyles;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ThemeId(pub String);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeColors {
    #[serde(deserialize_with = "deserialize_color")]
    pub background: Color,
    #[serde(deserialize_with = "deserialize_color")]
    pub foreground: Color,
    #[serde(deserialize_with = "deserialize_color")]
    pub card: Color,
    #[serde(deserialize_with = "deserialize_color")]
    pub card_foreground: Color,
    #[serde(deserialize_with = "deserialize_color")]
    pub popover: Color,
    #[serde(deserialize_with = "deserialize_color")]
    pub popover_foreground: Color,
    #[serde(deserialize_with = "deserialize_color")]
    pub primary: Color,
    #[serde(deserialize_with = "deserialize_color")]
    pub primary_foreground: Color,
    #[serde(deserialize_with = "deserialize_color")]
    pub secondary: Color,
    #[serde(deserialize_with = "deserialize_color")]
    pub secondary_foreground: Color,
    #[serde(deserialize_with = "deserialize_color")]
    pub muted: Color,
    #[serde(deserialize_with = "deserialize_color")]
    pub muted_foreground: Color,
    #[serde(deserialize_with = "deserialize_color")]
    pub accent: Color,
    #[serde(deserialize_with = "deserialize_color")]
    pub accent_foreground: Color,
    #[serde(deserialize_with = "deserialize_color")]
    pub destructive: Color,
    #[serde(deserialize_with = "deserialize_color")]
    pub border: Color,
    #[serde(deserialize_with = "deserialize_color")]
    pub input: Color,
    #[serde(deserialize_with = "deserialize_color")]
    pub ring: Color,
    #[serde(deserialize_with = "deserialize_color")]
    pub chart: Color,
    #[serde(deserialize_with = "deserialize_color")]
    pub sidebar: Color,
    #[serde(deserialize_with = "deserialize_color")]
    pub sidebar_foreground: Color,
    #[serde(deserialize_with = "deserialize_color")]
    pub sidebar_primary: Color,
    #[serde(deserialize_with = "deserialize_color")]
    pub sidebar_primary_foreground: Color,
    #[serde(deserialize_with = "deserialize_color")]
    pub sidebar_accent: Color,
    #[serde(deserialize_with = "deserialize_color")]
    pub sidebar_accent_foreground: Color,
    #[serde(deserialize_with = "deserialize_color")]
    pub sidebar_border: Color,
    #[serde(deserialize_with = "deserialize_color")]
    pub sidebar_ring: Color,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ThemeMode {
    #[serde(alias = "Light", alias = "LIGHT")]
    Light,
    #[serde(alias = "Dark", alias = "DARK")]
    Dark,
}

impl Default for ThemeMode {
    fn default() -> Self {
        ThemeMode::Light
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct ThemeConfig {
    name: String,
    modes: Vec<ThemeMode>,
    default_mode: ThemeMode,
    light: Option<ThemeModeConfigs>,
    dark: Option<ThemeModeConfigs>,
}

impl<'de> Deserialize<'de> for ThemeConfig {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct RawThemeConfig {
            name: String,
            modes: Vec<ThemeMode>,
            default_mode: ThemeMode,
            light: Option<ThemeModeConfigs>,
            dark: Option<ThemeModeConfigs>,
        }

        let raw = RawThemeConfig::deserialize(deserializer)?;

        if raw.modes.is_empty() {
            return Err(serde::de::Error::custom(
                "At least one mode must be provided",
            ));
        }

        if raw.modes.contains(&ThemeMode::Light) && raw.light.is_none() {
            return Err(serde::de::Error::custom(
                "Light mode is specified but no light theme is provided",
            ));
        }

        if raw.modes.contains(&ThemeMode::Dark) && raw.dark.is_none() {
            return Err(serde::de::Error::custom(
                "Dark mode is specified but no dark theme is provided",
            ));
        }

        if raw.default_mode == ThemeMode::Dark && raw.dark.is_none() {
            return Err(serde::de::Error::custom(
                "Default mode is dark but no dark theme is provided",
            ));
        }

        if raw.default_mode == ThemeMode::Light && raw.light.is_none() {
            return Err(serde::de::Error::custom(
                "Default mode is light but no light theme is provided",
            ));
        }

        Ok(ThemeConfig {
            name: raw.name,
            modes: raw.modes,
            default_mode: raw.default_mode,
            light: raw.light,
            dark: raw.dark,
        })
    }
}

fn deserialize_color<'de, D>(deserializer: D) -> Result<Color, D::Error>
where
    D: Deserializer<'de>,
{
    let values: Vec<f32> = Vec::deserialize(deserializer)?;
    match values.len() {
        3 => Ok(Color::oklcha(values[0], values[1], values[2], 1.0)),
        4 => Ok(Color::oklcha(values[0], values[1], values[2], values[3])),
        _ => Err(serde::de::Error::custom(
            "Expected 3 or 4 float values for OKLCH(A)",
        )),
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeModeConfigs {
    pub colors: ThemeColors,
    pub icons: HashMap<String, String>,
}

#[derive(Resource)]
pub struct ThemeManager {
    pub themes: HashMap<(ThemeId, ThemeMode), ThemeModeConfigs>,
    pub current_theme: ThemeId,
    pub current_mode: ThemeMode,
    pub styles: ThemeStyles,
}

impl Default for ThemeManager {
    fn default() -> Self {
        let mut themes: HashMap<(ThemeId, ThemeMode), ThemeModeConfigs> = HashMap::new();
        let current_theme = ThemeId("default".to_string());
        let mut current_mode = ThemeMode::Light;
        let mut theme_styles: Option<ThemeStyles> = None;

        // Load default theme files
        let theme_configs = ThemeManager::load_themes_from_dir("assets/themes").unwrap_or_default();
        for theme in theme_configs {
            let ThemeConfig {
                name,
                default_mode,
                light,
                dark,
                ..
            } = theme;
            let theme_id = ThemeId(name.clone());

            // Handle light mode
            match &light {
                Some(light_theme) => {
                    themes.insert((theme_id.clone(), ThemeMode::Light), light_theme.clone());
                }

                None => {
                    let dark_theme = dark.as_ref().unwrap();
                    themes.insert((theme_id.clone(), ThemeMode::Light), dark_theme.clone());
                }
            }

            // Handle dark mode
            match &dark {
                Some(dark_theme) => {
                    themes.insert((theme_id, ThemeMode::Dark), dark_theme.clone());
                }
                None => {
                    let light_theme = light.as_ref().unwrap();
                    themes.insert((theme_id, ThemeMode::Dark), light_theme.clone());
                }
            }

            if name.as_str() == "default" {
                current_mode = default_mode;
                match default_mode {
                    ThemeMode::Light => {
                        theme_styles = Some(ThemeStyles::from_colors(light.unwrap()));
                    }
                    ThemeMode::Dark => {
                        theme_styles = Some(ThemeStyles::from_colors(dark.unwrap()));
                    }
                }
            }
        }

        Self {
            themes,
            current_theme,
            current_mode,
            styles: theme_styles.unwrap(),
        }
    }
}

impl ThemeManager {
    pub fn set_theme(&mut self, theme: ThemeId) {
        if let Some(colors) = self.themes.get(&(theme.clone(), self.current_mode.clone())) {
            self.current_theme = theme.clone();
            self.styles = ThemeStyles::from_colors(colors.clone());
        } else {
            warn!("Theme {:?} not found", theme);
        }
    }

    pub fn set_theme_mode(&mut self, mode: ThemeMode) {
        if let Some(colors) = self.themes.get(&(self.current_theme.clone(), mode.clone())) {
            self.current_mode = mode.clone();
            self.styles = ThemeStyles::from_colors(colors.clone());
        } else {
            warn!(
                "Theme {:?} with mode {:?} not found",
                self.current_theme, mode
            );
        }
    }
    pub fn set_theme_and_mode(&mut self, theme: ThemeId, mode: ThemeMode) {
        if let Some(colors) = self.themes.get(&(theme.clone(), mode.clone())) {
            self.current_theme = theme.clone();
            self.current_mode = mode.clone();
            self.styles = ThemeStyles::from_colors(colors.clone());
        } else {
            warn!("Theme {:?} with mode {:?} not found", theme, mode);
        }
    }

    pub fn add_theme(&mut self, theme: ThemeId, mode: ThemeMode, configs: ThemeModeConfigs) {
        self.themes.insert((theme, mode), configs);
    }

    pub fn remove_theme(&mut self, theme: ThemeId, mode: ThemeMode) -> Result<(), String> {
        if self.themes.remove(&(theme.clone(), mode.clone())).is_none() {
            return Err(format!("Theme {:?} with mode {:?} not found", theme, mode));
        }
        Ok(())
    }

    pub fn update_theme(
        &mut self,
        theme: ThemeId,
        mode: ThemeMode,
        configs: ThemeModeConfigs,
    ) -> Result<(), String> {
        if !self.themes.contains_key(&(theme.clone(), mode.clone())) {
            return Err(format!("Theme {:?} with mode {:?} not found", theme, mode));
        }
        self.themes.insert((theme, mode), configs);
        Ok(())
    }

    pub fn get_theme(&self, theme: ThemeId, mode: ThemeMode) -> Option<&ThemeModeConfigs> {
        self.themes.get(&(theme, mode))
    }

    pub fn add_theme_from_file(&mut self, path: &str) -> Result<bool> {
        let ThemeConfig {
            name,
            modes: _, // Unused variable
            default_mode,
            light,
            dark,
        } = ThemeManager::load_theme_from_file(path)?;

        let theme_id = ThemeId(name);

        // Handle light mode
        match &light {
            Some(light_theme) => {
                self.add_theme(theme_id.clone(), ThemeMode::Light, light_theme.clone())
            }
            None => {
                let dark_theme = dark.as_ref().unwrap();
                self.add_theme(theme_id.clone(), ThemeMode::Light, dark_theme.clone())
            }
        }

        // Handle dark mode
        match &dark {
            Some(dark_theme) => self.add_theme(theme_id, ThemeMode::Dark, dark_theme.clone()),
            None => {
                let light_theme = light.as_ref().unwrap();
                self.add_theme(theme_id, ThemeMode::Dark, light_theme.clone())
            }
        }

        self.set_theme_mode(default_mode);
        Ok(true)
    }
    pub fn load_theme_from_file(path: &str) -> Result<ThemeConfig> {
        // Read file
        let file = match File::open(path) {
            Ok(file) => file,
            Err(e) => {
                error!("Failed to open theme file: {}", e);
                return Err("Failed to open theme file".into());
            }
        };

        // Deserialize yml
        let config: ThemeConfig = match serde_yaml::from_reader(file) {
            Ok(config) => config,
            Err(e) => {
                error!("Failed to parse theme file {} with error: {}", path, e);
                return Err("Failed to parse theme file".into());
            }
        };

        Ok(config)
    }

    pub fn load_themes_from_dir(path: &str) -> Result<Vec<ThemeConfig>> {
        let mut themes = Vec::new();
        let dir = match std::fs::read_dir(path) {
            Ok(v) => v,
            Err(e) => {
                error!("Failed to read themes directory: {}", e);
                return Err((format!("Failed to read themes directory {}", e)).into());
            }
        };
        for dir_entry in dir {
            let entry = match dir_entry {
                Ok(v) => v,
                Err(e) => {
                    error!("Failed to read theme file: {}", e);
                    continue;
                }
            };
            if entry.path().extension().map_or(false, |ext| ext == "yml") {
                let theme = ThemeManager::load_theme_from_file(entry.path().to_str().unwrap())?;
                themes.push(theme);
            }
        }
        Ok(themes)
    }
}
