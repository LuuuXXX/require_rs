use regex::Regex;

use crate::error::Error;

pub fn get_version_from_string(version: &str) -> Option<String> {
    let re = Regex::new(r"(\d+\.\d+\.\d+)").unwrap();
    if let Some(captures) = re.captures(version) {
        Some(captures.get(1).unwrap().as_str().to_owned())
        } else {
        None
    }
}

pub fn check_version(version: &str, expect_version: Option<&str>) -> Result<String, Error> {
    let local_version = match get_version_from_string(version) {
        Some(v) => v,
        None => return Err(Error::FailedFindBinaryVersion),
    };

    match expect_version {
        Some(expected) => {
            match expected.cmp(&local_version) {
                std::cmp::Ordering::Less => Ok(local_version),
                std::cmp::Ordering::Equal => Ok(local_version),
                std::cmp::Ordering::Greater => return Err(Error::FailedFindExpectedBinaryVersion),
            } 
        },
        None => Ok(local_version),
    }
}