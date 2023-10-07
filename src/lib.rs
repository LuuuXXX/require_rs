mod error;
mod finder;

use finder::RequiredCommand;

#[macro_export]
macro_rules! require_cmd {
    ($cmd:expr) => { // Only binary commands name
        println!("Executing required_cmd macro");
        println!("Command: {}", $cmd);
        require_rs::find($cmd, None);
    };
    ($cmd:expr, $version:expr) => { // Binary commands name and version
        println!("Executing require_cmd macro");
        println!("Command: {}", $cmd);
        if let Some(version) = $version {
            println!("Version: {}", version);
            require_rs::find($cmd, Some(version));
        } else {
            println!("No version specified");
            require_rs::find($cmd, None);
        }
    };
}

pub fn find(cmd: String, version: Option<String>) {
    let rc = RequiredCommand::new(cmd, version);
    rc.check();
}