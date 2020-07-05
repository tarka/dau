
use cfg_if::cfg_if;
use log::{error, info, warn};
use std::collections::HashMap;
use std::fs::read_to_string;
use std::os::unix::fs::MetadataExt;
use std::path::Path;
use serde::{self, Deserialize};
use toml;

use crate::errors::{DauError, Result};

pub const CONFFILE: &str = "/etc/dau.toml";

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


pub fn load_or_defaults<P: AsRef<Path>>(file: P) -> Result<Config> {
    load_config(file.as_ref())?
        .or_else(|| default_priv_groups())
        .ok_or(DauError::InvalidConfiguration.into())
}

fn load_config<P: AsRef<Path>>(fr: P) -> Result<Option<Config>> {
    let file = fr.as_ref();
    if !file.exists() {
        return Ok(None);
    }

    let content = read_to_string(file)?;
    let config = toml::from_str::<Config>(content.as_str())?;
    Ok(Some(config))
}


fn default_priv_groups() -> Option<Config> {
    cfg_if! {
        if #[cfg(feature = "auto_groups")] {
            // See build.rs
            const GROUP_ENV: &str = env!("DAU_PRIV_GROUPS");
            let perms = GROUP_ENV
                .split(':')
                .map(String::from)
                .map(|g| (g,
                          Perm {
                              all: true,
                              ptype: Type::Group,
                              ..Default::default()
                          }))
                .collect();
            Some(Config { perms, ..Default::default() })
        } else {
            None
        }
    }
}

pub fn check_perms<P: AsRef<Path>>(fr: P) -> Result<bool> {
    let file = fr.as_ref();
    if !file.exists() {
        info!("Config file {:?} doesn't exist (that's OK).", file);
        return Ok(true);
    }

    let meta = file.metadata()?;
    let mode = meta.mode();
    if (meta.uid() != 0)
        || (mode & 0o0004 != 0)
        || (mode & 0o0002 != 0)
    {
        error!("The config file has incorrect permissions; should be owned by root and not world readable or writable.");
        return Ok(false);
    }
    Ok(true)
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
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
        let c = toml::from_str(SIMPLE);
        let config: Config = c.unwrap();
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
        let r = load_config(&path);
        let config = r?.unwrap();
        assert_eq!("30s", config.timeout);
        assert!(config.perms["testuser"].all);
        assert_eq!(Type::Group, config.perms["testgroup"].ptype);
        assert!(!config.perms["limiteduser"].all);
        assert_eq!(vec!("/bin/ls"), config.perms["limiteduser"].commands);

        Ok(())
    }

    #[test]
    fn from_file_with_defaults() -> Result<()>{
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
        let config = load_or_defaults(&path);
        assert!(config.is_err());

        // assert_eq!("5m", config.timeout);

        // // NOTE: Depends on test OS
        // assert!(config.perms["sudo"].all);
        // assert_eq!(Type::Group, config.perms["sudo"].ptype);

        Ok(())
    }

    #[test]
    fn file_invalid_owner() -> Result<()>{
        let dir = tempdir()?;
        let path = dir.path().join("config.toml");
        {
            File::create(&path)?;
        }
        assert!(!check_perms(path)?);

        Ok(())
    }
}
