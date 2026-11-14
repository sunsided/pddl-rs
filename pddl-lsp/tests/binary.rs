//! Runs the `pddl-lsp` binary.

use std::io::{BufRead, BufReader, Write};
use std::process::{Command, Stdio};

use assert2::check;

const BINARY: &str = env!("CARGO_BIN_EXE_pddl-lsp");

#[test]
fn prints_its_version_and_help() {
    let version = Command::new(BINARY).arg("--version").output().unwrap();
    check!(version.status.success());
    check!(String::from_utf8_lossy(&version.stdout).starts_with("pddl-lsp "));
    let help = Command::new(BINARY).arg("-h").output().unwrap();
    check!(help.status.success());
    check!(String::from_utf8_lossy(&help.stdout).contains("Usage: pddl-lsp [--stdio]"));
}

#[test]
fn rejects_unknown_arguments() {
    let output = Command::new(BINARY).arg("--bogus").output().unwrap();
    check!(!output.status.success());
    check!(String::from_utf8_lossy(&output.stderr).contains("unknown argument `--bogus`"));
}

/// Writes a message with the protocol's header.
fn send(stdin: &mut impl Write, message: &serde_json::Value) {
    let body = message.to_string();
    write!(stdin, "Content-Length: {}\r\n\r\n{body}", body.len()).unwrap();
    stdin.flush().unwrap();
}

/// Reads one message with the protocol's header.
fn receive(stdout: &mut impl BufRead) -> serde_json::Value {
    let mut length = 0;
    loop {
        let mut line = String::new();
        stdout.read_line(&mut line).unwrap();
        if let Some(value) = line.strip_prefix("Content-Length: ") {
            length = value.trim().parse().unwrap();
        }
        if line == "\r\n" {
            break;
        }
    }
    let mut body = vec![0; length];
    stdout.read_exact(&mut body).unwrap();
    serde_json::from_slice(&body).unwrap()
}

#[test]
fn serves_over_stdio_until_exit() {
    let mut child = Command::new(BINARY)
        .arg("--stdio")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    let mut stdin = child.stdin.take().unwrap();
    let mut stdout = BufReader::new(child.stdout.take().unwrap());
    let initialize = serde_json::json!({
        "jsonrpc": "2.0", "id": 1, "method": "initialize", "params": { "capabilities": {} },
    });
    send(&mut stdin, &initialize);
    let response = receive(&mut stdout);
    check!(response["result"]["serverInfo"]["name"] == "pddl-lsp");
    send(
        &mut stdin,
        &serde_json::json!({ "jsonrpc": "2.0", "method": "initialized", "params": {} }),
    );
    send(
        &mut stdin,
        &serde_json::json!({ "jsonrpc": "2.0", "id": 2, "method": "shutdown" }),
    );
    check!(receive(&mut stdout)["id"] == 2);
    send(
        &mut stdin,
        &serde_json::json!({ "jsonrpc": "2.0", "method": "exit" }),
    );
    check!(child.wait().unwrap().success());
}
