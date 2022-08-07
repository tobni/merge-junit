use anyhow::Context;
use anyhow::Result;
use quick_xml::events::Event;
use quick_xml::Reader;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;

use crate::junit_merger::junit_reader::JunitReader;

pub struct ReadTarget<'a, S: AsRef<str>, R: BufRead> {
    name: &'a S,
    reader: Reader<R>,
}

impl<'a, S> ReadTarget<'a, S, BufReader<File>>
where
    S: AsRef<str> + AsRef<Path>,
{
    pub fn from_path(path: &'a S) -> Result<Self> {
        Ok(Self {
            name: path,
            reader: Reader::from_file(path).context(format!(
                "Cannot read file '{}'.",
                <S as AsRef<str>>::as_ref(path)
            ))?,
        })
    }
}

#[cfg(test)]
impl<'a, S: AsRef<str>> ReadTarget<'a, S, BufReader<&'a [u8]>> {
    pub fn from_buffer(name: &'a S, buffer: &'a [u8]) -> Self {
        Self {
            name,
            reader: Reader::from_reader(BufReader::new(buffer)),
        }
    }
}

impl<'a, P: AsRef<str>, R: BufRead> JunitReader for ReadTarget<'a, P, R> {
    fn name(&self) -> &str {
        self.name.as_ref()
    }

    fn read_event<'b>(&mut self, buffer: &'b mut Vec<u8>) -> Result<Event<'b>> {
        Ok(self.reader.trim_text(false).read_event(buffer)?)
    }

    fn read_trimmed_event<'b>(&mut self, buffer: &'b mut Vec<u8>) -> Result<Event<'b>> {
        Ok(self.reader.trim_text(true).read_event(buffer)?)
    }
}

#[cfg(test)]
#[path = "tests/read_target.rs"]
mod tests;
