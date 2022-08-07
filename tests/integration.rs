use assert_cmd::Command;
use assert_fs::prelude::*;
use predicates::prelude::*;

static VALID_JUNIT_XML: &str = include_str!("./resources/valid_junit.xml");
static VALID_JUNIT_XML_WITH_DIFFERENT_TIME: &str =
    include_str!("./resources/valid_junit_with_different_time.xml");
static INVALID_JUNIT_XML: &str = include_str!("./resources/invalid_junit.xml");

#[test]
fn prints_to_stdout_with_no_output_target() -> Result<(), Box<dyn std::error::Error>> {
    let file = assert_fs::NamedTempFile::new("junit.xml")?;
    file.write_str(VALID_JUNIT_XML)?;

    Command::cargo_bin("merge-junit")
        .unwrap()
        .args([file.path()])
        .assert()
        .success()
        .stdout(predicates::str::contains(VALID_JUNIT_XML));
    Ok(())
}

#[test]
fn merges_testsuites() -> Result<(), Box<dyn std::error::Error>> {
    let file_1 = assert_fs::NamedTempFile::new("junit_1.xml")?;
    file_1.write_str(VALID_JUNIT_XML)?;
    let file_2 = assert_fs::NamedTempFile::new("junit_2.xml")?;
    file_2.write_str(VALID_JUNIT_XML_WITH_DIFFERENT_TIME)?;

    let expected_content_1 = VALID_JUNIT_XML.strip_prefix("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<testsuites name=\"my tests\" time=\"0.03\">").unwrap().strip_suffix("</testsuites>").unwrap();
    let expected_content_2: &str = VALID_JUNIT_XML_WITH_DIFFERENT_TIME.strip_prefix("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<testsuites name=\"my tests\" time=\"0.6\">").unwrap();

    Command::cargo_bin("merge-junit")
        .unwrap()
        .args([file_1.path(), file_2.path()])
        .assert()
        .success()
        .stdout(
            predicates::str::starts_with("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<testsuites name=\"my tests\" time=\"0.63\">").and(
            predicates::str::contains(expected_content_1))
                .and(predicates::str::contains(expected_content_2)),
        );
    Ok(())
}

#[test]
fn writes_to_output_file() -> Result<(), Box<dyn std::error::Error>> {
    let file_1 = assert_fs::NamedTempFile::new("junit_1.xml")?;
    file_1.write_str(VALID_JUNIT_XML)?;
    let output_file = assert_fs::NamedTempFile::new("junit_2.xml")?;

    Command::cargo_bin("merge-junit")
        .unwrap()
        .arg(file_1.path())
        .arg("-o")
        .arg(output_file.path())
        .assert()
        .success();

    output_file.assert(VALID_JUNIT_XML);
    Ok(())
}

#[test]
fn does_not_overwrite_file_without_force_flag() -> Result<(), Box<dyn std::error::Error>> {
    let file_1 = assert_fs::NamedTempFile::new("junit_1.xml")?;
    file_1.write_str(VALID_JUNIT_XML)?;
    let output_file = assert_fs::NamedTempFile::new("junit_2.xml")?;
    output_file.touch()?;

    Command::cargo_bin("merge-junit")
        .unwrap()
        .arg(file_1.path())
        .arg("-o")
        .arg(output_file.path())
        .assert()
        .failure()
        .stdout(predicates::str::is_empty())
        .stderr(predicates::str::contains(
            "/junit_2.xml' exists. Set '--force' flag to overwrite.",
        ));

    output_file.assert("");
    Ok(())
}

#[test]
fn no_input_files_fail() -> Result<(), Box<dyn std::error::Error>> {
    let output_file = assert_fs::NamedTempFile::new("junit_1.xml")?;

    Command::cargo_bin("merge-junit")
        .unwrap()
        .arg("-o")
        .arg(output_file.path())
        .assert()
        .failure()
        .stdout(predicates::str::is_empty())
        .stderr(predicates::str::contains("required").and(predicates::str::contains("<FILE>...")));
    Ok(())
}

#[test]
fn input_file_without_testsuites_tag_fail() -> Result<(), Box<dyn std::error::Error>> {
    let input = assert_fs::NamedTempFile::new("junit_1.xml")?;

    input.write_str(INVALID_JUNIT_XML)?;

    Command::cargo_bin("merge-junit")
        .unwrap()
        .arg(input.path())
        .assert()
        .failure()
        .stderr(predicates::str::contains(
            "Could not locate <testsuites> start tag",
        ))
        .stdout(predicates::str::is_empty());
    Ok(())
}

#[test]
fn empty_input_file_fail() -> Result<(), Box<dyn std::error::Error>> {
    let input = assert_fs::NamedTempFile::new("junit_1.xml")?;
    input.touch()?;

    Command::cargo_bin("merge-junit")
        .unwrap()
        .arg(input.path())
        .assert()
        .failure()
        .stderr(predicates::str::contains(
            "Required declaration",
        ))
        .stdout(predicates::str::is_empty());
    Ok(())
}
