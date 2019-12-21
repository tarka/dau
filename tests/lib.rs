
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
