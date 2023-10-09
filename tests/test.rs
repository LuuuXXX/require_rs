use require_rs::require_cmd;

#[test]
fn test_cmd_finder_1_1() {
    require_cmd!("git");
}

#[test]
fn test_cmd_finder_1_2() {
    require_cmd!("");
}

#[test]
fn test_cmd_finder_1_3() {
    require_cmd!("ttt");
}


#[test]
fn test_cmd_finder_2_1() {
    require_cmd!("git", "");
}

#[test]
fn test_cmd_finder_2_2() {
    require_cmd!("git", "0.1");
}

#[test]
fn test_cmd_finder_2_3() {
    require_cmd!("git", "3.1");
}

#[test]
fn test_cmd_finder_3_1() {
    require_cmd!("git", "0.1", "-version");
}

#[test]
fn test_cmd_finder_3_2() {
    require_cmd!("git", "0.1", "");
}

#[test]
fn test_is_empty() {
    let str = String::from("");
    assert_eq!(true, str.is_empty());
}