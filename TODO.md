# Shai-Hulud-rs Development Plan

This file tracks the progress of the architectural refactoring.

- [x] **Phase 1: Project Setup & Dependencies** - Update `Cargo.toml` to include the `num_cpus` crate, which is required for the default parallelism feature.
- [x] **Phase 2: Refactor the Scanner** - Rewrite the `Scanner::scan` function in `src/scanner.rs` to correctly implement the two-pass, fan-out parallel architecture.
- [x] **Phase 3: Update the CLI** - Modify the `Cli` struct in `src/main.rs` to serve as a drop-in replacement for the bash script, adding the `--output` argument for enhanced reporting.
- [x] **Phase 4: Implement Reporting Logic** - Refactor `main.rs` to orchestrate the entire process: call the scanner, build the report from the grouped findings, and use a `Reporter` to generate the final output based on the CLI arguments.
- [x] **Phase 5: Verify Probe Implementation** - Ensure both probes (`check_file_hashes.rs` and `check_workflow_files.rs`) are correctly stateful, using their internal `suspects` list to pass state from the `select` pass to the `scan_suspects` pass.
