//! Drives the server through the protocol, as an editor would.

use std::thread;

use assert2::check;
use lsp_server::{Connection, Message, Notification, Request, RequestId, Response};
use serde_json::{json, Value};

const DOMAIN: &str = "(define (domain logistics)
  (:requirements :typing)
  (:types truck place)
  (:predicates (at ?t - truck ?p - place))
  (:action drive
    :parameters (?t - truck ?from ?to - place)
    :precondition (at ?t ?from)
    :effect (and (not (at ?t ?from)) (at ?t ?to))))
";

const PROBLEM: &str = "(define (problem deliver)
  (:domain logistics)
  (:objects t1 - truck home work - place)
  (:init (at t1 home))
  (:goal (at t1 work ham)))
";

/// A client talking to a server running on another thread.
struct Client {
    connection: Connection,
    next_id: i32,
    server: Option<thread::JoinHandle<()>>,
}

impl Client {
    fn start() -> Self {
        let (server, client) = Connection::memory();
        let server = thread::spawn(move || pddl_lsp::server::run(&server).unwrap());
        let mut client = Self {
            connection: client,
            next_id: 0,
            server: Some(server),
        };
        let result = client.request("initialize", json!({ "capabilities": {} }));
        check!(result["capabilities"]["positionEncoding"] == "utf-16");
        client.notify("initialized", json!({}));
        client
    }

    fn notify(&self, method: &str, params: Value) {
        let message = Notification::new(method.into(), params);
        self.connection.sender.send(message.into()).unwrap();
    }

    /// Sends a request and returns its result, collecting notifications on the way.
    fn request(&mut self, method: &str, params: Value) -> Value {
        self.next_id += 1;
        let id = RequestId::from(self.next_id);
        let request = Request::new(id.clone(), method.into(), params);
        self.connection.sender.send(request.into()).unwrap();
        loop {
            match self.connection.receiver.recv().unwrap() {
                Message::Response(Response {
                    id: got,
                    result,
                    error,
                }) if got == id => {
                    check!(error.is_none(), "{method}: {error:?}");
                    return result.unwrap_or(Value::Null);
                }
                _ => {}
            }
        }
    }

    /// The next diagnostics published for `uri`.
    fn diagnostics(&self, uri: &str) -> Vec<Value> {
        loop {
            if let Message::Notification(n) = self.connection.receiver.recv().unwrap() {
                if n.method == "textDocument/publishDiagnostics" && n.params["uri"] == uri {
                    return n.params["diagnostics"].as_array().unwrap().clone();
                }
            }
        }
    }

    fn open(&self, uri: &str, text: &str) {
        let document = json!({ "uri": uri, "languageId": "pddl", "version": 1, "text": text });
        self.notify("textDocument/didOpen", json!({ "textDocument": document }));
    }
}

impl Drop for Client {
    fn drop(&mut self) {
        if !thread::panicking() {
            self.request("shutdown", Value::Null);
            self.notify("exit", Value::Null);
            self.server.take().unwrap().join().unwrap();
        }
    }
}

fn at(uri: &str, line: u32, character: u32) -> Value {
    json!({ "textDocument": { "uri": uri }, "position": { "line": line, "character": character } })
}

