use anyhow::anyhow;
use anyhow::Result;
use quick_xml::events::attributes::Attributes;
use quick_xml::events::BytesStart;
use quick_xml::events::Event;
use std::str::from_utf8;
use std::str::FromStr;

#[derive(Debug, Default, PartialEq)]
pub struct Testsuites {
    pub disabled: Option<usize>,
    pub errors: Option<usize>,
    pub failures: Option<usize>,
    pub name: Option<String>,
    pub tests: Option<usize>,
    pub time: Option<f64>,
}

trait SafeAdd: Sized {
    fn safe_add(self, other: Self) -> Result<Self>;
}

impl SafeAdd for f64 {
    fn safe_add(self, other: Self) -> Result<Self> {
        Ok(self + other)
    }
}

impl SafeAdd for usize {
    fn safe_add(self, other: Self) -> Result<Self> {
        self.checked_add(other).ok_or_else(|| {
            anyhow!(
                "Adding {} and {} caused overflow when parsing attribute.",
                self,
                other
            )
        })
    }
}

impl Testsuites {
    pub fn into_start_event(self) -> Event<'static> {
        fn to_tuple_iterator<'a>(
            name: &'a [u8],
            option: &'a Option<String>,
        ) -> impl Iterator<Item = (&'a [u8], &'a [u8])> {
            option.iter().map(move |val| (name, val.as_bytes()))
        }

        macro_rules! chain_with {
            ($($member:ident),+) => {
                chain_with!($(
                    to_tuple_iterator(
                        stringify!($member).as_bytes(), &self.$member.map(|val| ToString::to_string(&val))
                    )
                ),+)
            };
            ($iter:expr, $next:expr, $($tail:expr),+) => {
                chain_with!($iter.chain($next), $($tail),+)
            };
            ($iter:expr, $next:expr) => {
                $iter.chain($next)
            };
        }

        Event::Start(
            BytesStart::borrowed_name(b"testsuites")
                .with_attributes(chain_with!(disabled, errors, failures, name, tests, time)),
        )
    }

    fn sum_or_either<T: SafeAdd>(first: Option<T>, second: Option<T>) -> Result<Option<T>> {
        Ok(match (first, second) {
            (None, Some(x)) | (Some(x), None) => Some(x),
            (Some(x), Some(y)) => Some(x.safe_add(y)?),
            _ => None,
        })
    }

    pub fn from_attributes(attributes: Attributes) -> Result<Testsuites> {
        macro_rules! parse_attributes {
            ($($key:ident),+) => {
                {
                    let mut header = Testsuites::default();

                    for attribute in attributes {
                        let attribute = attribute?;
                        $(if stringify!($key).as_bytes() == attribute.key {
                                header = Testsuites { $key: Some(Testsuites::parse_attribute(&attribute.value)?), ..header };
                                continue
                        })+
                    }
                    Ok(header)
                }
            }
        }

        parse_attributes!(disabled, errors, failures, name, tests, time)
    }

    fn parse_attribute<T: FromStr>(attribute: &[u8]) -> Result<T>
    where
        <T as FromStr>::Err: std::error::Error + Send + Sync + 'static,
    {
        let utf8_value = from_utf8(attribute)?;
        Ok(utf8_value.parse()?)
    }
}

pub trait Merge: Sized {
    type Output;
    fn merge(self, other: Self) -> Self::Output;
}

impl Merge for Testsuites {
    type Output = Result<Self>;
    fn merge(self, other: Self) -> Self::Output {
        Ok(Testsuites {
            disabled: Testsuites::sum_or_either(self.disabled, other.disabled)?,
            errors: Testsuites::sum_or_either(self.errors, other.errors)?,
            failures: Testsuites::sum_or_either(self.failures, other.failures)?,
            name: self.name.or(other.name),
            tests: Testsuites::sum_or_either(self.tests, other.tests)?,
            time: Testsuites::sum_or_either(self.time, other.time)?,
        })
    }
}

impl Merge for Result<Testsuites> {
    type Output = Result<Testsuites>;
    fn merge(self, other: Self) -> Self::Output {
        match (self, other) {
            (Ok(suites), Ok(other_suites)) => suites.merge(other_suites),
            (Err(err), _) | (_, Err(err)) => Err(err),
        }
    }
}

#[cfg(test)]
#[path = "tests/testsuites.rs"]
mod tests;
