use serde::Deserialize;
use std::fs;
use std::path::PathBuf;

const DEFAULT_CONFIG: &str = r#"# nine-search configuration
# Get your API key at https://tavily.com

default_provider = "tavily"

[[providers]]
id = "tavily"
name = "Tavily Search"
key = ""
"#;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub default_provider: String,
    pub providers: Vec<ProviderConfig>,
}

#[derive(Debug, Deserialize)]
pub struct ProviderConfig {
    pub id: String,
    pub name: String,
    pub key: String,
}

pub fn config_path() -> PathBuf {
    let home = std::env::var("HOME").expect("HOME environment variable not set");
    PathBuf::from(home).join(".config").join("nine-search").join("config.toml")
}

pub fn load_or_create() -> Config {
    let path = config_path();

    if !path.exists() {
        let parent = path.parent().expect("Failed to get config parent directory");
        fs::create_dir_all(parent).expect("Failed to create config directory");
        fs::write(&path, DEFAULT_CONFIG).expect("Failed to write default config");
    }

    let content = fs::read_to_string(&path).expect("Failed to read config file");
    let config: Config = toml::from_str(&content).expect("Failed to parse config file");
    config
}

pub fn validate_api_key(config: &Config) -> Option<&ProviderConfig> {
    let provider = config
        .providers
        .iter()
        .find(|p| p.id == config.default_provider);

    match provider {
        Some(p) if !p.key.is_empty() => Some(p),
        _ => None,
    }
}
