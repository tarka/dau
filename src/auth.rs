
use rpassword::read_password_from_tty;

use crate::config::Config;
use crate::errors::*;
use crate::options::Opts;

pub fn auth_user(opts: &Opts, config: &Config) -> Result<()> {
    let pw = read_password_from_tty(Some("Password: "))?;

    Ok(())
}
