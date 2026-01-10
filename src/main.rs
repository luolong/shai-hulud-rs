mod indikatif;
mod probe;
mod report;
mod scanner;

use std::path::PathBuf;

use clap::Parser;
use console::style;
use eros::{Context, bail};

use crate::probe::{check_file_hashes::CheckFileHashes, check_workflow_files::CheckWorkflowFiles};
use crate::report::{ReportBuilder, Reporter, console::ConsoleReporter};
use crate::scanner::Scanner;

/// Shai-Hulud: A command-line tool to detect NPM supply-chain attacks.
#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Directory to scan
    #[arg(default_value = ".")]
    directory_to_scan: PathBuf,

    /// Read list of compromised packages from a file
    #[arg(short = 'f', long, default_value = "compromised-packages.txt")]
    compromised_packages: PathBuf,

    /// Enable additional security checks (typosquatting, network patterns)
    #[arg(long)]
    paranoid: bool,

    /// Set the number of threads to use for parallelized steps
    #[arg(short = 't', long, value_name = "N")]
    parallelism: Option<usize>,

    /// Optional output file. Supports JSON, CSV, and HTML formats
    /// based on the file extension. Defaults to console output.
    #[arg(short = 'o', long, value_name = "FILE")]
    output_file: Option<PathBuf>,
}

fn main() -> eros::Result<()> {
    let cli = Cli::parse();

    if !cli.directory_to_scan.is_dir() {
        bail!(
            "Error: Directory '{}' does not exist.",
            cli.directory_to_scan.display()
        );
    }

    let scan_dir = cli
        .directory_to_scan
        .canonicalize()
        .context("Getting absolute path of directory to scan")?;

    let mut scanner = Scanner::with_probes(vec![
        Box::new(CheckWorkflowFiles::new()),
        Box::new(CheckFileHashes::new()),
    ]);

    println!("{}", style("Starting Shai-Hulud detection scan...").green());
    let scan_message = if cli.paranoid {
        format!(
            "Scanning directory: {} (with paranoid mode enabled)",
            scan_dir.display()
        )
    } else {
        format!("Scanning directory: {}", scan_dir.display())
    };

    let probe_findings = scanner.scan(&scan_dir, scan_message, cli.parallelism)?;

    // Group findings by probe for reporting
    let mut report_builder = ReportBuilder::new();
    report_builder.add_probe_findings(probe_findings);
    let report = report_builder.build();

    let reporter: Box<dyn Reporter> = match cli.output_file {
        None => Box::new(ConsoleReporter::new()),
        Some(path) => match path.extension().and_then(|s| s.to_str()) {
            Some("json") => bail!("JSON reporter not yet implemented"),
            Some("csv") => bail!("CSV reporter not yet implemented"),
            Some("html") => bail!("HTML reporter not yet implemented"),
            _ => bail!("Unsupported output file format. Use .json, .csv, or .html"),
        },
    };

    reporter.report(&report)
}
