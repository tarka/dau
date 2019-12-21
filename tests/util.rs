

use anyhow::Error;
use escargot::CargoBuild;
use std::result;
use std::process::{Command, Output};

pub type TResult = result::Result<(), Error>;
pub type Result<T> = result::Result<T, Error>;

pub const DOCKER_IMAGE: &str = "debian:buster-slim";
pub const BIN: &str = "target/release/dau";
pub const INST_BIN: &str = "/usr/bin/dau";
pub const TESTUSER: &str = "testuser";


pub fn docker(cmd: Vec<&str>) -> Result<Output> {
    let out = Command::new("docker")
        .args(cmd)
        .output()?;
    assert!(out.status.success());
    Ok(out)
}

pub struct Container {
    id: String
}

impl Container {
    pub fn new() -> Result<Self> {
        let out = docker(vec!["run", "--detach", DOCKER_IMAGE, "sleep", "15m"])?;
        let docker = Container {
            id: String::from_utf8(out.stdout)?.trim().to_string()
        };

        Ok(docker)
    }

    pub fn kill(&self) -> Result<()> {
        let _out = docker(vec!["rm", "--force", self.id.as_str()])?;
        Ok(())
    }

    pub fn exec(self: &Self, cmd: Vec<&str>) -> Result<Output> {
        self.exec_as("root", cmd)
    }

    pub fn exec_as(self: &Self, user: &str, cmd: Vec<&str>) -> Result<Output> {
        let out = Command::new("docker")
            .arg("exec")
            .arg("--user").arg(user)
            .arg("-i")
            .arg(&self.id)
            .args(cmd)
            .output()?;
        Ok(out)
    }

    pub fn cp(self: &Self, from: &str, to: &str) -> Result<Output> {
        let remote = format!("{}:{}", self.id, to);
        let out = docker(vec!["cp", from, remote.as_str()])?;
        Ok(out)
    }

}

impl Drop for Container {
    fn drop(self: &mut Self) {
        self.kill().unwrap();
    }
}

pub fn setup() -> Result<Container> {
    let _cmd = CargoBuild::new()
        .release()
        .exec()?;

    let container = Container::new()?;
    container.exec(vec!["adduser", "--disabled-password", TESTUSER])?;
    container.exec(vec!["addgroup", "--system", "sudoers"])?;
    container.cp(BIN, INST_BIN)?;
    container.exec(vec!["chown", "root.root", INST_BIN])?;
    container.exec(vec!["chmod", "4755", INST_BIN])?;

    Ok(container)
}
