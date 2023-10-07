use std::ops::Add;
use std::path::{PathBuf, Path};
use std::env;

#[derive(Debug)]
pub struct RequiredCommand {
    pub cmd: String,
    pub version: Option<String>,
}


impl RequiredCommand {
    pub fn new(cmd: String, version: Option<String>) -> RequiredCommand {
        let mut command = cmd.clone();
        let mut binary_name_without_extension = String::new();

        if let Some(file_name) = Path::new(&cmd).file_stem() {
            if let Some(file_name) = file_name.to_str() {
                binary_name_without_extension = file_name.to_string();
            } else {
                panic!("Invalid binary name");
            }
        }

        if let Some(extension) = Path::new(&cmd).extension() {
            if extension == "exe" {
                if env::consts::OS != "windows" {
                    println!("EXE extension Windows Platform support only");
                    command = binary_name_without_extension;
                }
            }
        } else {
            println!("No extension specified");
            if env::consts::OS == "windows" {
                println!("EXE extension Windows Platform support only");
                let tmp_path: String = cmd.clone().add(".exe");
                command = tmp_path;
            }
        }
        RequiredCommand {
            cmd: command,
            version,
        }
    }

    pub fn check(&self) {
        println!("{:?}", self);
        let vec = self.get_binary_paths();
        println!("{:?}", vec);
    }

    fn get_binary_paths(&self) -> Vec<PathBuf> {
        if let Some(path_env) = env::var_os("PATH") {
            let paths: Vec<PathBuf> = env::split_paths(&path_env).collect();
            let binary_paths: Vec<PathBuf> = paths
                .iter()
                .map(|path| path.join(self.cmd.clone()))
                .filter(|path| path.exists() && path.is_file())
                .collect();
            binary_paths
        } else {
            Vec::new()
        }
    }
}

#[cfg(test)]
mod tests {

    pub use crate::*;

    #[test]
    pub fn test_check() {
        let rc = RequiredCommand::new(String::from("git"), None);
        rc.check();
    }
}