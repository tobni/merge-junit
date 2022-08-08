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
            .arg(
                clap::Arg::new("indent-size")
                    .long("indent-size")
                    .short('s')
                    .help("Number of indentation characters")
                    .takes_value(true)
                    .value_name("INT")
                    .value_parser(clap::value_parser!(usize))
                    .default_value("3"),
            )
            .arg(
                clap::Arg::new("indent-character")
                    .long("indent-character")
                    .short('c')
                    .help("Whitespace character to use for indentation")
                    .takes_value(true)
                    .value_name("CHAR")
                    .value_parser(clap::value_parser!(char))
                    .default_value(" "),
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
        let config = request.config();
        let mut merger = JunitMerger::from_paths_and_config(request.paths, config)?;
        let result = merger.merge_into()?;
        OutputWriter::write(&result, request.output_path)
    }
}
