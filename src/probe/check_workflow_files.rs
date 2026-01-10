use crate::{
    probe::{Error, Finding, Probe},
    scanner::DirEntry,
};
use std::convert::Infallible;
use std::path::PathBuf;
use std::result::Result;

/// Detect malicious shai-hulud-workflow.yml files in project directories
pub struct CheckWorkflowFiles {
    suspects: Vec<PathBuf>,
}

impl CheckWorkflowFiles {
    pub fn new() -> Self {
        Self {
            suspects: Vec::new(),
        }
    }
}

fn is_shai_hulud_workflow_file(entry: &DirEntry) -> bool {
    entry.file_type().is_file()
        && entry
            .file_name()
            .to_string_lossy()
            .eq("shai-hulud-workflow.yml")
}

impl Probe for CheckWorkflowFiles {
    type Suspect = PathBuf;
    type Error = Infallible;

    fn name(&self) -> String {
        "Checking for malicious workflow files".to_string()
    }

    fn select(&mut self, entry: &DirEntry) -> bool {
        if is_shai_hulud_workflow_file(entry) {
            self.suspects.push(entry.path());
            true
        } else {
            false
        }
    }

    fn scan(&self, suspect: &Self::Suspect) -> Result<Vec<Finding>, Self::Error> {
        Ok(vec![Finding::high_risk(
            suspect,
            "Malicious workflow file detected",
        )])
    }

    fn suspects(&self) -> &[Self::Suspect] {
        &self.suspects
    }
}

/// Infallible errors never convert to findings (but also never occur)
impl TryFrom<Infallible> for Finding {
    type Error = Error;

    fn try_from(_: Infallible) -> Result<Finding, Error> {
        unreachable!("Infallible errors cannot occur")
    }
}
