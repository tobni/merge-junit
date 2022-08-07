use anyhow::Result;

use super::*;

#[test]
fn parses_from_matches_with_force_flag_to_none() -> Result<()> {
    let path: Option<MockPath> = None;
    let parser = OutputPathParser {
        path: path.as_ref(),
        force: true,
    };

    assert!(parser.parse()?.is_none());
    Ok(())
}

#[test]
fn parses_from_matches_with_force_flag_and_output_to_output() -> Result<()> {
    let path = given_path("/a/path", true);
    let parser = OutputPathParser {
        path: Some(&path),
        force: true,
    };

    assert_eq!(parser.parse()?.unwrap().to_str().unwrap(), "/a/path");
    Ok(())
}

#[test]
fn parses_from_matches_with_output_and_without_force_flag_is_an_error_when_file_exists(
) -> Result<()> {
    let path = given_path("/a/path", true);
    let parser = OutputPathParser {
        path: Some(&path),
        force: false,
    };

    assert!(parser.parse().is_err());
    Ok(())
}

fn given_path(string: &'static str, exists: bool) -> MockPath {
    MockPath { string, exists }
}

#[derive(Debug, Default, PartialEq, Eq)]
struct MockPath {
    string: &'static str,
    exists: bool,
}

impl PathLike for MockPath {
    fn as_path(&self) -> &Path {
        self.string.as_ref()
    }
    fn exists(&self) -> bool {
        self.exists
    }
    fn name(&self) -> &str {
        self.string
    }
}
