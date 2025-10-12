mod probe;
mod report;
mod scanner;

use std::path::PathBuf;

use clap::Parser;
use console::style;
use eros::{Context, bail};

use crate::{probe::CheckWorkflowFiles, report::generate_report, scanner::Scanner};

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    directory_to_scan: PathBuf,

    /// Read list of compromised packages from a file
    #[arg(short = 'f', long, default_value = "compromised-packages.txt")]
    compromised_packages: Option<PathBuf>,

    /// Enable additional security checks (typosquatting, network patterns)
    ///
    /// These are general security features, not specific to Shai-Hulud
    #[arg(long)]
    paranoid: bool,

    /// Set the number of threads to use for parallelized steps
    #[arg(short = 't', long, value_name = "N")]
    parallelism: Option<usize>,
}

fn main() -> eros::Result<()> {
    let cli = Cli::parse();

    if !cli.directory_to_scan.is_dir() {
        bail!(
            "Error: Directory {} does not exist.",
            cli.directory_to_scan.display()
        );
    }

    let scan_dir = cli
        .directory_to_scan
        .canonicalize()
        .context("Getting absolute path of directory to scan")?;

    let scanner = Scanner::with_probes(vec![CheckWorkflowFiles::new()]);

    println!("{}", style("Starting Shai-Hulud detection scan...").green());
    let scan_message = if cli.paranoid {
        format!(
            "Scanning directory: {} (with paranoid mode enabled)",
            scan_dir.display()
        )
    } else {
        format!("Scanning directory: {}", scan_dir.display())
    };

    let findings = scanner.scan(&scan_dir, scan_message, cli.parallelism)?;

    generate_report(&findings, cli.paranoid)?;

    Ok(())
}
