use std::{path::PathBuf, time::Duration};

use indicatif::{ParallelProgressIterator, ProgressBar};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use crate::{
    probe::{Finding, Probe},
    scanner::DirEntry,
};

/// Detect malicious shai-hulud-workflow.yml files in project directories
pub struct CheckWorkflowFiles {
    suspects: Vec<PathBuf>,
}

impl CheckWorkflowFiles {
    pub fn new() -> Box<Self> {
        Box::new(Self {
            suspects: Vec::new(),
        })
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
    fn name(&self) -> String {
        "Checking for malicious workflow files".to_string()
    }

    fn select(&mut self, entry: &DirEntry) -> bool {
        if is_shai_hulud_workflow_file(entry) {
            self.suspects.push(entry.path().to_owned());
            true
        } else {
            false
        }
    }

    fn scan_suspects(
        &self,
        multi_progress: &indicatif::MultiProgress,
    ) -> eros::Result<Vec<Finding>> {
        let progress_bar = multi_progress.add(
            ProgressBar::new_spinner()
                .with_prefix("🔍")
                .with_message("Checking for malicious workflow files")
                .with_finish(indicatif::ProgressFinish::WithMessage(
                    std::borrow::Cow::Owned(
                        "Finished checking for malicious workflow files.".to_string(),
                    ),
                ))
                .with_style(
                    crate::indikatif::spinners::sand()
                        .template("{prefix}{spinner}{wide_msg}({pos}/{len})")
                        .unwrap(),
                ),
        );
        progress_bar.enable_steady_tick(Duration::from_millis(100));
        progress_bar.set_length(self.suspects.len() as u64);

        let finsdings = self
            .suspects
            .par_iter()
            .progress_with(progress_bar)
            .map(|path| Finding::high_risk(path, "Malicious workflow file detected"))
            .collect();

        Ok(finsdings)
    }
}
