mod check_workflow_files;

pub use check_workflow_files::CheckWorkflowFiles;

use std::{fmt::Display, path::PathBuf};

use crate::scanner::DirEntry;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum FindingSeverity {
    LowRisk,
    MediumRisk,
    HighRisk,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum FindingKind {
    WorkflowFile,
}

/// Represents a finding of a vulnerability scan.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct Finding {
    severity: FindingSeverity,
    path: PathBuf,
    kind: FindingKind,
    message: String,
}

impl Finding {
    pub fn workflow_file(path: PathBuf) -> Self {
        Self {
            path,
            severity: FindingSeverity::HighRisk,
            kind: FindingKind::WorkflowFile,
            message: "Known malicious workflow filename".to_string(),
        }
    }
}

impl Finding {
    pub(crate) fn severity(&self) -> &FindingSeverity {
        &self.severity
    }
}

impl Display for Finding {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

/// Trait for a vulnerability probe.
///
/// Scanning vulnerabilities is performed in two passes:
/// 1. Marking entries to be scanned.
/// 2. Scanning the marked entries.
///
/// In the first pass, the `mark` method is called, that will mark each entry as suspicious to be scanned in a second pass.
/// The first pass mark function must be as cheap as possible and its main function is to narrow down number of entries to be scanned during the second pass.
///
/// In the second pass, the `scan` method is called that will return
pub(crate) trait Probe {
    /// Mark the directory entry to be scanned.
    ///
    /// This method should be designed to be as efficient as possible, avoiding unnecessary computation if possible.
    /// Be as paranoid as possible, marking all files that might need to be scanned during the second pass.
    fn select(&self, entry: &DirEntry) -> bool;

    /// Scan the marked directory entry.
    ///
    /// This method scans the marked directory entry and returns a `Result` indicating whether the entry is vulnerable.
    fn scan(&self, entry: &DirEntry) -> eros::Result<Vec<Finding>>;
}
