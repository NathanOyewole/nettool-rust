use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn test_main_functionality() {
    let mut cmd = Command::cargo_bin("nettool")
        .expect("Failed to find binary 'nettool'");
    let output = cmd.arg("--path")
        .arg("/root/nettool/src/main.rs/")
        .output()
        .expect("Failed to execute command");

    println!("Command output:\n{}", String::from_utf8_lossy(&output.stdout));

    assert!(String::from_utf8_lossy(&output.stdout).contains("Total size:"));
}

