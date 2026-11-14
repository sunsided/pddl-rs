//! A language server for PDDL.
//!
//! The binary speaks the Language Server Protocol over stdin and stdout; see
//! the README for setting it up in an editor. The modules are public for testing.

pub mod analysis;
pub mod format;
pub mod line_index;
pub mod server;
pub mod workspace;
