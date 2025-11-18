use std::{
    ffi::OsStr,
    path::{Path, PathBuf},
    sync::Arc,
    time::Duration,
};

use eros::{Context, bail};
use indicatif::{MultiProgress, ProgressBar, ProgressFinish, ProgressIterator};
use itertools::Itertools;
use jwalk::{Parallelism, WalkDirGeneric};
use rayon::{ThreadPoolBuilder, iter::ParallelExtend};

use crate::probe::Probe;

pub(crate) struct DirEntry(jwalk::DirEntry<((), ())>);

impl From<jwalk::DirEntry<((), ())>> for DirEntry {
    fn from(entry: jwalk::DirEntry<((), ())>) -> Self {
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
}

pub(crate) struct Scanner {
    probes: Vec<Box<dyn Probe>>,
}

impl Scanner {
    pub fn with_probes(probes: Vec<Box<dyn Probe>>) -> Self {
        Scanner { probes }
    }

    pub(crate) fn scan<P: AsRef<Path>>(
        &mut self,
        scan_dir: P,
        scan_message: impl Into<String>,
        parallelism: Option<usize>,
    ) -> eros::Result<()> {
        let mut pool = ThreadPoolBuilder::new();
        pool = match parallelism {
            Some(0) => bail!("Parallelism cannot be set to zero"),
            Some(1) => pool.use_current_thread().num_threads(1),
            Some(thread_count) => pool.num_threads(thread_count),
            None => pool,
        };

        let pool = pool.build().context("Failed to build thread pool")?;
        let pool = Arc::new(pool);

        let busy_timeout = Some(Duration::from_millis(100));
        let walker = WalkDirGeneric::<((), ())>::new(&scan_dir);
        let walker = match parallelism {
            Some(1) => walker.parallelism(Parallelism::Serial),
            Some(_) => walker.parallelism(Parallelism::RayonExistingPool { pool, busy_timeout }),
            None => walker,
        };

        let scan_message = scan_message.into();
        //let walk_style = crate::indikatif::spinners::dots13()
        let walk_style = crate::indikatif::spinners::point()
            .template("{msg:.blue} {spinner}")
            .unwrap();

        let walk_progress = ProgressBar::new_spinner();
        walk_progress.enable_steady_tick(Duration::from_millis(100));

        let probe_indices: Vec<usize> = walker
            .into_iter()
            .progress_with(walk_progress)
            .with_prefix("📂")
            .with_message(scan_message.clone())
            .with_finish(ProgressFinish::AndLeave)
            .with_style(walk_style)
            .filter_map(|e| e.ok())
            .flat_map(|e| {
                let mut probe_indices = Vec::with_capacity(self.probes.len());

                let entry = DirEntry::from(e);
                for (index, probe) in self.probes.iter_mut().enumerate() {
                    if probe.select(&entry) {
                        probe_indices.push(index);
                    }
                }

                probe_indices
            })
            .unique()
            .collect();

        let multi_progress = MultiProgress::new();
        multi_progress.set_alignment(indicatif::MultiProgressAlignment::Bottom);
        let mut findings = Vec::new();
        rayon::scope(|_s| {
            for probe_index in probe_indices {
                let probe = &self.probes[probe_index];
                let Ok(curent_findings) = probe.scan_suspects(&multi_progress) else {
                    unreachable!("Really?")
                };
                findings.par_extend(curent_findings);
            }
        });

        Ok(())
    }
}