#[test]
fn an_editor_session() {
    let (d, p) = ("file:///w/domain.pddl", "file:///w/problem.pddl");
    let mut client = Client::start();
    client.open(d, DOMAIN);
    check!(client.diagnostics(d).is_empty());
    client.open(p, PROBLEM);
    let diagnostics = client.diagnostics(p);
    check!(diagnostics.len() == 2);
    check!(diagnostics[0]["message"] == "predicate `at` takes 2 arguments, but 3 are given");
    check!(diagnostics[1]["message"] == "unknown object or constant `ham`");
    check!(diagnostics[1]["range"]["start"] == json!({ "line": 4, "character": 21 }));

    // `at` in the problem's `:init` leads to the domain.
    let definition = client.request("textDocument/definition", at(p, 3, 10));
    check!(definition["uri"] == d);
    check!(definition["range"]["start"] == json!({ "line": 3, "character": 16 }));

    let hover = client.request("textDocument/hover", at(p, 3, 10));
    let markdown = hover["contents"]["value"].as_str().unwrap();
    check!(markdown.contains("(at ?t - truck ?p - place)"));
    check!(markdown.contains("predicate in domain `logistics`"));

    let mut params = at(d, 3, 16);
    params["context"] = json!({ "includeDeclaration": true });
    let references = client.request("textDocument/references", params);
    check!(references.as_array().unwrap().len() == 6);

    let mut params = at(d, 2, 10);
    params["newName"] = json!("lorry");
    let edit = client.request("textDocument/rename", params);
    check!(edit["changes"][d].as_array().unwrap().len() == 3);
    check!(edit["changes"][p].as_array().unwrap().len() == 1);

    let symbols = client.request(
        "textDocument/documentSymbol",
        json!({ "textDocument": { "uri": d } }),
    );
    let children: Vec<_> = symbols[0]["children"]
        .as_array()
        .unwrap()
        .iter()
        .map(|s| s["name"].as_str().unwrap())
        .collect();
    check!(children == ["truck", "place", "at", "drive"]);

    // Completing a variable inside the action offers its parameters.
    let completion = client.request("textDocument/completion", at(d, 6, 23));
    let labels: Vec<_> = completion
        .as_array()
        .unwrap()
        .iter()
        .map(|i| i["label"].as_str().unwrap())
        .collect();
    check!(labels == ["?t", "?from", "?to"]);

    let help = client.request("textDocument/signatureHelp", at(p, 3, 16));
    check!(help["signatures"][0]["label"] == "(at ?t - truck ?p - place)");
    check!(help["signatures"][0]["activeParameter"] == 1);

    let tokens = client.request(
        "textDocument/semanticTokens/full",
        json!({ "textDocument": { "uri": p } }),
    );
    check!(tokens["data"].as_array().unwrap().len() % 5 == 0);

    // Formatting a messy file and fixing a misspelled section.
    let messy = "(define (domain m) (:predicats (p))\n(:action a :parameters () :effect (p)))";
    client.open("file:///w/messy.pddl", messy);
    let diagnostics = client.diagnostics("file:///w/messy.pddl");
    check!(diagnostics[0]["message"]
        .as_str()
        .unwrap()
        .ends_with("did you mean `:predicates`?"));
    let mut params = json!({ "textDocument": { "uri": "file:///w/messy.pddl" }, "context": { "diagnostics": [] } });
    params["range"] = diagnostics[0]["range"].clone();
    let actions = client.request("textDocument/codeAction", params);
    check!(actions[0]["title"] == "Replace with `:predicates`");
    let options = json!({ "tabSize": 2, "insertSpaces": true });
    let params = json!({ "textDocument": { "uri": "file:///w/messy.pddl" }, "options": options });
    let edits = client.request("textDocument/formatting", params);
    check!(
        edits[0]["newText"]
            == "(define (domain m)\n  (:predicats (p))\n  (:action a :parameters () :effect (p)))\n"
    );
}

#[test]
fn inlay_hints_name_parameters() {
    let (d, p) = ("file:///w/domain.pddl", "file:///w/problem.pddl");
    let mut client = Client::start();
    client.open(d, DOMAIN);
    client.open(p, PROBLEM);
    // Parameter names before the arguments of `(at t1 home)` in `:init`.
    let range =
        json!({ "start": { "line": 3, "character": 0 }, "end": { "line": 3, "character": 30 } });
    let hints = client.request(
        "textDocument/inlayHint",
        json!({ "textDocument": { "uri": p }, "range": range }),
    );
    let hints: Vec<_> = hints
        .as_array()
        .unwrap()
        .iter()
        .map(|h| {
            (
                h["label"].as_str().unwrap(),
                h["position"]["character"].as_u64().unwrap(),
            )
        })
        .collect();
    check!(hints == [("?t:", 13), ("?p:", 16)]);
    // In the domain, `(at ?t ?from)` only needs a hint for `?from`.
    let range =
        json!({ "start": { "line": 6, "character": 0 }, "end": { "line": 6, "character": 40 } });
    let hints = client.request(
        "textDocument/inlayHint",
        json!({ "textDocument": { "uri": d }, "range": range }),
    );
    let labels: Vec<_> = hints
        .as_array()
        .unwrap()
        .iter()
        .map(|h| h["label"].as_str().unwrap())
        .collect();
    check!(labels == ["?p:"]);
}
