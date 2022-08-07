use crate::cli::path_like::PathLike;
use anyhow::anyhow;
use anyhow::Result;
use clap::ArgMatches;
use std::path::Path;

#[derive(Debug)]
pub struct OutputPathParser<'a, P: PathLike> {
    path: Option<&'a P>,
    force: bool,
}

impl<'a, P: PathLike> OutputPathParser<'a, P> {
    pub fn parse(self) -> Result<Option<&'a Path>> {
        match (self.path, self.force) {
            (Some(path), false) if path.exists() => Err(anyhow!(
                "File '{}' exists. Set '--force' flag to overwrite.",
                path.name()
            )),
            (Some(path), _) => Ok(Some(path.as_path())),
            (None, _) => Ok(None),
        }
    }
}

impl<'a> OutputPathParser<'a, String> {
    pub fn from_matches(matches: &'a ArgMatches) -> Self {
        let path = matches.get_one("output");
        let force: bool = *matches.get_one("force").unwrap();
        Self { path, force }
    }
}

#[cfg(test)]
#[path = "tests/output_path_parser.rs"]
mod tests;
