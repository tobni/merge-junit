use anyhow::Context;
use anyhow::Result;
use quick_xml::events::BytesEnd;
use quick_xml::events::Event;
use quick_xml::Writer;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

pub mod junit_reader;
mod read_target;
mod testsuites;

use junit_reader::JunitReader;
use read_target::ReadTarget;
use testsuites::{Merge, Testsuites};

#[derive(Debug)]
pub struct JunitMerger<T: JunitReader> {
    readers: Vec<T>,
}

impl<T: JunitReader> JunitMerger<T> {
    pub fn new(readers: Vec<T>) -> Self {
        Self { readers }
    }

    pub fn merge_into(&mut self) -> Result<Vec<u8>> {
        let mut xml_writer = Writer::new_with_indent(Vec::from(
            b"<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n".as_slice(),
        ), ' '.try_into()?, 3);

        let mut buf = Vec::new();

        xml_writer.write_event(self.create_start_event(&mut buf)?)?;
        for xml_reader in &mut self.readers {
            'read: loop {
                match xml_reader.read_trimmed_event(&mut buf)? {
                    Event::End(tag) if tag.name() == b"testsuites" => break 'read,
                    Event::Eof => break 'read,
                    event => xml_writer.write_event(event)?,
                }
                buf.clear();
            }
        }
        xml_writer.write_event(Event::End(BytesEnd::borrowed(b"testsuites")))?;
        xml_writer.write_event(Event::Eof)?;
        Ok(xml_writer.into_inner())
    }

    fn create_start_event(&mut self, buffer: &mut Vec<u8>) -> Result<Event<'static>> {
        let mut testsuites_headers = self
            .readers
            .iter_mut()
            .map(|reader| {
                reader
                    .read_until_testsuites(buffer)
                    .context("Deserializing header tags.")
            })
            .filter_map(Result::transpose);
        let header = {
            if let Some(init) = testsuites_headers.next() {
                testsuites_headers.fold(init, Result::<Testsuites>::merge)
            } else {
                Ok(Testsuites::default())
            }
        }?;
        Ok(header.into_start_event())
    }
}

impl<'a, S: AsRef<str> + AsRef<Path> + 'a> JunitMerger<ReadTarget<'a, S, BufReader<File>>> {
    pub fn from_paths(paths: impl IntoIterator<Item = &'a S>) -> Result<Self> {
        let readers: Result<Vec<_>> = paths
            .into_iter()
            .map(|path| ReadTarget::from_path(path))
            .collect();
        readers.map(Self::new)
    }
}
