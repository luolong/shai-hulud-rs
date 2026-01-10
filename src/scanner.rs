use crate::probe::{Error, Finding, Probe};
use eros::{Context, TracedError, bail};
use indicatif::{MultiProgress, ProgressBar, ProgressFinish, ProgressIterator};
use itertools::Itertools;
use jwalk::{Parallelism, WalkDirGeneric};
use num_cpus;
use rayon::{ThreadPoolBuilder, prelude::*};
use std::{path::Path, time::Duration};

use std::ops::Deref;

/// A collection of findings reported by a single probe.
pub(crate) struct ProbeFindings {
    pub probe_name: String,
    pub findings: Vec<Finding>,
}

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

/// Internal trait to handle different error types generically
pub(crate) trait ProbeAdapter: Send + Sync {
    fn select(&mut self, entry: &DirEntry) -> bool;
    fn scan_all_suspects(&self) -> eros::Result<ProbeFindings>;
}

// Blanket implementation for ALL probes whose Error can convert to Finding
impl<P> ProbeAdapter for P
where
    P: Probe,
    Finding: TryFrom<P::Error, Error = Error>,
{
    fn select(&mut self, entry: &DirEntry) -> bool {
        Probe::select(self, entry)
    }

    fn scan_all_suspects(&self) -> eros::Result<ProbeFindings> {
        let findings = self
            .suspects()
            .par_iter()
            .map(|suspect| {
                self.scan(suspect).or_else(|e| {
                    // Try to convert the probe-specific error into a Finding
                    Finding::try_from(e)
                        // If conversion succeeds, wrap it in a Vec and treat as Ok
                        .map(|finding| vec![finding])
                        // If conversion fails, the original error is fatal
                        .map_err(TracedError::from)
                })
            })
            // Collect the results. `?` will propagate the first fatal error.
            .collect::<eros::Result<Vec<Vec<Finding>>>>()?;

        Ok(ProbeFindings {
            probe_name: self.name(),
            findings: findings.into_iter().flatten().collect(),
        })
    }
}

pub(crate) struct Scanner {
    probes: Vec<Box<dyn ProbeAdapter>>,
}

impl Scanner {
    pub fn with_probes(probes: Vec<Box<dyn ProbeAdapter>>) -> Self {
        Scanner { probes }
    }

    pub(crate) fn scan<P: AsRef<Path> + Sync + Send>(
        &mut self,
        scan_dir: P,
        scan_message: impl Into<String> + Send,
        parallelism: Option<usize>,
    ) -> eros::Result<Vec<ProbeFindings>> {
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

            let findings: Vec<ProbeFindings> = active_probe_indices
                .par_iter()
                .map(|&index| self.probes[index].scan_all_suspects())
                .collect::<eros::Result<Vec<ProbeFindings>>>()?;

            Ok(findings)
        })
    }
}
