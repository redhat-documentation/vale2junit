use std::convert::From;
use std::path::Path;

use junit_report::{
    Duration, Report, ReportBuilder, TestCase, TestCaseBuilder, TestSuite, TestSuiteBuilder,
};

use crate::vale::{Alert, Alerts, Severity};

impl Alert {
    fn into_testcase(&self, filename: &Path) -> TestCase {
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
    suggestions: TestSuite,
    warnings: TestSuite,
    errors: TestSuite,
}

impl From<Alerts> for ValeSuites {
    fn from(item: Alerts) -> Self {
        let hm = item.0;

        let mut sug_suite = TestSuiteBuilder::new("Suggestions");
        let mut warn_suite = TestSuiteBuilder::new("Warnings");
        let mut err_suite = TestSuiteBuilder::new("Errors");

        for (file, alerts) in hm.iter() {
            for alert in alerts {
                let suite = match alert.severity {
                    Severity::Suggestion => &mut sug_suite,
                    Severity::Warning => &mut warn_suite,
                    Severity::Error => &mut err_suite,
                };
                suite.add_testcase(alert.into_testcase(file));
            }
        }

        ValeSuites {
            suggestions: sug_suite.build(),
            warnings: warn_suite.build(),
            errors: err_suite.build(),
        }
    }
}

impl From<ValeSuites> for Report {
    fn from(item: ValeSuites) -> Self {
        ReportBuilder::new()
            .add_testsuite(item.suggestions)
            .add_testsuite(item.warnings)
            .add_testsuite(item.errors)
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

pub fn junit_report(alerts: Alerts) -> Report {
    alerts.into()
}
