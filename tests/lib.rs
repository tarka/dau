
mod util;

use test_case::test_case;

use crate::util::*;


#[test_case(""; "Test with default features")]
#[test_case("auto_groups"; "Test with auto-groups feature")]
fn help(features: &str) -> TResult {
    let container = setup(features)?;
    let out = container.exec(vec![INST_BIN, "--help"])?;
    assert!(out.status.success());
    assert!(String::from_utf8(out.stdout)?
            .contains("Do As User: Run commands as, or switch to, a user"));

    Ok(())
}

#[test_case(""; "Test with default features")]
#[test_case("auto_groups"; "Test with auto-groups feature")]
fn no_setuid(features: &str) -> TResult {
    let container = setup(features)?;
    container.exec(vec!["chmod", "a-s", INST_BIN])?;

    let out = container.exec_as(TESTUSER, vec![INST_BIN, "/bin/ls"])?;
    assert!(!out.status.success());
    assert!(String::from_utf8(out.stderr)?
            .contains("The dau binary is not setuid root"));

    Ok(())
}

#[test_case(""; "Test with default features")]
#[test_case("auto_groups"; "Test with auto-groups feature")]
fn config_not_root(features: &str) -> TResult {
    let container = setup(features)?;
    container.exec(vec!["touch", "/etc/dau.toml"])?;
    container.exec(vec!["chown", "testuser", "/etc/dau.toml"])?;

    let out = container.exec_as(TESTUSER, vec![INST_BIN, "/bin/ls"])?;
    assert!(!out.status.success());
    assert!(String::from_utf8(out.stderr)?
            .contains("The config file has incorrect permissions"));

    Ok(())
}

#[test_case(""; "Test with default features")]
#[test_case("auto_groups"; "Test with auto-groups feature")]
fn config_world_readable(features: &str) -> TResult {
    let container = setup(features)?;
    container.exec(vec!["touch", "/etc/dau.toml"])?;
    container.exec(vec!["chmod", "0666", "/etc/dau.toml"])?;

    let out = container.exec_as(TESTUSER, vec![INST_BIN, "/bin/ls"])?;
    assert!(!out.status.success());
    assert!(String::from_utf8(out.stderr)?
            .contains("The config file has incorrect permissions"));

    Ok(())
}

#[test_case(""; "Test with default features")]
#[test_case("auto_groups"; "Test with auto-groups feature")]
fn config_perms_ok(features: &str) -> TResult {
    let container = setup(features)?;
    container.exec(vec!["touch", "/etc/dau.toml"])?;
    container.exec(vec!["chmod", "0600", "/etc/dau.toml"])?;

    let out = container.exec_w_pass(TESTUSER, TESTPASS, vec![INST_BIN, "/bin/ls"])?;
    assert!(out.status.success());
    assert!(!String::from_utf8(out.stderr)?
            .contains("The config file has incorrect permissions"));

    Ok(())
}
