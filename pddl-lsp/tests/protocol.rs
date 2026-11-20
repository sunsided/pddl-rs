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
        let (client, result) = Self::start_with(json!({ "capabilities": {} }));
        check!(result["capabilities"]["positionEncoding"] == "utf-16");
        client
    }

    /// Starts a server with these initialize parameters, and returns its initialize result.
    fn start_with(params: Value) -> (Self, Value) {
        let (server, client) = Connection::memory();
        let server = thread::spawn(move || pddl_lsp::server::run(&server).unwrap());
        let mut client = Self {
            connection: client,
            next_id: 0,
            server: Some(server),
        };
        let result = client.request("initialize", params);
        client.notify("initialized", json!({}));
        (client, result)
    }

    fn notify(&self, method: &str, params: Value) {
        let message = Notification::new(method.into(), params);
        self.connection.sender.send(message.into()).unwrap();
    }

    /// Sends a request and returns its response, skipping notifications on the way.
    fn response(&mut self, method: &str, params: Value) -> Response {
        self.next_id += 1;
        let id = RequestId::from(self.next_id);
        let request = Request::new(id.clone(), method.into(), params);
        self.connection.sender.send(request.into()).unwrap();
        loop {
            if let Message::Response(response) = self.connection.receiver.recv().unwrap() {
                if response.id == id {
                    return response;
                }
            }
        }
    }

    /// Sends a request and returns its result, which must not be an error.
    fn request(&mut self, method: &str, params: Value) -> Value {
        let Response { result, error, .. } = self.response(method, params);
        check!(error.is_none(), "{method}: {error:?}");
        result.unwrap_or(Value::Null)
    }

    /// The next message from the server, failing rather than hanging if it
    /// sends none.
    fn recv(&self) -> Message {
        let wait = std::time::Duration::from_secs(10);
        self.connection.receiver.recv_timeout(wait).unwrap()
    }

    /// The next diagnostics published for `uri`.
    fn diagnostics(&self, uri: &str) -> Vec<Value> {
        loop {
            if let Message::Notification(n) = self.recv() {
                if n.method == "textDocument/publishDiagnostics" && n.params["uri"] == uri {
                    return n.params["diagnostics"].as_array().unwrap().clone();
                }
            }
        }
    }

    /// The URIs of the diagnostics published until now, in order, found by
    /// waiting for the answer to a request sent after them.
    fn published(&mut self) -> Vec<String> {
        self.next_id += 1;
        let id = RequestId::from(self.next_id);
        let params = json!({ "query": "" });
        let request = Request::new(id.clone(), "workspace/symbol".into(), params);
        self.connection.sender.send(request.into()).unwrap();
        let mut uris = Vec::new();
        loop {
            match self.recv() {
                Message::Response(response) if response.id == id => return uris,
                Message::Notification(n) if n.method == "textDocument/publishDiagnostics" => {
                    uris.push(n.params["uri"].as_str().unwrap().to_string());
                }
                _ => {}
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

    // Two uses of one missing requirement give one fix that adds it.
    let untyped = "(define (domain u) (:types a - object b - object))";
    client.open("file:///w/untyped.pddl", untyped);
    let warnings = client.diagnostics("file:///w/untyped.pddl");
    let warnings: Vec<_> = warnings
        .iter()
        .filter(|d| d["severity"] == 2)
        .map(|d| d["message"].as_str().unwrap())
        .collect();
    check!(warnings == ["`-` needs `:typing` in `:requirements`"; 2]);
    let whole =
        json!({ "start": { "line": 0, "character": 0 }, "end": { "line": 0, "character": 50 } });
    let params = json!({ "textDocument": { "uri": "file:///w/untyped.pddl" }, "range": whole, "context": { "diagnostics": [] } });
    let actions = client.request("textDocument/codeAction", params);
    check!(actions.as_array().unwrap().len() == 1);
    check!(actions[0]["title"] == "Add `:typing` to `:requirements`");
    let edit = &actions[0]["edit"]["changes"]["file:///w/untyped.pddl"][0];
    check!(edit["newText"] == "\n  (:requirements :typing)");
    check!(edit["range"]["start"]["character"] == 18);

    // Two domains that both miss it each get their own fix.
    let two = "(define (domain u) (:types a - object))\n(define (domain v) (:types b - object))";
    client.open("file:///w/two.pddl", two);
    client.diagnostics("file:///w/two.pddl");
    let both =
        json!({ "start": { "line": 0, "character": 0 }, "end": { "line": 1, "character": 40 } });
    let params = json!({ "textDocument": { "uri": "file:///w/two.pddl" }, "range": both, "context": { "diagnostics": [] } });
    let actions = client.request("textDocument/codeAction", params);
    check!(actions.as_array().unwrap().len() == 2);

    let options = json!({ "tabSize": 2, "insertSpaces": true });
    let params = json!({ "textDocument": { "uri": "file:///w/messy.pddl" }, "options": options });
    let edits = client.request("textDocument/formatting", params);
    check!(
        edits[0]["newText"]
            == "(define (domain m)\n  (:predicats (p))\n  (:action a :parameters () :effect (p)))\n"
    );
}

/// A predicate no action changes is static, in hover and highlighting.
#[test]
fn marks_static_predicates() {
    let mut client = Client::start();
    let roads = "(define (domain r) (:predicates (road ?a) (at ?a))\n(:action go :parameters (?a) :precondition (road ?a) :effect (at ?a)))";
    client.open("file:///w/roads.pddl", roads);
    client.diagnostics("file:///w/roads.pddl");
    let hover = client.request("textDocument/hover", at("file:///w/roads.pddl", 0, 33));
    check!(hover["contents"]["value"]
        .as_str()
        .unwrap()
        .contains("Static: no action changes it"));
    let hover = client.request("textDocument/hover", at("file:///w/roads.pddl", 0, 45));
    check!(!hover["contents"]["value"]
        .as_str()
        .unwrap()
        .contains("Static"));
    let tokens = client.request(
        "textDocument/semanticTokens/full",
        json!({ "textDocument": { "uri": "file:///w/roads.pddl" } }),
    );
    // The fifth number of each token is its modifiers: `road` is a static
    // declaration (1 | 2), `at` a plain one (1).
    let data = tokens["data"].as_array().unwrap();
    check!(data[4 + 5] == 3);
    check!(data[4 + 3 * 5] == 1);
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

/// The labels the server completes at `|` in `text`, opened as `uri`.
fn complete(client: &mut Client, uri: &str, text: &str) -> Vec<String> {
    let offset = text.find('|').unwrap();
    let text = text.replace('|', "");
    client.open(uri, &text);
    let before = &text[..offset];
    let line = before.matches('\n').count();
    let character = offset - before.rfind('\n').map_or(0, |i| i + 1);
    let items = client.request(
        "textDocument/completion",
        at(
            uri,
            u32::try_from(line).unwrap(),
            u32::try_from(character).unwrap(),
        ),
    );
    let items = items.as_array().unwrap();
    items
        .iter()
        .map(|i| i["label"].as_str().unwrap().to_string())
        .collect()
}

#[test]
fn completes_by_context() {
    let mut client = Client::start();
    client.open("file:///w/domain.pddl", DOMAIN);
    let cases = [
        ("(define (domain c1)\n  (:|", ":predicates"),
        ("(define (problem c2)\n  (:|", ":objects"),
        ("(define (domain c3)\n  (:action a\n    :|", ":precondition"),
        (
            "(define (domain c4)\n  (:durative-action a\n    :|",
            ":duration",
        ),
        ("(define (domain c5)\n  (:requirements :|", ":typing"),
        (
            "(define (domain c6) (:predicates (on ?x))\n  (:action a :parameters (?x) :effect (|",
            "on",
        ),
        (
            "(define (domain c7) (:predicates (on ?x))\n  (:action a :parameters (?x) :effect (|",
            "and",
        ),
        (
            "(define (domain c8) (:types block)\n  (:constants b1 - |",
            "block",
        ),
        (
            "(define (domain c9) (:types block)\n  (:constants b1 - |",
            "object",
        ),
        (
            "(define (problem c10) (:domain logistics) (:objects t9 - truck)\n  (:init (at |",
            "t9",
        ),
        ("(define (problem c11) (:domain |", "logistics"),
        (
            "(define (domain c12) (:types block)\n  (:constants b1 - (either |",
            "block",
        ),
        (
            "(define (problem c13) (:domain logistics)\n  (:metric |",
            "minimize",
        ),
        (
            "(define (problem c14) (:domain logistics)\n  (:metric minimize (|",
            "total-time",
        ),
        (
            "(define (problem c15) (:domain logistics)\n  (:init (|",
            "not",
        ),
        (
            "(define (problem c15) (:domain logistics)\n  (:init (|",
            "=",
        ),
        (
            "(define (problem c19) (:domain logistics)\n  (:init (at 10 (|",
            "not",
        ),
        (
            "(define (problem c20) (:domain logistics)\n  (:metric minimize |",
            "total-time",
        ),
        // `:init` is still open, but the metric decides.
        (
            "(define (problem c21) (:domain logistics)\n  (:init (at t1 home)\n  (:metric minimize (|",
            "total-time",
        ),
    ];
    for (index, (text, expected)) in cases.into_iter().enumerate() {
        let uri = format!("file:///w/complete{index}.pddl");
        let labels = complete(&mut client, &uri, text);
        check!(labels.iter().any(|l| l == expected), "{text:?}: {labels:?}");
    }
}

#[test]
fn leaves_out_completions_that_cannot_appear() {
    let mut client = Client::start();
    client.open("file:///w/domain.pddl", DOMAIN);
    let fuelled = "(define (domain fuelled) (:predicates (parked ?x)) (:functions (fuel ?x)))";
    client.open("file:///w/fuelled.pddl", fuelled);
    let absent = [
        // A function only starts a fact within `(= ...)`.
        (
            "(define (problem c27) (:domain fuelled)\n  (:init (|",
            "fuel",
        ),
        (
            "(define (problem c28) (:domain fuelled)\n  (:init (= (|",
            "parked",
        ),
        (
            "(define (problem c29) (:domain logistics)\n  (:init (at 10 (|",
            "and",
        ),
        (
            "(define (problem c16) (:domain logistics)\n  (:init (|",
            "forall",
        ),
        (
            "(define (problem c17) (:domain logistics)\n  (:metric minimize (|",
            "and",
        ),
        ("(define (problem c18) (:objects t9)\n  (:domain |", "t9"),
        (
            "(define (problem c22) (:domain logistics)\n  (:init (= (|",
            "at",
        ),
        (
            "(define (problem c23) (:domain logistics)\n  (:init (not (|",
            "not",
        ),
        (
            "(define (problem c24) (:domain logistics)\n  (:metric minimize |",
            "minimize",
        ),
    ];
    for (index, (text, unexpected)) in absent.into_iter().enumerate() {
        let uri = format!("file:///w/absent{index}.pddl");
        let labels = complete(&mut client, &uri, text);
        check!(
            !labels.iter().any(|l| l == unexpected),
            "{text:?}: {labels:?}"
        );
    }

    let text = "(define (problem c30) (:domain fuelled)\n  (:init (= (|";
    let labels = complete(&mut client, "file:///w/value.pddl", text);
    check!(labels == ["fuel"], "{labels:?}");

    // A function without arguments needs no `(`, but an object can't go there.
    let text = "(define (problem c31) (:domain fuelled) (:objects t9)\n  (:init (= |";
    let labels = complete(&mut client, "file:///w/bare.pddl", text);
    check!(labels == ["fuel"], "{labels:?}");

    // Only `)` can follow the domain's name.
    let text = "(define (problem c25) (:objects t9) (:domain logistics |";
    let labels = complete(&mut client, "file:///w/named.pddl", text);
    check!(labels.is_empty(), "{labels:?}");

    // The predicate `at` stands for the keyword `at`, so it's offered once.
    let text = "(define (problem c26) (:domain logistics)\n  (:init (|";
    let labels = complete(&mut client, "file:///w/once.pddl", text);
    check!(
        labels.iter().filter(|l| *l == "at").count() == 1,
        "{labels:?}"
    );
}

#[test]
fn navigates_variables_and_symbols() {
    let (d, p) = ("file:///w/domain.pddl", "file:///w/problem.pddl");
    let mut client = Client::start();
    client.open(d, DOMAIN);
    client.open(p, PROBLEM);

    // `?t` in the action's precondition.
    let t = at(d, 6, 22);
    let hover = client.request("textDocument/hover", t.clone());
    check!(hover["contents"]["value"]
        .as_str()
        .unwrap()
        .contains("?t - truck"));
    let highlights = client.request("textDocument/documentHighlight", t.clone());
    check!(highlights.as_array().unwrap().len() == 4);
    let range = client.request("textDocument/prepareRename", t.clone());
    check!(range["start"] == json!({ "line": 6, "character": 22 }));
    check!(range["end"] == json!({ "line": 6, "character": 24 }));
    let mut params = t;
    params["newName"] = json!("?truck");
    let edit = client.request("textDocument/rename", params);
    check!(edit["changes"][d].as_array().unwrap().len() == 4);

    // A predicate can't be renamed to something that isn't a name.
    let mut params = at(d, 3, 16);
    params["newName"] = json!("not a name");
    let response = client.response("textDocument/rename", params);
    check!(response.error.unwrap().message == "`not a name` is not a valid predicate name");

    let symbols = client.request("workspace/symbol", json!({ "query": "DELIV" }));
    let names: Vec<_> = symbols
        .as_array()
        .unwrap()
        .iter()
        .map(|s| s["name"].as_str().unwrap())
        .collect();
    check!(names == ["deliver"]);

    let folds = client.request(
        "textDocument/foldingRange",
        json!({ "textDocument": { "uri": d } }),
    );
    let folds = folds.as_array().unwrap();
    check!(folds
        .iter()
        .any(|f| f["startLine"] == 0 && f["endLine"] == 7));

    let tokens = client.request(
        "textDocument/semanticTokens/full",
        json!({ "textDocument": { "uri": d } }),
    );
    check!(!tokens["data"].as_array().unwrap().is_empty());
}

#[test]
fn answers_unknown_requests_and_bad_parameters_with_errors() {
    let mut client = Client::start();
    let unknown = client.response("pddl/unknown", json!({}));
    check!(unknown.error.unwrap().code == -32601);
    let bad = client.response("textDocument/hover", json!({ "nonsense": true }));
    check!(bad.error.unwrap().code == -32602);
    let bad = client.response("textDocument/rename", json!({ "nonsense": true }));
    check!(bad.error.unwrap().code == -32602);
}

/// A folder under the system's temporary directory, removed when dropped.
struct TempDir(std::path::PathBuf);

impl TempDir {
    fn new(name: &str) -> Self {
        let path = std::env::temp_dir().join(format!("pddl-lsp-{name}-{}", std::process::id()));
        std::fs::create_dir_all(&path).unwrap();
        Self(path)
    }

    fn uri(&self, file: &str) -> String {
        lsp_types::Url::from_file_path(self.0.join(file))
            .unwrap()
            .to_string()
    }
}

impl Drop for TempDir {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

#[test]
fn follows_workspace_files_on_disk() {
    let dir = TempDir::new("workspace");
    std::fs::write(dir.0.join("domain.pddl"), DOMAIN).unwrap();
    std::fs::create_dir(dir.0.join(".hidden")).unwrap();
    std::fs::write(
        dir.0.join(".hidden/ignored.pddl"),
        "(define (domain hidden))",
    )
    .unwrap();
    let folder = json!({ "uri": dir.uri(""), "name": "workspace" });
    let params = json!({
        "capabilities": { "general": { "positionEncodings": ["utf-8", "utf-16"] } },
        "workspaceFolders": [folder],
    });
    let (mut client, result) = Client::start_with(params);
    check!(result["capabilities"]["positionEncoding"] == "utf-8");
    let symbols = |client: &mut Client, query: &str| {
        let found = client.request("workspace/symbol", json!({ "query": query }));
        found.as_array().unwrap().len()
    };
    // The domain was read from disk; the hidden folder was skipped.
    check!(symbols(&mut client, "logistics") == 1);
    check!(symbols(&mut client, "hidden") == 0);

    // A problem using it is checked against it.
    let p = dir.uri("problem.pddl");
    client.open(&p, PROBLEM);
    check!(client.diagnostics(&p).len() == 2);
    let change = json!({
        "textDocument": { "uri": p, "version": 2 },
        "contentChanges": [{ "text": PROBLEM.replace(" ham", "") }],
    });
    client.notify("textDocument/didChange", change);
    check!(client.diagnostics(&p).is_empty());

    // Closing the edited domain goes back to what's on disk.
    let d = dir.uri("domain.pddl");
    client.open(&d, "(define (domain logistics) (:requirements :typing))");
    // Without its types and predicates: `truck`, `place` and both `at`s are unknown.
    check!(client.diagnostics(&p).len() == 4);
    client.notify(
        "textDocument/didClose",
        json!({ "textDocument": { "uri": d } }),
    );
    check!(client.diagnostics(&p).is_empty());

    // Files created and deleted on disk come and go.
    let extra = dir.0.join("extra.pddl");
    std::fs::write(&extra, "(define (domain extra))").unwrap();
    let created = json!({ "changes": [{ "uri": dir.uri("extra.pddl"), "type": 1 }] });
    client.notify("workspace/didChangeWatchedFiles", created);
    check!(symbols(&mut client, "extra") == 1);
    std::fs::remove_file(&extra).unwrap();
    let deleted = json!({ "changes": [{ "uri": dir.uri("extra.pddl"), "type": 3 }] });
    client.notify("workspace/didChangeWatchedFiles", deleted);
    check!(symbols(&mut client, "extra") == 0);
}

/// A closed or deleted document's diagnostics are cleared once, and closed
/// workspace files never get any.
#[test]
fn publishes_diagnostics_for_open_documents_only() {
    let dir = TempDir::new("publish");
    std::fs::write(dir.0.join("domain.pddl"), DOMAIN).unwrap();
    std::fs::write(dir.0.join("other.pddl"), "(define (domain other))").unwrap();
    let folder = json!({ "uri": dir.uri(""), "name": "workspace" });
    let mut client =
        Client::start_with(json!({ "capabilities": {}, "workspaceFolders": [folder] })).0;

    let p = dir.uri("problem.pddl");
    client.open(&p, PROBLEM);
    check!(client.published() == std::slice::from_ref(&p));

    // Editing the domain checks the open problem again.
    let d = dir.uri("domain.pddl");
    client.open(&d, DOMAIN);
    check!(client.published() == [d.clone(), p.clone()]);
    client.notify(
        "textDocument/didClose",
        json!({ "textDocument": { "uri": d } }),
    );
    check!(client.diagnostics(&d).is_empty());
    check!(client.published() == std::slice::from_ref(&p));

    // An untitled document goes away on close, and its diagnostics with it.
    let u = "untitled:Untitled-1";
    client.open(u, "(define (problem q) (:domain logistics) (:init (nope)))");
    check!(!client.diagnostics(u).is_empty());
    client.notify(
        "textDocument/didClose",
        json!({ "textDocument": { "uri": u } }),
    );
    check!(client.diagnostics(u).is_empty());
}

/// Rename refuses reserved words and names already defined.
#[test]
fn rename_rejects_reserved_and_taken_names() {
    let d = "file:///w/domain.pddl";
    let mut client = Client::start();
    client.open(d, "(define (domain d) (:predicates (p ?x) (q ?x)))");
    let mut rename = |name: &str| {
        let mut params = at(d, 0, 33);
        params["newName"] = json!(name);
        client
            .response("textDocument/rename", params)
            .error
            .map(|e| e.message)
    };
    check!(rename("and").as_deref() == Some("`and` is a reserved word of PDDL"));
    check!(rename("Either").as_deref() == Some("`Either` is a reserved word of PDDL"));
    check!(rename("object").as_deref() == Some("`object` is a reserved word of PDDL"));
    check!(rename("Q").as_deref() == Some("predicate `Q` already exists"));
    check!(rename("P").is_none());
    check!(rename("at").is_none());
}

/// A client that can register watchers dynamically is asked to watch `.pddl` files.
#[test]
fn registers_a_file_watcher() {
    let capabilities =
        json!({ "workspace": { "didChangeWatchedFiles": { "dynamicRegistration": true } } });
    let (client, _) = Client::start_with(json!({ "capabilities": capabilities }));
    let request = loop {
        if let Message::Request(r) = client.recv() {
            break r;
        }
    };
    check!(request.method == "client/registerCapability");
    let registration = &request.params["registrations"][0];
    check!(registration["method"] == "workspace/didChangeWatchedFiles");
    check!(registration["registerOptions"]["watchers"][0]["globPattern"] == "**/*.pddl");
    let reply = Response::new_ok(request.id, Value::Null);
    client.connection.sender.send(reply.into()).unwrap();
}

/// Rename reads closed files again when they changed on disk unannounced.
#[test]
fn rename_uses_the_current_text_of_closed_files() {
    let dir = TempDir::new("rename");
    std::fs::write(dir.0.join("domain.pddl"), DOMAIN).unwrap();
    let folder = json!({ "uri": dir.uri(""), "name": "workspace" });
    let mut client =
        Client::start_with(json!({ "capabilities": {}, "workspaceFolders": [folder] })).0;
    let p = dir.uri("problem.pddl");
    client.open(&p, PROBLEM);
    std::fs::write(dir.0.join("domain.pddl"), format!(";; moved\n{DOMAIN}")).unwrap();

    let mut params = at(&p, 3, 10);
    params["newName"] = json!("located");
    let edit = client.request("textDocument/rename", params);
    let edits = edit["changes"][dir.uri("domain.pddl")]
        .as_array()
        .unwrap()
        .clone();
    check!(edits[0]["range"]["start"] == json!({ "line": 4, "character": 16 }));
}

#[test]
fn validates_plans_over_the_protocol() {
    // `PROBLEM`'s goal has a wrong arity; a well-formed one is needed to validate.
    let problem = PROBLEM.replace("(at t1 work ham)", "(at t1 work)");
    let mut client = Client::start();
    let valid = client.request(
        "pddl/validatePlan",
        json!({ "domain": DOMAIN, "problem": problem, "plan": "(drive t1 home work)\n" }),
    );
    check!(
        valid
            == json!({
                "status": "valid", "message": "", "line": null, "advice": null,
                "metric": null, "minimize": null
            })
    );

    let invalid = client.request(
        "pddl/validatePlan",
        json!({ "domain": DOMAIN, "problem": problem, "plan": "(drive t1 work home)\n" }),
    );
    check!(invalid["status"] == "invalid");
    check!(invalid["line"] == 0);

    let bad = client.response("pddl/validatePlan", json!({ "domain": DOMAIN }));
    check!(bad.error.unwrap().code == -32602);
}
