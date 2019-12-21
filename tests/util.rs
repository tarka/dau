

use anyhow::Error;
use escargot::CargoBuild;
use std::result;
use std::process::{Command, Output};

pub type TResult = result::Result<(), Error>;
pub type Result<T> = result::Result<T, Error>;

pub const BIN: &str = "target/release/dau";


pub fn docker(cmd: Vec<&str>) -> Result<Output> {
    let out = Command::new("docker")
        .args(cmd)
        .output()?;
    assert!(out.status.success());
    Ok(out)
}

struct Docker {
    id: String
}

impl Docker {
    pub fn new() -> Result<Self> {
        let out = docker(vec!["run", "--detach", "alpine:3", "sleep", "15m"])?;
        let docker = Docker {
            id: String::from_utf8(out.stdout)?.trim().to_string()
        };

        Ok(docker)
    }

    pub fn kill(&self) -> Result<()> {
        let _out = docker(vec!["rm", "--force", self.id.as_str()])?;
        Ok(())
    }

    pub fn exec(self: &Self, cmd: Vec<&str>) -> Result<Output> {
        let out = Command::new("docker")
            .arg("exec")
            .arg(&self.id)
            .args(cmd)
            .output()?;
        assert!(out.status.success());
        Ok(out)
    }

    pub fn cp(self: &Self, from: &str, to: &str) -> Result<Output> {
        let out = Command::new("docker")
            .arg("cp")
            .arg(from)
            .arg(format!("{}:{}", self.id, to))
            .output()?;
        assert!(out.status.success());
        Ok(out)
    }

}

impl Drop for Docker {
    fn drop(self: &mut Self) {
        self.kill().unwrap();
    }
}

pub fn run() -> Result<()> {
    let _cmd = CargoBuild::new()
        .release()
        .exec()?;

    let container = Docker::new()?;
    container.exec(vec!["adduser", "-D", "testuser"])?;
    container.exec(vec!["addgroup", "-S", "sudoers"])?;
    container.cp(BIN, "/usr/bin/dau")?;

    Ok(())
}
