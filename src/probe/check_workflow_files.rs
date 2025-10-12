use crate::{
    probe::{Finding, Probe},
    scanner::DirEntry,
};

/// Detect malicious shai-hulud-workflow.yml files in project directories
pub struct CheckWorkflowFiles;

impl CheckWorkflowFiles {
    pub fn new() -> Box<Self> {
        Box::new(Self {})
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
    fn select(&self, entry: &DirEntry) -> bool {
        is_shai_hulud_workflow_file(entry)
    }

    fn scan(&self, entry: &DirEntry) -> eros::Result<Vec<super::Finding>> {
        Ok(vec![Finding::workflow_file(entry.path())])
    }
}
