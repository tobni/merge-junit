use super::*;

#[test]
fn test_can_parse_from_attributes() -> Result<()> {
    let attributes = Attributes::new("testsuites name=\"blark\" tests=\"2\"", 11);
    let testsuites = Testsuites::from_attributes(attributes)?;

    let expected = Testsuites {
        name: Some("blark".to_string()),
        tests: Some(2),
        ..Default::default()
    };

    assert_eq!(testsuites, expected);
    Ok(())
}

macro_rules! attribute_add_tests {
    ($([$test_name:ident, $attribute:ident, $left:expr, $right:expr, $expected:expr]),+) => {
        $(#[test]
        fn $test_name() -> Result<()> {
            let left = Testsuites {$attribute: $left.into(), ..Default::default()};
            let right = Testsuites {$attribute: $right.into(), ..Default::default()};

            let expected = Testsuites {
                $attribute: $expected.into(),
                ..Default::default()
            };

            assert_eq!(left.merge(right)?, expected);
            Ok(())
        })+
    };
}

attribute_add_tests! {
        [test_merging_testsuites_adds_tests_attribute, tests, 1, 2, 3],
        [test_merging_testsuites_picks_available_tests_from_right_attribute, tests, None, 2, 2],
        [test_merging_testsuites_picks_available_tests_from_left_attribute, tests, 1, None, 1],
        [test_merging_testsuites_adds_disabled_attribute, disabled, 1, 2, 3],
        [test_merging_testsuites_picks_available_disabled_from_right_attribute, disabled, None, 2, 2],
        [test_merging_testsuites_picks_available_disabled_from_left_attribute, disabled, 1, None, 1],
        [test_merging_testsuites_adds_failures_attribute, failures, 1, 2, 3],
        [test_merging_testsuites_picks_available_failures_from_right_attribute, failures, None, 2, 2],
        [test_merging_testsuites_picks_available_failures_from_left_attribute, failures, 1, None, 1],
        [test_merging_testsuites_adds_errors_attribute, errors, 1, 2, 3],
        [test_merging_testsuites_picks_available_errors_from_right_attribute, errors, None, 2, 2],
        [test_merging_testsuites_picks_available_errors_from_left_attribute, errors, 1, None, 1]
}
