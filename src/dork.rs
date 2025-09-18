use anyhow::Result;
use log::info;

use std::env;
use std::fs;
use std::path::PathBuf;

use crate::config::Config;
pub struct Dork {
    pub config: Config,
    pub downloads_dir: PathBuf,
}

impl Dork {
    pub fn list(&self) {
        for entry in fs::read_dir(&self.downloads_dir).unwrap() {
            println!("{}", entry.unwrap().path().display());
        }
    }

    pub fn new() -> Result<Self> {
        let downloads_dir = match env::var("XDG_DOWNLOAD_DIR") {
            Ok(path) => path,
            Err(_) => String::from("~/Downloads"),
        };

        let config = Config::find_config()?;

        Ok(Self {
            downloads_dir: PathBuf::from(downloads_dir),
            config,
        })
    }

    pub fn by_type(&self) -> Result<()> {
        info!("Creating directories!");

        for directory in self.config.sorting.by_type.keys() {
            info!("- Creating {}", &directory);
            fs::create_dir_all(self.downloads_dir.join(directory))?;
        }

        if let Some(misc) = &self.config.sorting.misc {
            info!("- Creating misc dir: {}", &misc);

            fs::create_dir_all(self.downloads_dir.join(misc))?
        }

        info!("Moving files");

        for entry in fs::read_dir(&self.downloads_dir)? {
            let entry = entry?;
            let path = entry.path();

            if !path.is_file() {
                continue;
            }

            let Some(extension) = path.extension().and_then(|s| s.to_str()) else {
                continue;
            };

            let mut moved = false;

            for (category, extensions) in &self.config.sorting.by_type {
                if extensions.iter().any(|e| e.eq_ignore_ascii_case(extension)) {
                    let target = self
                        .downloads_dir
                        .join(category)
                        .join(path.file_name().unwrap());

                    fs::rename(&path, &target)?;

                    info!(
                        "Moving {} to {}",
                        &path.as_path().display(),
                        &target.as_path().display()
                    );

                    moved = true;
                    break;
                }
            }

            if !moved {
                if let Some(misc) = &self.config.sorting.misc {
                    let target = self
                        .downloads_dir
                        .join(misc)
                        .join(path.file_name().unwrap());

                    fs::rename(&path, &target)?;

                    info!(
                        "Moving {} to {}",
                        &path.as_path().display(),
                        &target.as_path().display()
                    );
                }
            }
        }

        Ok(())
    }

    pub fn recent(&self) -> Result<()> {
        info!("Checking for the most recent file!");
        let downloads_dir = &self.downloads_dir;

        let category_dirs = self
            .config
            .sorting
            .by_type
            .keys()
            .map(|key| downloads_dir.join(key))
            .collect::<Vec<PathBuf>>();

        let misc_dir = downloads_dir.join(self.config.sorting.misc.as_deref().unwrap_or("Misc"));

        let mut newest: Option<PathBuf> = None;

        for entry in fs::read_dir(downloads_dir)? {
            let entry = entry?;
            let path = entry.path();

            if category_dirs.iter().any(|directory| *directory == path) || path == misc_dir {
                continue;
            }

            if newest.is_none()
                || path.file_name().unwrap() > newest.as_ref().unwrap().file_name().unwrap()
            {
                newest = Some(path);
            }
        }

        if let Some(path) = newest {
            println!("{}", path.display());
        } else {
            println!("No recent files");
        }

        Ok(())
    }
}
