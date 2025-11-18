mod check_file_hashes;
mod check_workflow_files;

use std::path::{Path, PathBuf};

pub use check_file_hashes::CheckFileHashes;
pub use check_workflow_files::CheckWorkflowFiles;
use indicatif::MultiProgress;

use crate::scanner::DirEntry;

pub(crate) enum Severity {
    LowRisk,
    MediumRisk,
    HighRisk,
}

pub(crate) struct Finding {
    path: PathBuf,
    message: String,
    severity: Severity,
    payload: Option<Box<dyn Payload + Send + Sync>>,
}

impl Finding {
    pub(crate) fn high_risk<T, M>(path: &T, message: &M) -> Self
    where
        T: AsRef<Path>,
        M: AsRef<str> + ?Sized,
    {
        Self {
            path: path.as_ref().to_owned(),
            message: message.as_ref().to_string(),
            severity: Severity::HighRisk,
            payload: None,
        }
    }
    pub(crate) fn medium_risk<T, M>(path: &T, message: &M) -> Self
    where
        T: AsRef<Path>,
        M: AsRef<str> + ?Sized,
    {
        Self {
            path: path.as_ref().to_owned(),
            message: message.as_ref().to_string(),
            severity: Severity::MediumRisk,
            payload: None,
        }
    }
    pub(crate) fn low_risk<T, M>(path: &T, message: &M) -> Self
    where
        T: AsRef<Path>,
        M: AsRef<str> + ?Sized,
    {
        Self {
            path: path.as_ref().to_owned(),
            message: message.as_ref().to_string(),
            severity: Severity::LowRisk,
            payload: None,
        }
    }

    pub(crate) fn with_payload(self, payload: Box<dyn Payload + Send + Sync>) -> Self {
        Self {
            path: self.path.clone(),
            message: self.message.clone(),
            severity: self.severity,
            payload: Some(payload),
        }
    }
}

pub(crate) trait Payload: Send + Sync {}

/// Trait for a vulnerability probe.
///
/// Scanning vulnerabilities is performed in two passes:
/// 1. Marking entries to be scanned by the probe.
/// 2. Scanning the marked entries.
///
/// During the first pass of the scan, the `select` method is called, whose sole purpose is to select directory entries to be scanned by the probe.
/// Select is performed during the initial directory tree traversal and has to be as cheap as possible, as the depth of the directory tree can be deep and number of entries traversed
/// is virtually unbounded.
///
/// After the first pass, each probe is assumed to have selected entries to be scanned and then the `scan` method is called to scan the all suspicious entries for vulnerabilities.
/// All probes that have been included in the scan, will be running in parallel to speed up the overall scanning process.
///
/// Each probe will submit asynchronous scanning tasks for the scanner to be executed in parallel by `Scanner`.
/// Each task yields zero or more diagnostic results, pointing at each individual vulnerability discovered.
pub(crate) trait Probe: Send + Sync {
    fn name(&self) -> String;

    /// Mark the directory entry to be scanned.
    ///
    /// This method should be designed to be as efficient as possible, avoiding unnecessary computation if possible.
    /// Be as paranoid as possible, marking all files that might need to be scanned during the second pass.
    ///
    /// Return true if the entry was selected to be scanned by the probe, false otherwise.
    fn select(&mut self, entry: &DirEntry) -> bool;

    /// Scan the selected entries for vulnerabilities.
    ///
    /// This method is called after the first pass of the scan, where all entries to be scanned have been selected.
    /// The method should scan all selected entries for vulnerabilities and return a vector of findings.
    ///
    /// The progress bar is provided to allow the probe to report progress during the scan.
    fn scan_suspects(&self, progress_bar: &MultiProgress) -> eros::Result<Vec<Finding>>;
}
