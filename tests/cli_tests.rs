use assert_cmd::Command;

#[test]
fn test_supp_tags_flag() {
    let mut cmd = Command::cargo_bin("id3-parser").unwrap();
    cmd.arg("--supp-tags")
        .assert()
        .success()
        .stdout(predicates::str::contains("Supported ID3 Tag IDs"));
}

#[test]
fn test_show_command() {
    let mut cmd = Command::cargo_bin("id3-parser").unwrap();
    cmd.arg("--file")
        .arg("resources/Maestro Chives, Egzod, Neoni - Royalty [NCS Release] TEST.mp3")
        .arg("--show")
        .assert()
        .success()
        .stdout(predicates::str::contains("Parsing results for"));
}

#[test]
fn test_add_command() {
    let mut cmd = Command::cargo_bin("id3-parser").unwrap();
    cmd.arg("--file")
        .arg("resources/Maestro Chives, Egzod, Neoni - Royalty [NCS Release] TEST.mp3")
        .arg("--add")
        .arg("TIT2=Test Title")
        .assert()
        .success()
        .stdout(predicates::str::contains("Added tag"));
}

#[test]
fn test_no_file_provided() {
    let mut cmd = Command::cargo_bin("id3-parser").unwrap();
    cmd.assert()
        .failure()
        .stderr(predicates::str::contains("❌ No file provided"));
}

#[test]
fn test_no_other_command() {
    let mut cmd = Command::cargo_bin("id3-parser").unwrap();
    cmd.arg("--file")
        .arg("resources/Maestro Chives, Egzod, Neoni - Royalty [NCS Release] TEST.mp3")
        .assert()
        .success()
        .stdout(predicates::str::contains("No other command found"));
}
