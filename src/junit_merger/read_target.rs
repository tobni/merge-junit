use anyhow::Context;
use anyhow::Result;
use quick_xml::events::Event;
use quick_xml::Reader;
use std::collections::VecDeque;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;

use crate::junit_merger::junit_reader::JunitReader;

pub struct ReadTarget<'a, S: AsRef<str>, R: BufRead> {
    name: &'a S,
    reader: Reader<R>,
    staged_events: VecDeque<Event<'static>>,
}

impl<'a, S: AsRef<str>, R: BufRead> ReadTarget<'a, S, R> {
    fn read_event_from_reader<'b>(&mut self, buffer: &'b mut Vec<u8>) -> Result<Event<'b>> {
        self.staged_events
            .pop_front()
            .map_or_else(|| Ok(self.reader.read_event(buffer)?), Ok)
    }
    fn new(name: &'a S, mut reader: Reader<R>) -> Self {
        reader.trim_text(true);
        Self {
            name,
            reader,
            staged_events: VecDeque::default(),
        }
    }
}

impl<'a, S> ReadTarget<'a, S, BufReader<File>>
where
    S: AsRef<str> + AsRef<Path>,
{
    pub fn from_path(path: &'a S) -> Result<Self> {
        Ok(Self::new(
            path,
            Reader::from_file(path).context(format!(
                "Cannot read file '{}'.",
                <S as AsRef<str>>::as_ref(path)
            ))?,
        ))
    }
}

#[cfg(test)]
impl<'a, S: AsRef<str>> ReadTarget<'a, S, BufReader<&'a [u8]>> {
    pub fn from_buffer(name: &'a S, buffer: &'a [u8]) -> Self {
        Self::new(name, Reader::from_reader(BufReader::new(buffer)))
    }
}

impl<'a, P: AsRef<str>, R: BufRead> JunitReader for ReadTarget<'a, P, R> {
    fn name(&self) -> &str {
        self.name.as_ref()
    }

    fn stage_event(&mut self, event: Event<'static>) {
        self.staged_events.push_back(event);
    }

    fn read_event<'b>(&mut self, buffer: &'b mut Vec<u8>) -> Result<Event<'b>> {
        self.read_event_from_reader(buffer)
    }
}

#[cfg(test)]
#[path = "tests/read_target.rs"]
mod tests;
