
mod util;

use crate::util::*;


#[test]
fn help() -> TResult {
    let container = setup()?;
    let out = container.exec(vec![INST_BIN, "--help"])?;
    assert!(out.status.success());
    assert!(String::from_utf8(out.stdout)?
            .contains("Do As User: Run commands as, or switch to, a user"));

    Ok(())
}

#[test]
fn no_setuid() -> TResult {
    let container = setup()?;
    container.exec(vec!["chmod", "a-s", INST_BIN])?;

    let out = container.exec_as(TESTUSER, vec![INST_BIN, "/bin/ls"])?;
    assert!(!out.status.success());
    assert!(String::from_utf8(out.stderr)?
            .contains("The dau binary is not setuid root"));

    Ok(())
}

#[test]
fn config_not_root() -> TResult {
    let container = setup()?;
    container.exec(vec!["touch", "/etc/dau.toml"])?;
    container.exec(vec!["chown", "testuser", "/etc/dau.toml"])?;

    let out = container.exec_as(TESTUSER, vec![INST_BIN, "/bin/ls"])?;
    assert!(!out.status.success());
    assert!(String::from_utf8(out.stderr)?
            .contains("The config file has incorrect permissions"));

    Ok(())
}

#[test]
fn config_world_readable() -> TResult {
    let container = setup()?;
    container.exec(vec!["touch", "/etc/dau.toml"])?;
    container.exec(vec!["chmod", "0666", "/etc/dau.toml"])?;

    let out = container.exec_as(TESTUSER, vec![INST_BIN, "/bin/ls"])?;
    assert!(!out.status.success());
    assert!(String::from_utf8(out.stderr)?
            .contains("The config file has incorrect permissions"));

    Ok(())
}

#[test]
fn config_perms_ok() -> TResult {
    let container = setup()?;
    container.exec(vec!["touch", "/etc/dau.toml"])?;
    container.exec(vec!["chmod", "0600", "/etc/dau.toml"])?;

    let out = container.exec_as(TESTUSER, vec![INST_BIN, "/bin/ls"])?;
    assert!(out.status.success());
    assert!(!String::from_utf8(out.stderr)?
            .contains("The config file has incorrect permissions"));

    Ok(())
}
