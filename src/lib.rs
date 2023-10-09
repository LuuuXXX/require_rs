mod error;
mod finder;
mod version;

use finder::RequiredCommand;

#[macro_export]
macro_rules! require_cmd {
    ($bin:expr) => {
        require_rs::cmd_finder($bin, None, None);
    };
    ($bin:expr, $version:expr) => {
        require_rs::cmd_finder($bin, Some($version), None);
    };
    ($bin:expr, $version:expr, $version_cmd:expr) => {
        require_rs::cmd_finder($bin, Some($version), Some($version_cmd));
    };
}

pub fn cmd_finder(cmd: &str, version: Option<&str>, version_cmd: Option<&str>) {
    let rc = RequiredCommand::parse(cmd, version, version_cmd);
    match rc {
        Ok(rc) => {
            if let Err(err) = rc.output() {
                panic!("{:?}", err);
            };
        },
        Err(e) => panic!("{:?}", e),
    }
}