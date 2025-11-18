use crate::probe::{Finding, Severity};
use std::collections::HashMap;
use std::path::PathBuf;

pub mod console;

#[derive(Debug, Default, Clone)]
pub struct Stats {
    pub high_risk: usize,
    pub medium_risk: usize,
    pub low_risk: usize,
}

impl Stats {
    fn inc(&mut self, severity: &Severity) {
        match severity {
            Severity::LowRisk => self.low_risk += 1,
            Severity::MediumRisk => self.medium_risk += 1,
            Severity::HighRisk => self.high_risk += 1,
        }
    }
}

#[derive(Debug)]
pub struct ReportableFinding {
    pub path: PathBuf,
    pub message: String,
    pub severity: Severity,
    // pub payload_summary: Option<String>,
}

#[derive(Debug)]
pub struct ProbeResult {
    pub probe_name: String,
    pub findings: Vec<ReportableFinding>,
}

#[derive(Debug)]
pub struct Report {
    pub stats: Stats,
    pub results: Vec<ProbeResult>,
}

pub struct ReportBuilder {
    findings: Vec<Finding>,
}

impl ReportBuilder {
    pub fn new() -> Self {
        Self {
            findings: Vec::new(),
        }
    }

    pub fn add_findings(&mut self, findings: Vec<Finding>) {
        self.findings.extend(findings);
    }

    pub fn build(self) -> Report {
        let mut stats = Stats::default();
        let mut grouped: HashMap<String, Vec<ReportableFinding>> = HashMap::new();

        for finding in self.findings {
            stats.inc(&finding.severity);
            let reportable = ReportableFinding {
                path: finding.path,
                message: finding.message,
                severity: finding.severity,
                // payload_summary: finding.payload.map(|p| p.summary()),
            };
            grouped
                .entry(finding.probe_name)
                .or_default()
                .push(reportable);
        }

        let results = grouped
            .into_iter()
            .map(|(probe_name, findings)| ProbeResult {
                probe_name,
                findings,
            })
            .collect();

        Report { stats, results }
    }
}

/// Trait for a report generator.
pub trait Reporter {
    fn report(&self, report: &Report) -> eros::Result<()>;
}
