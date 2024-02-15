// use std::process::Command;
use assert_cmd::Command;

#[test]
fn works() {
    // let mut cmd = Command::new("./target/debug/hello_cargo");
    let mut cmd = Command::cargo_bin("hello_cargo").unwrap();
    cmd.assert().success().stdout("Hello, world!\n");
    // ブール式が true であることをテストする
    // assert!(res.is_ok());
}

#[test]
fn true_ok() {
    let mut cmd = Command::cargo_bin("true").unwrap();
    cmd.assert().success();
}

#[test]
fn false_not_ok() {
    let mut cmd = Command::cargo_bin("false").unwrap();
    cmd.assert().failure();
}
