use assert_cmd::prelude::*; // Add methods on commands
use predicates::prelude::*; // Used for writing assertions
use std::process::Command; // Run programs

#[test]
#[ignore]
fn file_doesnt_exist() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("barcode_cli")?;

    cmd.arg("--fq1")
        .arg("test_1.fq")
        .arg("--fq2")
        .arg("test_2.fq.fake");
    
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("No such file or directory"));

    Ok(())
}

#[test]
fn test_main() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("barcode_cli")?;

    cmd.arg("--fq1")
        .arg("test_1.fq")
        .arg("--fq2")
        .arg("test_2.fq")
        .output()
        .expect("failed");

    Ok(())
}