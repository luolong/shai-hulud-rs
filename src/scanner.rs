use crate::indikatif::spinners::dots_with_final;
use crate::print_status;
use crate::probe::{Error, Finding, Metadata, Probe, ProgressMeta, ProgressMetaFinish};
use eros::{Context, TracedError, bail};
use indicatif::{
    MultiProgress, ParallelProgressIterator, ProgressBar, ProgressFinish, ProgressIterator,
};
use jwalk::{Parallelism, WalkDirGeneric};
use num_cpus;
use rayon::{ThreadPoolBuilder, prelude::*};
use std::borrow::Cow;
use std::sync::Arc;
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
    fn scan_all_suspects(&self, multi_progress: &Arc<MultiProgress>)
    -> eros::Result<ProbeFindings>;
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

    fn scan_all_suspects(
        &self,
        multi_progress: &Arc<MultiProgress>,
    ) -> eros::Result<ProbeFindings> {
        let suspects = self.suspects();
        let progress = multi_progress.add(create_progress_bar_from(self.metadata()));
        progress.set_length(suspects.len() as u64);

        let findings = suspects
            .par_iter()
            .progress_with(progress)
            .map(|suspect| {
                self.scan(suspect).or_else(|e| {
                    // Try to convert the probe-specific error into a Finding
                    Finding::try_from(e)
                        .map(|finding| vec![finding])
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

fn create_progress_bar_from(metadata: Metadata) -> ProgressBar {
    let ProgressMeta {
        prefix,
        message,
        finish,
    } = metadata.progress;
    let progress = ProgressBar::new_spinner()
        .with_prefix(prefix)
        .with_message(message)
        .with_style(
            dots_with_final("✅")
                .template("  {prefix} {spinner} {wide_msg} {pos}/{len} ({percent}%)")
                .unwrap(),
        );

    match finish {
        ProgressMetaFinish::Clear => progress.with_finish(ProgressFinish::AndClear),
        ProgressMetaFinish::Abandon => progress.with_finish(ProgressFinish::Abandon),
        ProgressMetaFinish::WithMessage(message) => {
            progress.with_finish(ProgressFinish::AbandonWithMessage(Cow::Borrowed(message)))
        }
    }
}

pub(crate) struct Scanner {
    stages: Vec<ScannerStage>,
}

pub(crate) struct ScannerStage {
    name: String,
    probes: Vec<Box<dyn ProbeAdapter>>,
}

impl Scanner {
    pub fn new() -> Self {
        Scanner { stages: Vec::new() }
    }

    pub fn with_stage<S: AsRef<str>>(
        &mut self,
        name: S,
        probes: Vec<Box<dyn ProbeAdapter>>,
    ) -> &mut Self {
        self.stages.push(ScannerStage {
            name: name.as_ref().to_string(),
            probes,
        });
        self
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

        let stage_count = self.stages.len() + 1;
        pool.install(|| {
            // --- Pass 1: Selection ---
            print_status!(
                ORANGE,
                format!("[Stage 1/{stage_count}] Collecting file inventory for analysis")
            );

            let scan_message = scan_message.into();
            let walk_style = crate::indikatif::spinners::point()
                .template("  {msg:.blue} {spinner}")
                .unwrap();
            let walk_progress = ProgressBar::new_spinner()
                .with_prefix("📂")
                .with_message(scan_message.clone())
                .with_finish(ProgressFinish::AndLeave)
                .with_style(walk_style);

            walk_progress.enable_steady_tick(Duration::from_millis(100));

            let walker =
                WalkDirGeneric::<((), ())>::new(&scan_dir).parallelism(if num_threads == 1 {
                    Parallelism::Serial
                } else {
                    Parallelism::RayonNewPool(num_threads)
                });

            walker
                .into_iter()
                .progress_with(walk_progress)
                .filter_map(|e| e.ok())
                .for_each(|e| {
                    let entry = DirEntry::from(e);
                    self.stages
                        .iter_mut()
                        .flat_map(|stage| stage.probes.iter_mut())
                        .for_each(|probe| {
                            probe.select(&entry);
                        });
                });

            let mut findings = Vec::new();

            // --- Pass 2: Analysis ---
            for (index, stage) in self.stages.iter().enumerate() {
                let stage_num = index + 2;
                let stage_name = stage.name.clone();
                print_status!(
                    ORANGE,
                    format!("[Stage {stage_num}/{stage_count}] {stage_name}")
                );

                let multi_progress = Arc::new(MultiProgress::new());
                multi_progress.set_alignment(indicatif::MultiProgressAlignment::Bottom);
                multi_progress.set_draw_target(indicatif::ProgressDrawTarget::stdout_with_hz(80));

                let mut stage_findings = stage
                    .probes
                    .par_iter()
                    .map(|probe| probe.scan_all_suspects(&multi_progress))
                    .collect::<eros::Result<Vec<ProbeFindings>>>()?;

                findings.append(&mut stage_findings);
            }

            Ok(findings)
        })
    }
}
