use std::process::Command;

use crate::error::Error;
use crate::version::check_version;

#[derive(Debug)]
pub struct RequiredCommand {
    /// The name of the required command.
    pub binary: String,
    /// The version of the required command.
    pub version: Option<String>,
    /// Way to get the version of the needed binary
    /// By default this will be ignored. We usually want to 
    /// get the version by using the `--version` option.
    pub version_cmd: Option<String>,
}

impl RequiredCommand {
    pub fn parse(
        binary: &str,
        version: Option<&str>,
        version_cmd: Option<&str>,
    )  -> Result<RequiredCommand, Error> {
        if binary.is_empty() {
            return Err(Error::BadBinaryName);
        }
        let mut vs: Option<String> = None;
        if let Some(version) = version {
            if !version.is_empty() {
                vs = Some(version.to_owned());
            }
        }
        let mut vc: Option<String> = None;
        if let Some(version_cmd) = version_cmd {
            if !version_cmd.is_empty() {
                vc = Some(version_cmd.to_owned());
            }
        }
        Ok(Self::parser_inner(binary.to_owned(), vs, vc))
    }

    fn parser_inner(
        binary: String,
        version: Option<String>,
        version_cmd: Option<String>,
    ) -> RequiredCommand {
        RequiredCommand { 
            binary, 
            version, 
            version_cmd 
        }
    }

    pub fn output(&self) -> Result<(), Error> {
        self.version_cmd()
    }

    fn version_cmd(&self) -> Result<(), Error> {
        let mut cmd = Command::new(&self.binary);
        if let Some(version_cmd) = &self.version_cmd {
            cmd.arg(version_cmd); // custom version command
        } else {
            cmd.arg("--version");
        }
        println!("{:?}", cmd);
        if let Ok(output) = cmd.output() {
            if output.status.success() {
                let stdout = String::from_utf8_lossy(&output.stdout);
                let version = check_version(&stdout, self.version.as_deref());
                println!("Command output: {:?}", version);
            } else {
                let stderr = String::from_utf8_lossy(&output.stderr);
                println!("Error version command: \n {:?}", stderr);
                return Err(Error::InvalidBinaryVersionCommand);
            }
        } else {
            return Err(Error::FailedFindBinary);
        }
        Ok(())
    }
}