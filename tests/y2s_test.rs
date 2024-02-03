use assert_cmd::Command;

#[test]
fn run(){
    let mut cmd = Command::cargo_bin("y2s").unwrap();

    cmd.assert().success();
}