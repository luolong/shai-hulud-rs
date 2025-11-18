use crate::report::{Report, Reporter};
use console::style;

pub struct ConsoleReporter;

impl ConsoleReporter {
    pub fn new() -> Self {
        Self
    }
}

impl Reporter for ConsoleReporter {
    fn report(&self, report: &Report) -> eros::Result<()> {
        println!();
        println!(
            "{}",
            style("==============================================").blue()
        );
        println!("{}", style("      SHAI-HULUD DETECTION REPORT").blue());
        println!(
            "{}",
            style("==============================================").blue()
        );
        println!();

        if report.results.is_empty() {
            println!(
                "{}",
                style("✅ No indicators of compromise detected.").green()
            );
            return Ok(());
        }

        for result in &report.results {
            println!(
                "{}",
                style(format!(
                    "{} found {} issues:",
                    result.probe_name,
                    result.findings.len()
                ))
                .bold()
            );
            for finding in &result.findings {
                let severity_style = match finding.severity {
                    crate::probe::Severity::HighRisk => style("HIGH RISK").red().bold(),
                    crate::probe::Severity::MediumRisk => style("MEDIUM RISK").yellow(),
                    crate::probe::Severity::LowRisk => style("LOW RISK").cyan(),
                };
                println!(
                    "  - [{}]: {} at {}",
                    severity_style,
                    finding.message,
                    finding.path.display()
                );
            }
            println!();
        }

        println!(
            "{}",
            style("==============================================").blue()
        );
        println!("{}", style("🔍 SUMMARY:").red());
        println!(
            "   {} High Risk Issues",
            style(report.stats.high_risk.to_string()).red()
        );
        println!(
            "   {} Medium Risk Issues",
            style(report.stats.medium_risk.to_string()).yellow()
        );
        println!(
            "   {} Low Risk Issues",
            style(report.stats.low_risk.to_string()).cyan()
        );
        println!(
            "{}",
            style("==============================================").blue()
        );

        Ok(())
    }
}
