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

#[test]
fn false_ok() {
    let mut cmd = Command::cargo_bin("false").unwrap();
    cmd.assert().failure();
}

#[test]
fn runs() {
    let mut cmd = Command::cargo_bin("hello").unwrap();
    cmd.assert().success().stdout("Hello, world!\n");
}
