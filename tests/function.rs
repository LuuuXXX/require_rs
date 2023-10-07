use require_rs::require_cmd;


#[test]
fn test_fnnction_like_macro_without_version() {
    let cmd: String = String::from("git");
    require_cmd!(cmd);
}


#[test]
fn test_fnnction_like_macro_with_none_version() {
    let cmd: String = String::from("git");
    let version: Option<String> = None;
    require_cmd!(cmd, version);
}

#[test]
fn test_fnnction_like_macro_with_version() {
    let cmd: String = String::from("git");
    let version: Option<String> = Some(String::from("1.1.1"));
    require_cmd!(cmd, version);
}