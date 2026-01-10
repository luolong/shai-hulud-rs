use crate::probe::Severity;
use crate::scanner::ProbeFindings;
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
    findings: Vec<ProbeResult>,
}

impl ReportBuilder {
    pub fn new() -> Self {
        Self {
            findings: Vec::new(),
        }
    }

    pub fn add_probe_findings(&mut self, probe_findings: Vec<ProbeFindings>) {
        for pf in probe_findings {
            let findings = pf
                .findings
                .into_iter()
                .map(|f| ReportableFinding {
                    path: f.path,
                    message: f.message,
                    severity: f.severity,
                })
                .collect();

            self.findings.push(ProbeResult {
                probe_name: pf.probe_name,
                findings,
            });
        }
    }

    pub fn build(self) -> Report {
        let mut stats = Stats::default();
        for result in &self.findings {
            for finding in &result.findings {
                stats.inc(&finding.severity);
            }
        }
        Report {
            stats,
            results: self.findings,
        }
    }
}

/// Trait for a report generator.
pub trait Reporter {
    fn report(&self, report: &Report) -> eros::Result<()>;
}
