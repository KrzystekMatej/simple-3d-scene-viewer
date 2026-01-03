use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct AppConfig {
    pub title: String,
    pub width: u32,
    pub height: u32,
    pub min_width: u32,
    pub min_height: u32,
}

const DEFAULT_CONFIG: &str = include_str!("app_config.toml");

pub fn load_default_config() -> AppConfig {
    toml::from_str(DEFAULT_CONFIG).expect("Failed to load default app configuration")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_config_parses() {
        let cfg: AppConfig = toml::from_str(DEFAULT_CONFIG).unwrap();
        assert!(!cfg.title.is_empty());
        assert!(cfg.min_width > 0);
        assert!(cfg.min_height > 0);
        assert!(cfg.width >= cfg.min_width);
        assert!(cfg.height >= cfg.min_height);
    }
}
