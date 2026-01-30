use crate::{
    probe::{Error, Finding, Metadata, Payload, Probe, ProgressMeta},
    scanner::DirEntry,
};
use std::io;
use std::path::PathBuf;

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

use std::fmt::{Debug, Formatter};

use super::ProgressMetaFinish;

pub struct MaliciousHash(String);

impl Debug for MaliciousHash {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("MaliciousHash").field(&self.0).finish()
    }
}

impl Payload for MaliciousHash {}

/// Errors that can occur during file hash checking
#[derive(Debug, thiserror::Error)]
pub enum HashCheckError {
    #[error("Failed to read file: {0}")]
    Io(#[from] io::Error),

    #[error("Failed to compute hash: {0}")]
    HashComputation(String),
}

/// Convert transient errors to low-risk findings
impl TryFrom<HashCheckError> for Finding {
    type Error = Error;

    fn try_from(error: HashCheckError) -> Result<Finding, Error> {
        match error {
            HashCheckError::Io(io_err) => Ok(Finding::low_risk(
                &PathBuf::from("<file>"),
                &format!("Could not read file: {}", io_err),
            )),
            HashCheckError::HashComputation(msg) => Ok(Finding::low_risk(
                &PathBuf::from("<file>"),
                &format!("Could not compute hash: {}", msg),
            )),
        }
    }
}

/// Scan files and compare SHA256 hashes against a known malicious hash list.
pub struct CheckFileHashes {
    suspects: Vec<PathBuf>,
}

impl CheckFileHashes {
    pub fn new() -> Self {
        Self {
            suspects: Vec::new(),
        }
    }
}

impl Probe for CheckFileHashes {
    type Suspect = PathBuf;
    type Error = HashCheckError;

    fn name(&self) -> String {
        "Check malicious file hashes".to_owned()
    }

    fn metadata(&self) -> Metadata {
        Metadata {
            name: "Check malicious file hashes".to_owned(),
            description:
                "Scan files and compare SHA256 hashes against a known malicious hash list."
                    .to_owned(),

            progress: ProgressMeta {
                prefix: "🧮".to_owned(),
                message: "Checking file hashes...".to_owned(),
                finish: ProgressMetaFinish::WithMessage("Finished checking file hashes..."),
            },
        }
    }

    fn select(&mut self, entry: &DirEntry) -> bool {
        if entry.file_type().is_file() {
            let file_name = entry.file_name().to_string_lossy();
            if file_name.ends_with(".js")
                || file_name.ends_with(".ts")
                || file_name.ends_with(".json")
            {
                self.suspects.push(entry.path());
                return true;
            }
        }
        false
    }

    fn scan(&self, suspect: &Self::Suspect) -> Result<Vec<Finding>, Self::Error> {
        let path = suspect;
        let file_hash =
            sha256::try_digest(path).map_err(|e| HashCheckError::HashComputation(e.to_string()))?;

        let mut findings = Vec::new();
        if MALICIOUS_HASHLIST.contains(&file_hash.as_str()) {
            let payload = MaliciousHash(file_hash);
            findings.push(
                Finding::high_risk(path, "File matches known malicious SHA-256 hash")
                    .with_payload(Box::new(payload)),
            );
        }
        Ok(findings)
    }

    fn suspects(&self) -> &[Self::Suspect] {
        &self.suspects
    }
}
