use anyhow::Result;
use clap::parser::ValuesRef;
use clap::ArgMatches;
use std::path::Path;

use crate::cli::output_path_parser::OutputPathParser;
use crate::junit_merger::config::Config;

#[derive(Debug)]
pub struct ApplicationRequest<'a> {
    pub paths: ValuesRef<'a, String>,
    pub output_path: Option<&'a Path>,
    pub indent_size: usize,
    pub indent_character: u8,
}

impl<'a> ApplicationRequest<'a> {
    pub fn from_matches(matches: &'a ArgMatches) -> Result<Option<Self>> {
        Ok(if let Some(paths) = matches.get_many::<String>("FILE") {
            let output_path = OutputPathParser::from_matches(matches).parse()?;
            let indent_size = *matches
                .get_one::<usize>("indent-size")
                .expect("--indent-size has a default value.");
            let indent_character: char = *matches
                .get_one::<char>("indent-character")
                .expect("--indent-character has a default value.");
            let indent_character = indent_character.try_into()?;
            Some(ApplicationRequest {
                paths,
                output_path,
                indent_size,
                indent_character,
            })
        } else {
            None
        })
    }

    pub fn config(&self) -> Config {
        Config {
            indent_char: self.indent_character,
            indent_size: self.indent_size,
        }
    }
}
