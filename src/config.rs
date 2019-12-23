
use log::info;
use std::collections::HashMap;
use std::fs::read_to_string;
use std::path::Path;
use serde::{self, Deserialize};
use toml;

use crate::errors::Result;

pub const CONFFILE: &'static str = "/etc/dau.toml";

// See build.rs
pub const GROUP_ENV: &'static str = env!("DAU_PRIV_GROUPS");

pub fn default_priv_groups() -> Result<Vec<String>> {
    Ok(GROUP_ENV
       .split(":")
       .map(String::from)
       .collect())
}

fn load_config(file: &Path) -> Option<Config> {
    if !file.exists() {
        info!("Config file {:?} doesn't exist", file);
        return None;
    }
    let content = read_to_string(file).ok()?;
    toml::from_str(content.as_str()).ok()
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Type  {
    User,
    Group
}
impl Default for Type {
    fn default() -> Self { Type::User }
}

#[derive(Deserialize, Debug)]
#[serde(default)]
pub struct Config {
    timeout: String,
    #[serde(flatten)]
    perms: HashMap<String, Perm>,
}
impl Default for Config {
    fn default() -> Self {
        Self {
            timeout: "5m".to_string(),
            perms: HashMap::new(),
        }
    }
}

#[derive(Deserialize, Debug)]
#[serde(default, deny_unknown_fields)]
pub struct Perm {
    all: bool,
    #[serde(alias = "type")]
    ptype: Type,
    commands: Vec<String>,
}
impl Default for Perm {
    fn default() -> Self {
        Self {
            all: false,
            ptype: Type::User,
            commands: vec![],
        }
    }
}


pub fn load_or_defaults(file: &Path) -> Result<Config> {
    match load_config(file) {
        Some(cfg) => Ok(cfg),
        None => {
            info!("Couldn't load config, falling back to defaults");
            let mut perms = HashMap::new();
            for group in default_priv_groups()? {
                let perm = Perm {
                    all: true,
                    ptype: Type::Group,
                    ..Default::default()
                };
                perms.insert(group, perm);
            }
            Ok(Config { perms: perms, ..Default::default() })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{File};
    use std::io::{Write};
    use tempfile::tempdir;

    #[test]
    fn toml_default() {
        let config: Config = toml::from_str("").unwrap();
        assert_eq!("5m", config.timeout);
        assert_eq!(0, config.perms.len());
    }

    const SIMPLE: &str = "
        timeout = '30s'

        [testuser]
        all = true

        [testgroup]
        type = 'group'
        all = true

        [limiteduser]
        commands = ['/bin/ls']";


    #[test]
    fn toml_test() {
        let config: Config = toml::from_str(SIMPLE).unwrap();
        assert_eq!("30s", config.timeout);
        assert!(config.perms["testuser"].all);
        assert_eq!(Type::Group, config.perms["testgroup"].ptype);
        assert!(!config.perms["limiteduser"].all);
        assert_eq!(vec!("/bin/ls"), config.perms["limiteduser"].commands);
    }

    #[test]
    fn toml_unknown_field() {
        let s = "[testuser]
                 all = true
                 unknown = true";
        let config  = toml::from_str::<Config>(s);
        assert!(config.is_err());
    }

    #[test]
    fn from_file() -> Result<()>{
        let dir = tempdir()?;
        let path = dir.path().join("config.toml");
        {
            let mut fd = File::create(&path)?;
            fd.write_all(SIMPLE.as_bytes())?;
        }
        let config = load_or_defaults(&path)?;
        assert_eq!("30s", config.timeout);
        assert!(config.perms["testuser"].all);
        assert_eq!(Type::Group, config.perms["testgroup"].ptype);
        assert!(!config.perms["limiteduser"].all);
        assert_eq!(vec!("/bin/ls"), config.perms["limiteduser"].commands);

        Ok(())
    }

    #[test]
    fn file_missing() -> Result<()>{
        let dir = tempdir()?;
        let path = dir.path().join("config.toml");
        let config = load_or_defaults(&path)?;
        assert_eq!("5m", config.timeout);

        // NOTE: Depends on test OS
        assert!(config.perms["sudo"].all);
        assert_eq!(Type::Group, config.perms["sudo"].ptype);

        Ok(())
    }
}
