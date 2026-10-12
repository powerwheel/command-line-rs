use assert_cmd::Command;

#[test]
fn run() {
    let mut cmd = Command::cargo_bin("hello").unwrap();
    let output = cmd.output().expect("fail");
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("Invalid Utf-8");
    assert_eq!(stdout, "Hello, world!\n");
}
#[test]
fn test_true() {
    let mut cmd = Command::cargo_bin("true").unwrap();
    cmd.assert().success();
}

#[test]
fn test_false() {
    let mut cmd = Command::cargo_bin("false").unwrap();
    cmd.assert().failure();
}
