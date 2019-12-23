
use std::collections::HashMap;
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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn toml_default() {
        let config: Config = toml::from_str("").unwrap();
        assert_eq!("5m", config.timeout);
        assert_eq!(0, config.perms.len());
    }

    #[test]
    fn toml_test() {
        let s = "timeout = '30s'

                 [testuser]
                 all = true

                 [testgroup]
                 type = 'group'
                 all = true

                 [limiteduser]
                 commands = ['/bin/ls']";
        let config: Config = toml::from_str(s).unwrap();
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
}
