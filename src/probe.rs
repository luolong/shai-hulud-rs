use eros::AnyError;

use crate::scanner::DirEntry;
use std::path::{Path, PathBuf};
use std::result::Result;

pub mod check_file_hashes;
pub mod check_workflow_files;

// A marker trait for items that can be considered a "suspect" for a probe.
pub trait Suspect: Send + Sync + 'static {}

/// Type alias for fatal errors that stop probe scanning
pub type Error = Box<dyn AnyError>;

// The most common suspect is a simple file path.
impl Suspect for PathBuf {}

#[derive(Debug, Clone)]
pub enum Severity {
    LowRisk,
    MediumRisk,
    HighRisk,
}

#[derive(Debug)]
pub struct Finding {
    pub path: PathBuf,
    pub message: String,
    pub severity: Severity,
    pub payload: Option<Box<dyn Payload>>,
}

impl Finding {
    pub fn high_risk(path: &Path, message: &str) -> Self {
        Self::new(Severity::HighRisk, path, message)
    }

    pub fn medium_risk(path: &Path, message: &str) -> Self {
        Self::new(Severity::MediumRisk, path, message)
    }

    pub fn low_risk(path: &Path, message: &str) -> Self {
        Self::new(Severity::LowRisk, path, message)
    }

    fn new(severity: Severity, path: &Path, message: &str) -> Self {
        Self {
            path: path.to_path_buf(),
            message: message.to_string(),
            severity,
            payload: None,
        }
    }

    pub fn with_payload(mut self, payload: Box<dyn Payload>) -> Self {
        self.payload = Some(payload);
        self
    }
}

use std::fmt::Debug;

pub trait Payload: Send + Sync + Debug {
    // Maybe a method here in the future to summarize the payload for reporting
}

#[derive(Debug, Clone)]
pub struct Metadata {
    pub name: String,
    pub description: String,
    pub progress: ProgressMeta,
}

#[derive(Debug, Clone)]
pub struct ProgressMeta {
    pub prefix: String,
    pub message: String,
    pub finish: ProgressMetaFinish,
}

#[derive(Debug, Clone)]
pub enum ProgressMetaFinish {
    Clear,
    Abandon,
    WithMessage(&'static str),
}

/// Trait for a vulnerability probe.
///
/// Scanning vulnerabilities is performed in two passes:
/// 1. Marking entries to be scanned by the probe.
/// 2. Scanning the marked entries.
///
/// During the first pass of the scan, the `select` method is called, whose sole purpose is to select directory entries for a deeper vulnerability scan.
/// Select is performed during the initial directory tree traversal and has to be as cheap as possible, as the depth of the directory tree can be extremely
/// deep and number of entries traversed is virtually unbounded.
///
/// After the first pass, each probe is assumed to have selected entries to be scanned and then the `scan` method is called on each of the suspected entries.
/// All active probes will have their suspects scanned in parallel by the `Scanner`.
pub trait Probe: Send + Sync {
    /// The type of item this probe selects as a suspect for scanning.
    type Suspect: Suspect;

    /// The error type this probe can return from scanning.
    type Error: Send + Sync + 'static;

    /// Returns the human-readable name of the probe.
    fn name(&self) -> String {
        self.metadata().name.clone()
    }

    ///
    fn metadata(&self) -> Metadata;

    /// Mark the directory entry to be scanned.
    ///
    /// This method should be designed to be as efficient as possible, avoiding unnecessary computation if possible.
    /// Be as paranoid as possible, marking all files that might need to be scanned during the second pass.
    ///
    /// Return true if the entry was selected to be scanned by the probe, false otherwise.
    fn select(&mut self, entry: &DirEntry) -> bool;

    /// Performs the intensive scan on a single suspect item and returns any number of findings that are discovered.
    fn scan(&self, suspect: &Self::Suspect) -> Result<Vec<Finding>, Self::Error>;

    /// Returns a slice of the suspects collected by the probe during the selection pass.
    fn suspects(&self) -> &[Self::Suspect];
}
