use anyhow::bail;
use anyhow::Context;
use anyhow::Result;
use quick_xml::events::Event;

use crate::junit_merger::testsuites::Testsuites;

pub trait JunitReader {
    fn name(&self) -> &str;

    fn read_event<'a>(&mut self, buffer: &'a mut Vec<u8>) -> Result<Event<'a>>;

    fn read_trimmed_event<'a>(&mut self, buffer: &'a mut Vec<u8>) -> Result<Event<'a>>;

    fn read_until_testsuites(&mut self, buffer: &'_ mut Vec<u8>) -> Result<Testsuites> {
        if let Event::Decl(_) = self.read_trimmed_event(buffer)? {
            self.read_testsuites(buffer)
        } else {
            bail!("Required declaration <?xml version=\"1.0\" encoding=\"UTF-8\"?> was not at the top of xml content.")
        }
    }

    fn read_testsuites(&mut self, buffer: &'_ mut Vec<u8>) -> Result<Testsuites> {
        let result = match self.read_trimmed_event(buffer)? {
            Event::Start(tag) if tag.name() == b"testsuites" => {
                Testsuites::from_attributes(tag.attributes()).context("Parsing attributes.")?
            }
            _ => bail!(
                "Could not locate <testsuites> start tag within '{}'. Is it valid JUnit XML?",
                self.name()
            ),
        };

        buffer.clear();
        Ok(result)
    }
}
