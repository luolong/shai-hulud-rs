use std::{
    path::{Path, PathBuf},
    time::Duration,
};

use indicatif::{MultiProgress, ParallelProgressIterator, ProgressBar};
use rayon::prelude::*;

use crate::{
    probe::{Finding, Payload, Probe},
    scanner::DirEntry,
};

const MALICIOUS_HASHLIST: [&'static str; 9] = [
    "de0e25a3e6c1e1e5998b306b7141b3dc4c0088da9d7bb47c1c00c91e6e4f85d6",
    "81d2a004a1bca6ef87a1caf7d0e0b355ad1764238e40ff6d1b1cb77ad4f595c3",
    "83a650ce44b2a9854802a7fb4c202877815274c129af49e6c2d1d5d5d55c501e",
    "4b2399646573bb737c4969563303d8ee2e9ddbd1b271f1ca9e35ea78062538db",
    "dc67467a39b70d1cd4c1f7f7a459b35058163592f4a9e8fb4dffcbba98ef210c",
    "46faab8ab153fae6e80e7cca38eab363075bb524edd79e42269217a083628f09",
    "b74caeaa75e077c99f7d44f46daaf9796a3be43ecf24f2a1fd381844669da777",
    "86532ed94c5804e1ca32fa67257e1bb9de628e3e48a1f56e67042dc055effb5b", // test-cases/multi-hash-detection/file1.js
    "aba1fcbd15c6ba6d9b96e34cec287660fff4a31632bf76f2a766c499f55ca1ee", // test-cases/multi-hash-detection/file2.js
];

pub(crate) struct MaliciousHash(String);

impl Payload for MaliciousHash {
    //TODO: Implement payload methods
}

impl From<String> for Box<MaliciousHash> {
    fn from(value: String) -> Self {
        Box::new(MaliciousHash(value))
    }
}

/// Scan typescript source files and compare SHA256 hashes against known malicious hash list
pub struct CheckFileHashes {
    suspects: Vec<PathBuf>,
}

impl CheckFileHashes {
    pub fn new() -> Box<Self> {
        Box::new(Self {
            suspects: Vec::new(),
        })
    }
}

impl Probe for CheckFileHashes {
    fn name(&self) -> String {
        "Check malicious file hashes".to_owned()
    }

    fn select(&mut self, entry: &DirEntry) -> bool {
        if entry.file_type().is_file() {
            let file_name = entry.file_name().to_string_lossy();
            if file_name.ends_with(".js")
                || file_name.ends_with(".ts")
                || file_name.ends_with(".json")
            {
                self.suspects.push(entry.path().to_owned());
                return true;
            }
        }
        false
    }

    fn scan_suspects(&self, multi_progress: &MultiProgress) -> eros::Result<Vec<super::Finding>> {
        let file_count = self.suspects.len();
        let progress_bar = multi_progress.add(
            ProgressBar::new_spinner()
                .with_prefix("🔍")
                .with_message(format!(
                    "Checking {file_count} files for known malicious content..."
                ))
                .with_finish(indicatif::ProgressFinish::WithMessage(std::borrow::Cow::Owned(format!(
                    "Finished checking {file_count} files for known malicious content."
                ))))
                .with_style(
                    crate::indikatif::spinners::sand()
                        .template("{prefix}{spinner}{wide_msg} {pos} / {len} checked ({percent}% / {duration})")
                        .unwrap(),
                ),
        );
        progress_bar.enable_steady_tick(Duration::from_millis(80));
        progress_bar.set_length(self.suspects.len() as u64);

        let findings = self
            .suspects
            .par_iter()
            .progress_with(progress_bar)
            .filter_map(|path| scan(&path))
            .collect::<Vec<_>>();

        Ok(findings)
    }
}

fn scan<P: AsRef<Path>>(entry: &P) -> Option<super::Finding> {
    let path = entry.as_ref();
    let Ok(file_hash) = sha256::try_digest(&path) else {
        return Some(Finding::low_risk(&path, "Failed to compute SHA-256 hash"));
    };

    if MALICIOUS_HASHLIST.contains(&file_hash.as_str()) {
        let malicious_hash = MaliciousHash(file_hash);
        let finding = Finding::high_risk(&path, "File matches known malicious SHA-256 hash")
            .with_payload(Box::new(malicious_hash));
        let finding = finding;
        Some(finding)
    } else {
        None
    }
}
