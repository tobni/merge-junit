use anyhow::Result;
use std::io::stdout;
use std::io::Write;
use std::path::Path;

#[derive(Debug)]
pub struct OutputWriter;

impl OutputWriter {
    pub fn write(result: &[u8], path: Option<&Path>) -> Result<()> {
        if let Some(path) = path {
            std::fs::write(path, result)?;
        } else {
            stdout().lock().write_all(result)?;
        }
        Ok(())
    }
}
