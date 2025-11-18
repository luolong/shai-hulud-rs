use crate::probe::{Finding, Probe, Suspect};
use eros::{Context, bail};
use indicatif::{MultiProgress, ProgressBar, ProgressFinish, ProgressIterator};
use itertools::Itertools;
use jwalk::{Parallelism, WalkDirGeneric};
use num_cpus;
use rayon::{ThreadPoolBuilder, prelude::*};
use std::{
    ffi::OsStr,
    path::{Path, PathBuf},
    sync::Arc,
    time::Duration,
};

use std::ops::Deref;

pub(crate) struct DirEntry(jwalk::DirEntry<((), ())>);

impl Deref for DirEntry {
    type Target = jwalk::DirEntry<((), ())>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<jwalk::DirEntry<((), ())>> for DirEntry {
    fn from(entry: jwalk::DirEntry<((), ())>) -> Self {
        DirEntry(entry)
    }
}

pub(crate) struct Scanner {
    probes: Vec<Box<dyn Probe<Suspect = PathBuf>>>,
}

impl Scanner {
    pub fn with_probes(probes: Vec<Box<dyn Probe<Suspect = PathBuf>>>) -> Self {
        Scanner { probes }
    }

    pub(crate) fn scan<P: AsRef<Path> + Sync + Send>(
        &mut self,
        scan_dir: P,
        scan_message: impl Into<String> + Send,
        parallelism: Option<usize>,
    ) -> eros::Result<Vec<Finding>> {
        let num_threads = parallelism.unwrap_or_else(num_cpus::get);
        if num_threads == 0 {
            bail!("Parallelism cannot be set to zero");
        }

        let pool = ThreadPoolBuilder::new()
            .num_threads(num_threads)
            .build()
            .context("Failed to build thread pool")?;

        pool.install(|| {
            // --- Pass 1: Selection ---
            let scan_message = scan_message.into();
            let walk_style = crate::indikatif::spinners::point()
                .template("{msg:.blue} {spinner}")
                .unwrap();
            let walk_progress = ProgressBar::new_spinner();
            walk_progress.enable_steady_tick(Duration::from_millis(100));

            let walker =
                WalkDirGeneric::<((), ())>::new(&scan_dir).parallelism(if num_threads == 1 {
                    Parallelism::Serial
                } else {
                    Parallelism::RayonNewPool(num_threads)
                });

            let active_probe_indices: Vec<usize> = walker
                .into_iter()
                .progress_with(walk_progress)
                .with_prefix("📂")
                .with_message(scan_message.clone())
                .with_finish(ProgressFinish::AndLeave)
                .with_style(walk_style)
                .filter_map(|e| e.ok())
                .flat_map(|e| {
                    let entry = DirEntry::from(e);
                    self.probes
                        .iter_mut()
                        .enumerate()
                        .filter_map(|(index, probe)| {
                            if probe.select(&entry) {
                                Some(index)
                            } else {
                                None
                            }
                        })
                        .collect::<Vec<_>>()
                })
                .unique()
                .collect();

            // --- Pass 2: Analysis ---
            let multi_progress = MultiProgress::new();
            multi_progress.set_alignment(indicatif::MultiProgressAlignment::Bottom);

            let findings: Vec<Finding> = active_probe_indices
                .par_iter()
                .flat_map(|&index| {
                    let probe = &self.probes[index];
                    probe
                        .suspects()
                        .par_iter()
                        .map(move |suspect| match probe.scan(suspect) {
                            Ok(findings) => findings,
                            Err(_) => Vec::new(), // Decide on error handling
                        })
                        .flatten()
                })
                .collect();

            Ok(findings)
        })
    }
}
