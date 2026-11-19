//! The `pddl-lsp` binary: a PDDL language server on stdin and stdout.

use std::process::ExitCode;

fn main() -> ExitCode {
    for arg in std::env::args().skip(1) {
        match arg.as_str() {
            "--version" | "-V" => {
                println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
                return ExitCode::SUCCESS;
            }
            "--help" | "-h" => {
                println!(
                    "{} {}\n{}\n\nUsage: pddl-lsp [--stdio]\n\n\
                     Speaks the Language Server Protocol on stdin and stdout.",
                    env!("CARGO_PKG_NAME"),
                    env!("CARGO_PKG_VERSION"),
                    env!("CARGO_PKG_DESCRIPTION"),
                );
                return ExitCode::SUCCESS;
            }
            // Some clients always pass it; stdio is the only transport.
            "--stdio" => {}
            other => {
                eprintln!("pddl-lsp: unknown argument `{other}`; see --help");
                return ExitCode::FAILURE;
            }
        }
    }
    match pddl_lsp::server::run_stdio() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("pddl-lsp: {error}");
            ExitCode::FAILURE
        }
    }
}
