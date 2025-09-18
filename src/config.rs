use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::{env, fs};

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub sorting: SortingConfig,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SortingConfig {
    #[serde(rename = "by-type")]
    pub by_type: HashMap<String, Vec<String>>,

    pub misc: Option<String>,
}

impl Config {
    pub fn find_config() -> Result<Self> {
        let config_dir = match env::var("XDG_CONFIG_HOME") {
            Ok(path) => PathBuf::from(path).join("dork"),
            Err(_) => {
                panic!("Could not use $XDG_CONFIG_HOME to find config file!");
            }
        };

        let config_file = config_dir.join("config.toml");
        if config_file.exists() {
            let config_contents = fs::read_to_string(&config_file)
                .map_err(|e| anyhow!("Failed to read config file {:?}: {}", config_file, e))?;

            let config: Config = toml::from_str(&config_contents)
                .map_err(|e| anyhow!("Failed to parse config file: {}", e))?;

            return Ok(config);
        }

        println!(
            "Config file not found at {:?}. Creating a new one...",
            config_file
        );

        fs::create_dir_all(&config_dir)
            .map_err(|e| anyhow!("Failed to create config directory: {}", e))?;

        let default_config = Config::default();
        let config_str = toml::to_string_pretty(&default_config)
            .map_err(|e| anyhow!("Failed to serialize default config: {}", e))?;

        fs::write(&config_file, config_str)
            .map_err(|e| anyhow!("Failed to write config file: {}", e))?;

        println!("Created config at {:?}", config_file);

        Ok(default_config)
    }
}

impl Default for Config {
    fn default() -> Self {
        let mut directory_types = HashMap::new();

        directory_types.insert(
            "Images".to_string(),
            vec!["png", "jpg", "jpeg", "svg", "gif"]
                .into_iter()
                .map(String::from)
                .collect(),
        );

        directory_types.insert(
            "Documents".to_string(),
            vec!["docx", "pdf", "txt", "pptx", "xlsx"]
                .into_iter()
                .map(String::from)
                .collect(),
        );

        directory_types.insert(
            "Videos".to_string(),
            vec!["mp4", "mkv"].into_iter().map(String::from).collect(),
        );

        directory_types.insert(
            "Code".to_string(),
            vec![
                "py", "java", "class", "rs", "toml", "yaml", "json", "html", "nix",
            ]
            .into_iter()
            .map(String::from)
            .collect(),
        );

        Self {
            sorting: SortingConfig {
                by_type: directory_types,
                misc: Some("Misc".to_string()),
            },
        }
    }
}
