use anyhow::Result;
use clap::arg;
use clap::command;
use clap::AppSettings;
use clap::ArgAction;

use crate::cli::application_request::ApplicationRequest;
use crate::junit_merger::JunitMerger;

use super::output_writer::OutputWriter;

#[derive(Debug)]
pub struct Application<'help> {
    clap: clap::App<'help>,
}

impl<'help> Application<'help> {
    pub fn new() -> Self {
        let clap = command!()
            .arg(arg!(<FILE>... "Input file(s), should be valid JUnit XML format"))
            .arg(
                arg!(-o --output "Output file path, omit for STDOUT")
                    .takes_value(true)
                    .value_name("FILE"),
            )
            .arg(
                arg!(-f --force "No error if output file exists, overwrites content")
                    .action(ArgAction::SetTrue),
            )
            .setting(AppSettings::ArgRequiredElseHelp)
            .disable_colored_help(true);
        Self { clap }
    }

    pub fn run(self) -> Result<()> {
        if let Some(request) = ApplicationRequest::from_matches(&self.clap.get_matches())? {
            Self::run_with_request(request)?;
        }
        Ok(())
    }

    fn run_with_request(request: ApplicationRequest) -> Result<()> {
        let mut merger = JunitMerger::from_paths(request.paths)?;
        let result = merger.merge_into()?;
        OutputWriter::write(&result, request.output_path)
    }
}
