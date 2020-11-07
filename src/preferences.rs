use crate::utils::BoxResult;
use serde::{Deserialize, Serialize};
use std::fs::{canonicalize, File};
use std::io::prelude::*;

/// The Preferences for rust_bot_detector
#[derive(Serialize, Deserialize, Debug)]
pub struct Preferences {
    pub ip: String,
    pub port: u16,
    pub password: String,

    pub tf2_exe: String,

    pub tf2_log_file: String,
}

const PREFERENCE_FILENAME: &str = "preferences.rust_bot_detector.json";

impl Preferences {
    /// Tries to load the preferences.rust_bot_detector.json file from the current directory.
    /// If the file don't exist, use default values.
    pub fn load_or_default() -> Self {
        match Self::load() {
            Ok(preferences) => preferences,
            Err(error) => {
                println!("Error loading preference file: {}.", error);

                let preferences = Default::default();
                println!("Using default values: {:#?}.", preferences);
                preferences
            }
        }
    }

    pub fn load() -> BoxResult<Preferences> {
        let mut f = File::open(PREFERENCE_FILENAME)?;
        let mut json = String::new();
        f.read_to_string(&mut json)?;
        let preferences: Preferences = serde_json::from_str(&json).unwrap();

        Ok(preferences)
    }

    pub fn save(&self) {
        let json = serde_json::to_string_pretty(self).unwrap();
        let mut f = File::create(PREFERENCE_FILENAME).unwrap();
        f.write(json.as_bytes()).unwrap();

        println!("Preferences saved to file {}", PREFERENCE_FILENAME);
    }
}

/// Default preference values for the platform.
impl Default for Preferences {
    #[cfg(target_os = "windows")]
    fn default() -> Self {
        Preferences {
            ip: "127.0.0.1".to_string(),
            port: 40434,
            password: "".to_string(),
            tf2_exe: r"C:\Program Files (x86)\Steam\steamapps\common\Team Fortress 2\hl2.exe"
                .to_string(),
            tf2_log_file:
                r"C:\Program Files (x86)\Steam\steamapps\common\Team Fortress 2\tf\console.log"
                    .to_string(),
        }
    }

    #[cfg(target_os = "linux")]
    fn default() -> Self {
        // TODO: If "~" does not work, use $HOME or use some other function to expand the path to absolute.
        Preferences {
            ip: "127.0.0.1".to_string(),
            port: 40434,
            password: "".to_string(),
            tf2_exe: canonicalize(
                r"~/.local/share/Steam/steamapps/common/Team Fortress 2/hl2_linux",
            )
            .unwrap()
            .to_str()
            .unwrap()
            .to_string(),
            tf2_log_file: canonicalize(
                r"~/.local/share/Steam/steamapps/common/Team Fortress 2/tf/console.log",
            )
            .unwrap()
            .to_str()
            .unwrap()
            .to_string(),
        }
    }

    #[cfg(target_os = "macos")]
    fn default() -> Self {
        // TODO: Use macOS default values.
        Preferences {
            ip: "127.0.0.1".to_string(),
            port: 40434,
            password: "".to_string(),
            tf2_exe: r"".to_string(),
            tf2_log_file: r"".to_string(),
        }
    }
}
