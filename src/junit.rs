use std::convert::From;
use std::path::Path;

use junit_report::{
    Duration, Report, ReportBuilder, TestCase, TestCaseBuilder, TestSuite,
};

use crate::vale::{Alert, Alerts, Severity};

impl Alert {
    fn to_testcase(&self, filename: &Path) -> TestCase {
        let readable_path = filename.display().to_string();
        TestCaseBuilder::failure(
            &self.main_description(),
            Duration::seconds(0),
            self.severity.as_str(),
            &self.details(),
        )
        .set_filepath(&readable_path)
        .build()
    }
}

struct ValeSuites {
    errors: TestSuite,
    warnings: TestSuite,
    suggestions: TestSuite,
}

impl From<Alerts> for ValeSuites {
    fn from(item: Alerts) -> Self {
        let hashmap = item.0;

        let mut err_suite = TestSuite::new("Errors");
        let mut warn_suite = TestSuite::new("Warnings");
        let mut sug_suite = TestSuite::new("Suggestions");

        for (file, alerts) in &hashmap {
            for alert in alerts {
                let suite = match alert.severity {
                    Severity::Suggestion => &mut sug_suite,
                    Severity::Warning => &mut warn_suite,
                    Severity::Error => &mut err_suite,
                };
                suite.add_testcase(alert.to_testcase(file));
            }
        }

        ValeSuites {
            errors: err_suite,
            warnings: warn_suite,
            suggestions: sug_suite,
        }
    }
}

impl From<ValeSuites> for Report {
    fn from(item: ValeSuites) -> Self {
        ReportBuilder::new()
            .add_testsuite(item.errors)
            .add_testsuite(item.warnings)
            .add_testsuite(item.suggestions)
            .build()
    }
}

impl From<Alerts> for Report {
    fn from(item: Alerts) -> Self {
        let suites: ValeSuites = item.into();
        let report: Report = suites.into();

        report
    }
}

pub fn report(alerts: Alerts) -> Report {
    alerts.into()
}
