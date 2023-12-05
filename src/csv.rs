use std::path::Path;

use color_eyre::eyre::{Result, WrapErr};
use serde_derive::Serialize;

use crate::vale::{Alert, Alerts, Severity};

#[derive(Debug, Serialize)]
struct CsvEntry<'a> {
    pub file: &'a Path,
    pub severity: Severity,
    pub line: u64,
    pub span: String,
    pub r#match: &'a str,
    pub message: &'a str,
    pub check: &'a str,
}

impl<'a> Alert {
    fn as_entry(&'a self, file: &'a Path) -> CsvEntry<'a> {
        CsvEntry {
            file,
            severity: self.severity,
            line: self.line,
            span: format!("{}–{}", self.span.0, self.span.1),
            r#match: &self.r#match,
            message: &self.message,
            check: &self.check,
        }
    }
}

pub fn table(alerts: &Alerts, out_file: &Path) -> Result<()> {
    let mut wtr =
        csv::Writer::from_path(out_file).wrap_err("Failed to write to the CSV output file.")?;

    for (file, alerts_list) in &alerts.0 {
        for alert in alerts_list {
            let entry = alert.as_entry(file);
            wtr.serialize(entry)
                .wrap_err("Failed to serialize a CSV row.")?;
        }
    }
    Ok(())
}
