
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
