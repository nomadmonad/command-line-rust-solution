use assert_cmd::Command;

#[test]
fn works() {
    let mut cmd = Command::cargo_bin("hello").unwrap();
    cmd.assert().success();
}

#[test]
fn true_ok() {
    let mut cmd = Command::cargo_bin("hello").unwrap();
    cmd.assert().success();
}