

use anyhow::Error;
use escargot::CargoBuild;
use std::result;
use std::process::{Command, Output};

pub type TResult = result::Result<(), Error>;
pub type Result<T> = result::Result<T, Error>;

pub const DOCKER_IMAGE: &str = "debian:buster-slim";
pub const INST_BIN: &str = "/usr/bin/dau";
pub const TESTUSER: &str = "testuser";
pub const TESTPASS: &str = "testpass";


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

    pub fn exec_w_pass<'a>(self: &Self, user: &'a str, pass: &'a str, mut cmd: Vec<&'a str>) -> Result<Output>
    {
        let mut ncmd = vec!["echo", pass, "|"];
        ncmd.append(&mut cmd);
        let out = self.exec_as(user, ncmd)?;
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

fn build_target(features: &str) -> Result<String> {
    let target_dir = if features == "" {
        "target".to_owned()
    } else {
        format!("target/{}", features.replace(" ", "_"))
    };
    let bin = format!("{}/release/dau", target_dir);

    let _cmd = CargoBuild::new()
        .features(features)
        .target_dir(target_dir.as_str())
        .release()
        .exec()?;

    Ok(bin)
}

pub fn setup(features: &str) -> Result<Container> {
    let bin = build_target(features)?;

    let container = Container::new()?;
    container.exec(vec!["adduser", "--disabled-password", TESTUSER])?;
    container.exec(vec!["echo", format!("{}\n{}\n", TESTPASS, TESTPASS).as_str(), "|", "passwd", TESTUSER])?;
    container.exec(vec!["addgroup", "--system", "sudoers"])?;

    container.cp(bin.as_str(), INST_BIN)?;
    container.exec(vec!["chown", "root.root", INST_BIN])?;
    container.exec(vec!["chmod", "4755", INST_BIN])?;

    Ok(container)
}
