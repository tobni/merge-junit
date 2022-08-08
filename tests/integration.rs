use assert_fs::prelude::*;
use predicates::prelude::*;

static VALID_JUNIT_XML: &str = include_str!("./resources/valid_junit.xml");
static VALID_JUNIT_XML_WITH_DIFFERENT_TIME: &str =
    include_str!("./resources/valid_junit_with_different_time.xml");
static INVALID_JUNIT_XML: &str = include_str!("./resources/invalid_junit.xml");
static VALID_JUNIT_XML_SINGLE_TESTSUITE: &str =
    include_str!("./resources/valid_junit_single_testsuite.xml");

#[test]
fn prints_to_stdout_with_no_output_target() -> Result<(), Box<dyn std::error::Error>> {
    let file = assert_fs::NamedTempFile::new("junit.xml")?;
    file.write_str(VALID_JUNIT_XML)?;

    assert_cmd::Command::cargo_bin("merge-junit")
        .unwrap()
        .args([file.path()])
        .assert()
        .success()
        .stdout_contains_lines_trimmed(VALID_JUNIT_XML.lines());
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

    assert_cmd::Command::cargo_bin("merge-junit")
        .unwrap()
        .args([file_1.path(), file_2.path()])
        .assert()
        .success()
        .stdout(
            predicates::str::starts_with("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<testsuites name=\"my tests\" time=\"0.63\">")
        ).stdout_contains_lines_trimmed(expected_content_1.lines().chain(expected_content_2.lines()));
    Ok(())
}

#[test]
fn writes_to_output_file() -> Result<(), Box<dyn std::error::Error>> {
    let file_1 = assert_fs::NamedTempFile::new("junit_1.xml")?;
    file_1.write_str(VALID_JUNIT_XML)?;
    let output_file = assert_fs::NamedTempFile::new("junit_2.xml")?;

    assert_cmd::Command::cargo_bin("merge-junit")
        .unwrap()
        .arg(file_1.path())
        .arg("-o")
        .arg(output_file.path())
        .assert()
        .success();

    output_file.contains_lines_trimmed(VALID_JUNIT_XML.lines());
    Ok(())
}

#[test]
fn does_not_overwrite_file_without_force_flag() -> Result<(), Box<dyn std::error::Error>> {
    let file_1 = assert_fs::NamedTempFile::new("junit_1.xml")?;
    file_1.write_str(VALID_JUNIT_XML)?;
    let output_file = assert_fs::NamedTempFile::new("junit_2.xml")?;
    output_file.touch()?;

    assert_cmd::Command::cargo_bin("merge-junit")
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

    assert_cmd::Command::cargo_bin("merge-junit")
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

    assert_cmd::Command::cargo_bin("merge-junit")
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
fn accepts_single_testsuite() -> Result<(), Box<dyn std::error::Error>> {
    let input = assert_fs::NamedTempFile::new("junit_1.xml")?;
    input.write_str(VALID_JUNIT_XML_SINGLE_TESTSUITE)?;
    let lines = VALID_JUNIT_XML_SINGLE_TESTSUITE.lines();
    assert_cmd::Command::cargo_bin("merge-junit")
        .unwrap()
        .arg(input.path())
        .assert()
        .success()
        .stdout_contains_lines_trimmed(lines);
    Ok(())
}

#[test]
fn can_merge_single_testsuite_with_testsuites() -> Result<(), Box<dyn std::error::Error>> {
    let input_1 = assert_fs::NamedTempFile::new("junit_1.xml")?;
    input_1.write_str(VALID_JUNIT_XML_SINGLE_TESTSUITE)?;
    let input_2 = assert_fs::NamedTempFile::new("junit_2.xml")?;
    input_2.write_str(VALID_JUNIT_XML)?;

    assert_cmd::Command::cargo_bin("merge-junit")
        .unwrap()
        .args([input_1.path(), input_2.path()])
        .assert()
        .success()
        .stdout_contains_lines_trimmed(
            VALID_JUNIT_XML_SINGLE_TESTSUITE
                .lines()
                .chain(VALID_JUNIT_XML.lines()),
        );

    Ok(())
}

trait ContainsLinesTrimmedStdout {
    fn stdout_contains_lines_trimmed<'a>(self, lines: impl Iterator<Item = &'a str>) -> Self;
}

impl ContainsLinesTrimmedStdout for assert_cmd::assert::Assert {
    fn stdout_contains_lines_trimmed<'a>(self, lines: impl Iterator<Item = &'a str>) -> Self {
        fn contains_trimmed_line(
            assert: assert_cmd::assert::Assert,
            line: &str,
        ) -> assert_cmd::assert::Assert {
            assert.stdout(predicates::str::contains(line.trim()))
        }
        lines.fold(self, contains_trimmed_line)
    }
}

trait ContainsLinesTrimmedFile {
    fn contains_lines_trimmed<'a>(self, lines: impl Iterator<Item = &'a str>) -> Self;
}

impl ContainsLinesTrimmedFile for &assert_fs::NamedTempFile {
    fn contains_lines_trimmed<'a>(self, lines: impl Iterator<Item = &'a str>) -> Self {
        let content = std::fs::read(self).unwrap();
        let string = String::from_utf8(content).unwrap();
        for line in lines {
            assert!(string.contains(line.trim()))
        }
        self
    }
}
