use anyhow::Result;
use clap::parser::ValuesRef;
use clap::ArgMatches;
use std::path::Path;

use crate::cli::output_path_parser::OutputPathParser;

#[derive(Debug)]
pub struct ApplicationRequest<'a> {
    pub paths: ValuesRef<'a, String>,
    pub output_path: Option<&'a Path>,
}

impl<'a> ApplicationRequest<'a> {
    pub fn from_matches(matches: &'a ArgMatches) -> Result<Option<Self>> {
        Ok(if let Some(paths) = matches.get_many::<String>("FILE") {
            let output_path = OutputPathParser::from_matches(matches).parse()?;
            Some(ApplicationRequest { paths, output_path })
        } else {
            None
        })
    }
}
