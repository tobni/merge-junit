use crate::junit_merger::testsuites::Testsuites;

use super::*;

#[test]
fn can_read_testsuites_start_tag() -> Result<()> {
    let mut read_target =
        given_read_target(b"<?xml version=\"1.0\" encoding=\"UTF-8\"?><testsuites>");

    let testsuites = read_target.read_until_testsuites(&mut vec![])?;

    assert_eq!(testsuites, Testsuites::default());
    Ok(())
}

#[test]
fn can_read_testsuites_start_tag_with_attributes() -> Result<()> {
    let mut read_target = given_read_target(
        b"<?xml version=\"1.0\" encoding=\"UTF-8\"?><testsuites name=\"My Module\">",
    );

    let testsuites = read_target.read_until_testsuites(&mut vec![])?;

    let expected = Testsuites {
        name: Some("My Module".into()),
        ..Default::default()
    };

    assert_eq!(testsuites, expected);
    Ok(())
}

#[test]
fn content_missing_xml_decl_is_an_error() -> Result<()> {
    let mut read_target = given_read_target(b"<testsuites>");

    let testsuites = read_target.read_until_testsuites(&mut vec![]);

    assert!(testsuites.is_err());
    assert_eq!(testsuites.unwrap_err().to_string(), "Required declaration <?xml version=\"1.0\" encoding=\"UTF-8\"?> was not at the top of xml content.");
    Ok(())
}

fn given_read_target<'a>(input: &'static [u8]) -> ReadTarget<&'static str, BufReader<&'a [u8]>> {
    ReadTarget::from_buffer(&"My JunitXml", input)
}
