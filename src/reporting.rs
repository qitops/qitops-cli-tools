use crate::common::TestResult;
use crate::error::{Error, Result};
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ReportFormat {
    Json,
    Xml,
    Html,
    Csv,
}

impl ReportFormat {
    pub fn from_str(s: &str) -> Result<Self> {
        match s.to_lowercase().as_str() {
            "json" => Ok(ReportFormat::Json),
            "html" => Ok(ReportFormat::Html),
            "csv" => Ok(ReportFormat::Csv),
            "xml" => Ok(ReportFormat::Xml),
            _ => Err(Error::ValidationError(format!("Unsupported report format: {}", s))),
        }
    }
}

pub struct ReportGenerator {
    format: ReportFormat,
    output_path: PathBuf,
}

impl ReportGenerator {
    pub fn new(format: ReportFormat, output_path: PathBuf) -> Self {
        Self {
            format,
            output_path,
        }
    }

    pub fn generate(&self, results: &[TestResult]) -> Result<()> {
        match self.format {
            ReportFormat::Json => self.generate_json(results),
            ReportFormat::Xml => self.generate_xml(results),
            ReportFormat::Html => self.generate_html(results),
            ReportFormat::Csv => self.generate_csv(results),
        }
    }

    fn generate_json(&self, results: &[TestResult]) -> Result<()> {
        let json = serde_json::to_string_pretty(results)?;
        let mut file = File::create(&self.output_path)?;
        file.write_all(json.as_bytes())?;
        Ok(())
    }

    fn generate_xml(&self, results: &[TestResult]) -> Result<()> {
        let mut xml = String::from("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
        xml.push_str("<testsuites>\n");

        // Group results by test name
        let mut test_suites = std::collections::HashMap::new();
        for result in results {
            test_suites
                .entry(result.name.clone())
                .or_insert_with(Vec::new)
                .push(result);
        }

        for (suite_name, suite_results) in test_suites {
            let total_tests = suite_results.len();
            let failures = suite_results.iter().filter(|r| r.status == "failed").count();
            let errors = suite_results.iter().filter(|r| r.status == "error").count();
            let skipped = suite_results.iter().filter(|r| r.status == "skipped").count();
            let total_time: f64 = suite_results.iter().map(|r| r.duration).sum();

            xml.push_str(&format!(
                "  <testsuite name=\"{}\" tests=\"{}\" failures=\"{}\" errors=\"{}\" skipped=\"{}\" time=\"{:.2}\">\n",
                suite_name, total_tests, failures, errors, skipped, total_time
            ));

            for result in suite_results {
                xml.push_str(&format!(
                    "    <testcase name=\"{}\" time=\"{:.2}\">\n",
                    result.name, result.duration
                ));

                match result.status.as_str() {
                    "failed" => {
                        xml.push_str("      <failure>\n");
                        if let Some(details) = &result.details {
                            xml.push_str(&format!("        {}\n", serde_json::to_string(details)?));
                        }
                        xml.push_str("      </failure>\n");
                    }
                    "error" => {
                        xml.push_str("      <error>\n");
                        if let Some(details) = &result.details {
                            xml.push_str(&format!("        {}\n", serde_json::to_string(details)?));
                        }
                        xml.push_str("      </error>\n");
                    }
                    "skipped" => {
                        xml.push_str("      <skipped />\n");
                    }
                    _ => {}
                }

                xml.push_str("    </testcase>\n");
            }

            xml.push_str("  </testsuite>\n");
        }

        xml.push_str("</testsuites>");

        let mut file = File::create(&self.output_path)?;
        file.write_all(xml.as_bytes())?;

        Ok(())
    }

    fn generate_html(&self, results: &[TestResult]) -> Result<()> {
        let mut html = String::from("<!DOCTYPE html>\n<html>\n<head>\n");
        html.push_str("  <title>QitOps Test Report</title>\n");
        html.push_str("  <style>\n");
        html.push_str("    body { font-family: Arial, sans-serif; margin: 20px; }\n");
        html.push_str("    h1 { color: #333; }\n");
        html.push_str("    .summary { margin-bottom: 20px; }\n");
        html.push_str("    .test-result { margin-bottom: 10px; border: 1px solid #ddd; padding: 10px; }\n");
        html.push_str("    .passed { background-color: #dff0d8; }\n");
        html.push_str("    .failed { background-color: #f2dede; }\n");
        html.push_str("    .error { background-color: #fcf8e3; }\n");
        html.push_str("    .skipped { background-color: #d9edf7; }\n");
        html.push_str("    .details { margin-top: 10px; font-family: monospace; white-space: pre-wrap; }\n");
        html.push_str("  </style>\n");
        html.push_str("</head>\n<body>\n");
        html.push_str("  <h1>QitOps Test Report</h1>\n");

        // Summary
        let total = results.len();
        let passed = results.iter().filter(|r| r.status == "passed").count();
        let failed = results.iter().filter(|r| r.status == "failed").count();
        let errors = results.iter().filter(|r| r.status == "error").count();
        let skipped = results.iter().filter(|r| r.status == "skipped").count();

        html.push_str("  <div class=\"summary\">\n");
        html.push_str(&format!("    <p>Total Tests: {}</p>\n", total));
        html.push_str(&format!("    <p>Passed: {}</p>\n", passed));
        html.push_str(&format!("    <p>Failed: {}</p>\n", failed));
        html.push_str(&format!("    <p>Errors: {}</p>\n", errors));
        html.push_str(&format!("    <p>Skipped: {}</p>\n", skipped));
        html.push_str("  </div>\n");

        // Test Results
        html.push_str("  <h2>Test Results</h2>\n");

        for result in results {
            let status_class = match result.status.as_str() {
                "passed" => "passed",
                "failed" => "failed",
                "error" => "error",
                "skipped" => "skipped",
                _ => "",
            };

            html.push_str(&format!("  <div class=\"test-result {}\">\n", status_class));
            html.push_str(&format!("    <h3>{}</h3>\n", result.name));
            html.push_str(&format!("    <p>Status: {}</p>\n", result.status));
            html.push_str(&format!("    <p>Duration: {:.2}s</p>\n", result.duration));
            html.push_str(&format!("    <p>Timestamp: {}</p>\n", result.timestamp));

            if let Some(details) = &result.details {
                html.push_str("    <div class=\"details\">\n");
                html.push_str(&format!("      <pre>{}</pre>\n", serde_json::to_string_pretty(details)?));
                html.push_str("    </div>\n");
            }

            html.push_str("  </div>\n");
        }

        html.push_str("</body>\n</html>");

        let mut file = File::create(&self.output_path)?;
        file.write_all(html.as_bytes())?;

        Ok(())
    }

    fn generate_csv(&self, results: &[TestResult]) -> Result<()> {
        let mut csv = String::from("Name,Status,Duration,Timestamp\n");

        for result in results {
            csv.push_str(&format!(
                "\"{}\",\"{}\",{:.2},\"{}\"\n",
                result.name, result.status, result.duration, result.timestamp
            ));
        }

        let mut file = File::create(&self.output_path)?;
        file.write_all(csv.as_bytes())?;

        Ok(())
    }
}
