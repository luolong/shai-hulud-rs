use std::{
    ffi::OsStr,
    path::{Path, PathBuf},
};

use indicatif::{ProgressBar, ProgressFinish, ProgressIterator, ProgressStyle};
use jwalk::{Parallelism, WalkDirGeneric};

use crate::probe::{Finding, Probe};

const TICK_CHARS: &str = "🕐🕑🕒🕓🕔🕕🕖🕗🕘🕙🕚🕛🔍";

#[derive(Debug, Clone, Default)]
pub struct Marker(Vec<usize>);

impl Marker {
    pub(crate) fn mark(&mut self, index: usize) {
        self.0.push(index);
    }

    pub(crate) fn markers(&self) -> Vec<usize> {
        self.0.clone()
    }
}

#[derive(Debug)]
pub struct DirEntry(jwalk::DirEntry<((), Marker)>);

impl From<jwalk::DirEntry<((), Marker)>> for DirEntry {
    fn from(entry: jwalk::DirEntry<((), Marker)>) -> Self {
        DirEntry(entry)
    }
}

impl DirEntry {
    pub(crate) fn path(&self) -> PathBuf {
        self.0.path()
    }

    pub(crate) fn file_type(&self) -> std::fs::FileType {
        self.0.file_type()
    }

    pub(crate) fn file_name(&self) -> &OsStr {
        self.0.file_name()
    }

    pub(crate) fn mark(&mut self, index: usize) {
        let _ = &self.0.client_state.mark(index);
    }

    pub(crate) fn select<'a>(
        &self,
        probes: &'a [Box<dyn Probe>],
    ) -> Vec<(&DirEntry, &'a Box<dyn Probe>)> {
        let markers = self.0.client_state.markers();
        markers
            .into_iter()
            .filter_map(|index| probes.get(index).map(|probe| (self, probe)))
            .collect()
    }
}

pub(crate) struct Scanner {
    probes: Vec<Box<dyn Probe>>,
}

impl Scanner {
    pub fn with_probes(probes: Vec<Box<dyn Probe>>) -> Self {
        Scanner { probes }
    }

    pub(crate) fn scan<P: AsRef<Path>>(
        &self,
        scan_dir: P,
        scan_message: impl Into<String>,
        parallelism: Option<usize>,
    ) -> eros::Result<Vec<Finding>> {
        let walker = WalkDirGeneric::<((), Marker)>::new(&scan_dir);

        let walker = match parallelism {
            Some(1) => walker.parallelism(Parallelism::Serial),
            Some(p) => walker.parallelism(Parallelism::RayonNewPool(p)),
            None => walker,
        };

        let scan_message = scan_message.into();
        let walk_style = ProgressStyle::default_spinner()
            .tick_chars(TICK_CHARS)
            .template("{spinner} {msg:.blue}")
            .unwrap();

        let marked: Vec<DirEntry> = walker
            .into_iter()
            .progress_with(ProgressBar::new_spinner())
            .with_message(scan_message.clone())
            .with_finish(ProgressFinish::AndLeave)
            .with_style(walk_style)
            .filter_map(|e| e.ok())
            .filter_map(|e| {
                let mut entry = DirEntry::from(e);
                let mut marked = false;
                for (index, probe) in self.probes.iter().enumerate() {
                    if probe.select(&entry) {
                        entry.mark(index);
                        marked = true;
                    }
                }

                if marked { Some(entry) } else { None }
            })
            .collect();

        println!("Marked {} files for scanning", marked.len());

        let mut findings = Vec::new();
        for (entry, probe) in marked.iter().flat_map(|e| e.select(&self.probes)) {
            let mut f = probe.scan(&entry)?;
            findings.append(&mut f);
        }

        Ok(findings)
    }
}
