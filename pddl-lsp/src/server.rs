//! Answers LSP requests from the editor about the [`Workspace`].

use std::collections::HashMap;
use std::error::Error;
use std::fmt::Write as _;
use std::ops::Range;
use std::path::Path;

use lsp_server::{Connection, ErrorCode, Message, Notification, Request, Response};
use lsp_types::notification::{
    DidChangeTextDocument, DidChangeWatchedFiles, DidCloseTextDocument, DidOpenTextDocument,
    Notification as _, PublishDiagnostics,
};
use lsp_types::request::{
    CodeActionRequest, Completion, DocumentHighlightRequest, DocumentSymbolRequest,
    FoldingRangeRequest, Formatting, GotoDefinition, HoverRequest, PrepareRenameRequest,
    References, Rename, Request as _, SemanticTokensFullRequest, SignatureHelpRequest,
    WorkspaceSymbolRequest,
};
use lsp_types::{
    CodeAction, CodeActionKind, CodeActionOrCommand, CodeActionParams,
    CodeActionProviderCapability, CompletionItem, CompletionItemKind, CompletionOptions,
    CompletionParams, CompletionResponse, Diagnostic, DiagnosticSeverity,
    DidChangeTextDocumentParams, DidChangeWatchedFilesParams, DidCloseTextDocumentParams,
    DidOpenTextDocumentParams, DocumentFormattingParams, DocumentHighlight,
    DocumentHighlightParams, DocumentSymbol, DocumentSymbolParams, DocumentSymbolResponse,
    FileChangeType, FoldingRange, FoldingRangeParams, FoldingRangeProviderCapability,
    GotoDefinitionParams, GotoDefinitionResponse, Hover, HoverContents, HoverParams,
    HoverProviderCapability, InitializeParams, Location, MarkupContent, MarkupKind, OneOf,
    ParameterInformation, ParameterLabel, PositionEncodingKind, PrepareRenameResponse,
    PublishDiagnosticsParams, ReferenceParams, RenameOptions, RenameParams, SemanticToken,
    SemanticTokenModifier, SemanticTokenType, SemanticTokens, SemanticTokensFullOptions,
    SemanticTokensLegend, SemanticTokensOptions, SemanticTokensParams, SemanticTokensResult,
    SemanticTokensServerCapabilities, ServerCapabilities, ServerInfo, SignatureHelp,
    SignatureHelpOptions, SignatureHelpParams, SignatureInformation, SymbolKind,
    TextDocumentIdentifier, TextDocumentPositionParams, TextDocumentSyncCapability,
    TextDocumentSyncKind, TextEdit, Url, WorkspaceEdit, WorkspaceSymbolParams,
    WorkspaceSymbolResponse,
};
use pddl::syntax::{self, Node};
use pddl::{Name, Parser, Variable};
use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::analysis::{node_range, Definition, Expect, Kind, Occurrence};
use crate::format::format;
use crate::line_index::Encoding;
use crate::workspace::{Document, Severity, Target, Workspace};

/// Files larger than this are not read from the workspace folders.
const MAX_FILE_SIZE: u64 = 10 * 1024 * 1024;
/// At most this many files are read from the workspace folders.
const MAX_FILES: usize = 10_000;

/// The sections of a domain, for completion.
const DOMAIN_SECTIONS: &[&str] = &[
    ":requirements",
    ":types",
    ":constants",
    ":predicates",
    ":functions",
    ":constraints",
    ":action",
    ":durative-action",
    ":derived",
    ":extends",
    ":timeless",
];
/// The sections of a problem, for completion.
const PROBLEM_SECTIONS: &[&str] = &[
    ":domain",
    ":requirements",
    ":objects",
    ":init",
    ":goal",
    ":constraints",
    ":metric",
    ":length",
];
/// The keywords of an action, for completion.
const ACTION_KEYWORDS: &[&str] = &[":parameters", ":precondition", ":effect"];
/// The keywords of a durative action, for completion.
const DURATIVE_ACTION_KEYWORDS: &[&str] = &[":parameters", ":duration", ":condition", ":effect"];
/// The requirements PDDL 3.1 defines, for completion.
const REQUIREMENTS: &[&str] = &[
    ":strips",
    ":typing",
    ":negative-preconditions",
    ":disjunctive-preconditions",
    ":equality",
    ":existential-preconditions",
    ":universal-preconditions",
    ":quantified-preconditions",
    ":conditional-effects",
    ":fluents",
    ":numeric-fluents",
    ":object-fluents",
    ":adl",
    ":durative-actions",
    ":duration-inequalities",
    ":continuous-effects",
    ":derived-predicates",
    ":timed-initial-literals",
    ":preferences",
    ":constraints",
    ":action-costs",
];
/// The words that can start a formula, for completion after `(`.
const FORMULA_WORDS: &[&str] = &[
    "and",
    "or",
    "not",
    "imply",
    "forall",
    "exists",
    "when",
    "at start",
    "at end",
    "over all",
    "preference",
    "increase",
    "decrease",
    "assign",
    "scale-up",
    "scale-down",
    "always",
    "sometime",
    "within",
    "at-most-once",
    "sometime-after",
    "sometime-before",
    "always-within",
    "hold-during",
    "hold-after",
    "either",
];

/// The semantic token types, in legend order.
const TOKEN_TYPES: &[SemanticTokenType] = &[
    SemanticTokenType::NAMESPACE,
    SemanticTokenType::TYPE,
    SemanticTokenType::ENUM_MEMBER,
    SemanticTokenType::FUNCTION,
    SemanticTokenType::PROPERTY,
    SemanticTokenType::METHOD,
    SemanticTokenType::PARAMETER,
];

/// The language server's state.
pub struct Server {
    /// Every known PDDL file.
    workspace: Workspace,
    /// How the client counts columns.
    encoding: Encoding,
}

/// Runs the server on `connection` until the client shuts it down.
///
/// ## Errors
///
/// Fails if the connection breaks or the client sends a malformed initialization.
pub fn run(connection: &Connection) -> Result<(), Box<dyn Error + Send + Sync>> {
    let (id, params) = connection.initialize_start()?;
    let params: InitializeParams = serde_json::from_value(params)?;
    let utf8 = params
        .capabilities
        .general
        .as_ref()
        .and_then(|g| g.position_encodings.as_ref())
        .is_some_and(|e| e.contains(&PositionEncodingKind::UTF8));
    let encoding = if utf8 {
        Encoding::Utf8
    } else {
        Encoding::Utf16
    };
    let result = serde_json::json!({
        "capabilities": capabilities(encoding),
        "serverInfo": ServerInfo {
            name: env!("CARGO_PKG_NAME").into(),
            version: Some(env!("CARGO_PKG_VERSION").into()),
        },
    });
    connection.initialize_finish(id, result)?;

    let mut server = Server {
        workspace: Workspace::default(),
        encoding,
    };
    #[allow(deprecated)] // `root_uri` is still what clients without folders send.
    let roots = match params.workspace_folders {
        Some(folders) => folders.into_iter().map(|f| f.uri).collect(),
        None => params.root_uri.into_iter().collect::<Vec<_>>(),
    };
    for root in roots {
        if let Ok(path) = root.to_file_path() {
            server.scan(&path);
        }
    }

    for message in &connection.receiver {
        match message {
            Message::Request(request) => {
                if connection.handle_shutdown(&request)? {
                    return Ok(());
                }
                let response = server.request(request);
                connection.sender.send(Message::Response(response))?;
            }
            Message::Notification(notification) => {
                if server.notification(notification) {
                    for diagnostics in server.diagnostics() {
                        let params = serde_json::to_value(diagnostics)?;
                        let message = Notification::new(PublishDiagnostics::METHOD.into(), params);
                        connection.sender.send(Message::Notification(message))?;
                    }
                }
            }
            Message::Response(_) => {}
        }
    }
    Ok(())
}

/// What the server can do.
fn capabilities(encoding: Encoding) -> ServerCapabilities {
    ServerCapabilities {
        position_encoding: Some(match encoding {
            Encoding::Utf8 => PositionEncodingKind::UTF8,
            Encoding::Utf16 => PositionEncodingKind::UTF16,
        }),
        text_document_sync: Some(TextDocumentSyncCapability::Kind(TextDocumentSyncKind::FULL)),
        hover_provider: Some(HoverProviderCapability::Simple(true)),
        definition_provider: Some(OneOf::Left(true)),
        references_provider: Some(OneOf::Left(true)),
        document_highlight_provider: Some(OneOf::Left(true)),
        rename_provider: Some(OneOf::Right(RenameOptions {
            prepare_provider: Some(true),
            work_done_progress_options: lsp_types::WorkDoneProgressOptions::default(),
        })),
        document_symbol_provider: Some(OneOf::Left(true)),
        workspace_symbol_provider: Some(OneOf::Left(true)),
        completion_provider: Some(CompletionOptions {
            trigger_characters: Some(vec!["(".into(), "?".into(), ":".into()]),
            ..CompletionOptions::default()
        }),
        signature_help_provider: Some(SignatureHelpOptions {
            trigger_characters: Some(vec!["(".into(), " ".into()]),
            ..SignatureHelpOptions::default()
        }),
        document_formatting_provider: Some(OneOf::Left(true)),
        code_action_provider: Some(CodeActionProviderCapability::Simple(true)),
        folding_range_provider: Some(FoldingRangeProviderCapability::Simple(true)),
        semantic_tokens_provider: Some(SemanticTokensServerCapabilities::SemanticTokensOptions(
            SemanticTokensOptions {
                legend: SemanticTokensLegend {
                    token_types: TOKEN_TYPES.to_vec(),
                    token_modifiers: vec![SemanticTokenModifier::DECLARATION],
                },
                full: Some(SemanticTokensFullOptions::Bool(true)),
                ..SemanticTokensOptions::default()
            },
        )),
        ..ServerCapabilities::default()
    }
}

/// Parses `params` and runs `handler` on them.
fn call<P: DeserializeOwned, R: Serialize>(
    server: &Server,
    params: serde_json::Value,
    handler: impl FnOnce(&Server, P) -> R,
) -> Result<serde_json::Value, String> {
    let params = serde_json::from_value(params).map_err(|e| e.to_string())?;
    serde_json::to_value(handler(server, params)).map_err(|e| e.to_string())
}

// The handlers take their parameters by value, as `call` passes them.
#[allow(clippy::needless_pass_by_value, clippy::unnecessary_wraps)]
impl Server {
    /// Creates a server without documents, for tests.
    #[must_use]
    pub fn new(encoding: Encoding) -> Self {
        Self {
            workspace: Workspace::default(),
            encoding,
        }
    }

    /// Answers `request`.
    #[must_use]
    pub fn request(&self, request: Request) -> Response {
        let params = request.params;
        let result = match request.method.as_str() {
            HoverRequest::METHOD => call(self, params, Self::hover),
            GotoDefinition::METHOD => call(self, params, Self::definition),
            References::METHOD => call(self, params, Self::references),
            DocumentHighlightRequest::METHOD => call(self, params, Self::highlights),
            PrepareRenameRequest::METHOD => call(self, params, Self::prepare_rename),
            Rename::METHOD => match serde_json::from_value(params) {
                Ok(params) => self
                    .rename(params)
                    .map(|edit| serde_json::to_value(edit).unwrap_or_default()),
                Err(e) => Err(e.to_string()),
            },
            DocumentSymbolRequest::METHOD => call(self, params, Self::document_symbols),
            WorkspaceSymbolRequest::METHOD => call(self, params, Self::workspace_symbols),
            Completion::METHOD => call(self, params, Self::completion),
            SignatureHelpRequest::METHOD => call(self, params, Self::signature_help),
            Formatting::METHOD => call(self, params, Self::formatting),
            CodeActionRequest::METHOD => call(self, params, Self::code_actions),
            FoldingRangeRequest::METHOD => call(self, params, Self::folding_ranges),
            SemanticTokensFullRequest::METHOD => call(self, params, Self::semantic_tokens),
            _ => {
                return Response::new_err(
                    request.id,
                    ErrorCode::MethodNotFound as i32,
                    format!("unsupported request `{}`", request.method),
                )
            }
        };
        match result {
            Ok(value) => Response::new_ok(request.id, value),
            Err(message) => Response::new_err(request.id, ErrorCode::InvalidParams as i32, message),
        }
    }

    /// Applies `notification`, and returns whether documents changed.
    pub fn notification(&mut self, notification: Notification) -> bool {
        let params = notification.params;
        match notification.method.as_str() {
            DidOpenTextDocument::METHOD => {
                let Ok(params) = serde_json::from_value::<DidOpenTextDocumentParams>(params) else {
                    return false;
                };
                let document = params.text_document;
                self.open(document.uri, document.text);
            }
            DidChangeTextDocument::METHOD => {
                let Ok(params) = serde_json::from_value::<DidChangeTextDocumentParams>(params)
                else {
                    return false;
                };
                // Full sync: the last change holds the whole text.
                if let Some(change) = params.content_changes.into_iter().last() {
                    self.open(params.text_document.uri, change.text);
                }
            }
            DidCloseTextDocument::METHOD => {
                let Ok(params) = serde_json::from_value::<DidCloseTextDocumentParams>(params)
                else {
                    return false;
                };
                let uri = params.text_document.uri;
                if let Some(document) = self.workspace.documents.get_mut(&uri) {
                    document.open = false;
                }
                self.reload(&uri);
            }
            DidChangeWatchedFiles::METHOD => {
                let Ok(params) = serde_json::from_value::<DidChangeWatchedFilesParams>(params)
                else {
                    return false;
                };
                for change in params.changes {
                    let open = self
                        .workspace
                        .documents
                        .get(&change.uri)
                        .is_some_and(|d| d.open);
                    if open {
                        continue;
                    }
                    if change.typ == FileChangeType::DELETED {
                        self.workspace.documents.remove(&change.uri);
                    } else {
                        self.reload(&change.uri);
                    }
                }
            }
            _ => return false,
        }
        true
    }

    /// Sets the text of the open document `uri`.
    pub fn open(&mut self, uri: Url, text: String) {
        let document = Document::new(text, self.encoding, true);
        self.workspace.documents.insert(uri, document);
    }

    /// Reads a closed document `uri` from the disk again, or forgets it if it's gone.
    fn reload(&mut self, uri: &Url) {
        let text = uri
            .to_file_path()
            .ok()
            .and_then(|p| std::fs::read_to_string(p).ok());
        match text {
            Some(text) => {
                let document = Document::new(text, self.encoding, false);
                self.workspace.documents.insert(uri.clone(), document);
            }
            None => {
                self.workspace.documents.remove(uri);
            }
        }
    }

    /// Reads the `.pddl` files under `root`, skipping hidden and build folders.
    fn scan(&mut self, root: &Path) {
        let mut pending = vec![root.to_path_buf()];
        let mut read = 0;
        while let Some(dir) = pending.pop() {
            let Ok(entries) = std::fs::read_dir(&dir) else {
                continue;
            };
            for entry in entries.flatten() {
                let path = entry.path();
                let name = entry.file_name();
                let name = name.to_string_lossy();
                let Ok(kind) = entry.file_type() else {
                    continue;
                };
                if kind.is_dir() {
                    if !name.starts_with('.') && name != "target" && name != "node_modules" {
                        pending.push(path);
                    }
                    continue;
                }
                let is_pddl = path
                    .extension()
                    .is_some_and(|e| e.eq_ignore_ascii_case("pddl"));
                let small = entry.metadata().is_ok_and(|m| m.len() <= MAX_FILE_SIZE);
                if !kind.is_file() || !is_pddl || !small {
                    continue;
                }
                if read == MAX_FILES {
                    return;
                }
                if let (Ok(text), Ok(uri)) =
                    (std::fs::read_to_string(&path), Url::from_file_path(&path))
                {
                    let document = Document::new(text, self.encoding, false);
                    self.workspace.documents.insert(uri, document);
                    read += 1;
                }
            }
        }
    }

    /// The diagnostics of every open document, and empty ones for closed documents.
    ///
    /// All open documents are checked again after any change, since a change to
    /// a domain changes what its problems may use.
    #[must_use]
    pub fn diagnostics(&self) -> Vec<PublishDiagnosticsParams> {
        self.workspace
            .documents
            .iter()
            .map(|(uri, document)| {
                let diagnostics = if document.open {
                    self.workspace
                        .problems(uri)
                        .into_iter()
                        .map(|problem| Diagnostic {
                            range: self.range(document, problem.span),
                            severity: Some(match problem.severity {
                                Severity::Error => DiagnosticSeverity::ERROR,
                                Severity::Warning => DiagnosticSeverity::WARNING,
                            }),
                            source: Some("pddl".into()),
                            message: problem.message,
                            ..Diagnostic::default()
                        })
                        .collect()
                } else {
                    Vec::new()
                };
                PublishDiagnosticsParams::new(uri.clone(), diagnostics, None)
            })
            .collect()
    }

    /// The document `uri` and the byte offset of `position` in it.
    fn locate(&self, at: &TextDocumentPositionParams) -> Option<(&Document, usize)> {
        let document = self.workspace.documents.get(&at.text_document.uri)?;
        let offset = document.lines.offset(&document.text, at.position);
        Some((document, offset))
    }

    /// The LSP range of `span` in `document`.
    fn range(&self, document: &Document, span: Range<usize>) -> lsp_types::Range {
        let _ = self;
        document.lines.range(&document.text, span)
    }

    /// The location of `span` in `uri`.
    fn location(&self, uri: &Url, span: Range<usize>) -> Option<Location> {
        let document = self.workspace.documents.get(uri)?;
        Some(Location::new(uri.clone(), self.range(document, span)))
    }

    /// The definition the name at `at` refers to.
    fn target(&self, at: &TextDocumentPositionParams) -> Option<Target> {
        let (_, offset) = self.locate(at)?;
        self.workspace.target_at(&at.text_document.uri, offset)
    }

    /// The definition's summary for the name under the cursor.
    fn hover(&self, params: HoverParams) -> Option<Hover> {
        let at = &params.text_document_position_params;
        let target = self.target(at)?;
        let definition = self.workspace.definition(&target)?;
        let document = &self.workspace.documents[&target.uri];
        let owner = &document.analysis.contexts[definition.context];
        let mut value = format!("```pddl\n{}\n```", definition.detail);
        if !matches!(
            definition.kind,
            Kind::Domain | Kind::Problem | Kind::Variable
        ) {
            let _ = write!(
                value,
                "\n\n{} in {} `{}`",
                definition.kind.label(),
                owner.kind.label(),
                owner.name
            );
        }
        let (document, offset) = self.locate(at)?;
        let span = match document.analysis.occurrence_at(offset)? {
            Occurrence::Definition(i) => document.analysis.definitions[i].span.clone(),
            Occurrence::Reference(i) => document.analysis.references[i].span.clone(),
            Occurrence::Variable(i) => document.analysis.variables[i].span.clone(),
        };
        Some(Hover {
            contents: HoverContents::Markup(MarkupContent {
                kind: MarkupKind::Markdown,
                value,
            }),
            range: Some(self.range(document, span)),
        })
    }

    /// Where the name under the cursor is defined.
    fn definition(&self, params: GotoDefinitionParams) -> Option<GotoDefinitionResponse> {
        let target = self.target(&params.text_document_position_params)?;
        let definition = self.workspace.definition(&target)?;
        let location = self.location(&target.uri, definition.span.clone())?;
        Some(GotoDefinitionResponse::Scalar(location))
    }

    /// Every use of the name under the cursor.
    fn references(&self, params: ReferenceParams) -> Option<Vec<Location>> {
        let target = self.target(&params.text_document_position)?;
        let skip = usize::from(!params.context.include_declaration);
        let found = self.workspace.references(&target);
        Some(
            found
                .into_iter()
                .skip(skip)
                .filter_map(|(uri, span)| self.location(&uri, span))
                .collect(),
        )
    }

    /// The uses of the name under the cursor in its document.
    fn highlights(&self, params: DocumentHighlightParams) -> Option<Vec<DocumentHighlight>> {
        let at = &params.text_document_position_params;
        let target = self.target(at)?;
        let (document, _) = self.locate(at)?;
        let found = self.workspace.references(&target);
        Some(
            found
                .into_iter()
                .filter(|(uri, _)| *uri == at.text_document.uri)
                .map(|(_, span)| DocumentHighlight {
                    range: self.range(document, span),
                    kind: None,
                })
                .collect(),
        )
    }

    /// The range of the name under the cursor, if it can be renamed.
    fn prepare_rename(&self, params: TextDocumentPositionParams) -> Option<PrepareRenameResponse> {
        let (document, offset) = self.locate(&params)?;
        self.target(&params)?;
        let span = match document.analysis.occurrence_at(offset)? {
            Occurrence::Definition(i) => document.analysis.definitions[i].span.clone(),
            Occurrence::Reference(i) => document.analysis.references[i].span.clone(),
            Occurrence::Variable(i) => document.analysis.variables[i].span.clone(),
        };
        Some(PrepareRenameResponse::Range(self.range(document, span)))
    }

    /// Renames the name under the cursor everywhere it is used.
    ///
    /// ## Errors
    ///
    /// Fails if the new name is not a valid name, or variable for a variable.
    pub fn rename(&self, params: RenameParams) -> Result<Option<WorkspaceEdit>, String> {
        let Some(target) = self.target(&params.text_document_position) else {
            return Ok(None);
        };
        let Some(definition) = self.workspace.definition(&target) else {
            return Ok(None);
        };
        let new_name = params.new_name;
        let valid = if definition.kind == Kind::Variable {
            Variable::from_str(&new_name).is_ok()
        } else {
            !new_name.starts_with('?') && Name::from_str(&new_name).is_ok()
        };
        if !valid {
            return Err(format!(
                "`{new_name}` is not a valid {} name",
                definition.kind.label()
            ));
        }
        let mut changes: HashMap<Url, Vec<TextEdit>> = HashMap::new();
        for (uri, span) in self.workspace.references(&target) {
            if let Some(location) = self.location(&uri, span) {
                let edit = TextEdit::new(location.range, new_name.clone());
                changes.entry(uri).or_default().push(edit);
            }
        }
        Ok(Some(WorkspaceEdit::new(changes)))
    }

    /// The outline of a document: its domains and problems and what they define.
    fn document_symbols(&self, params: DocumentSymbolParams) -> Option<DocumentSymbolResponse> {
        let document = self.workspace.documents.get(&params.text_document.uri)?;
        let analysis = &document.analysis;
        #[allow(deprecated)] // `deprecated` has no replacement but must be set.
        let symbol = |definition: &Definition, children| DocumentSymbol {
            name: definition.name.clone(),
            detail: Some(definition.detail.clone()),
            kind: symbol_kind(definition.kind),
            tags: None,
            deprecated: None,
            range: self.range(document, definition.range.clone()),
            selection_range: self.range(document, definition.span.clone()),
            children,
        };
        let roots = analysis
            .definitions
            .iter()
            .enumerate()
            .filter(|(_, d)| matches!(d.kind, Kind::Domain | Kind::Problem))
            .map(|(_, root)| {
                let children = analysis
                    .definitions
                    .iter()
                    .filter(|d| {
                        d.context == root.context
                            && !matches!(d.kind, Kind::Domain | Kind::Problem | Kind::Variable)
                    })
                    .map(|d| symbol(d, None))
                    .collect();
                symbol(root, Some(children))
            })
            .collect();
        Some(DocumentSymbolResponse::Nested(roots))
    }

    /// The definitions across the workspace whose name contains the query.
    fn workspace_symbols(&self, params: WorkspaceSymbolParams) -> Option<WorkspaceSymbolResponse> {
        let query = params.query.to_ascii_lowercase();
        let mut symbols = Vec::new();
        for (uri, document) in &self.workspace.documents {
            for definition in &document.analysis.definitions {
                if definition.kind == Kind::Variable
                    || !definition.name.to_ascii_lowercase().contains(&query)
                {
                    continue;
                }
                let owner = &document.analysis.contexts[definition.context];
                #[allow(deprecated)] // `deprecated` has no replacement but must be set.
                symbols.push(lsp_types::SymbolInformation {
                    name: definition.name.clone(),
                    kind: symbol_kind(definition.kind),
                    tags: None,
                    deprecated: None,
                    location: Location::new(
                        uri.clone(),
                        self.range(document, definition.span.clone()),
                    ),
                    container_name: Some(owner.name.clone()),
                });
            }
        }
        Some(WorkspaceSymbolResponse::Flat(symbols))
    }

    /// Words that fit at the cursor.
    fn completion(&self, params: CompletionParams) -> Option<CompletionResponse> {
        let at = &params.text_document_position;
        let (document, offset) = self.locate(at)?;
        let text = &document.text;
        let start = text[..offset]
            .rfind(|c: char| c.is_whitespace() || "();".contains(c))
            .map_or(0, |i| i + 1);
        let word = &text[start..offset];
        let before = text[..start].trim_end();
        let tree = syntax::read(text).ok()?;
        let context = document.analysis.context_at(offset);
        let item = |label: &str, kind, detail: Option<&str>| CompletionItem {
            label: label.to_string(),
            kind: Some(kind),
            detail: detail.map(str::to_string),
            ..CompletionItem::default()
        };
        let visible = |kinds: &[Kind]| -> Vec<CompletionItem> {
            context
                .map(|c| self.workspace.visible(&at.text_document.uri, c))
                .unwrap_or_default()
                .into_iter()
                .filter(|d| kinds.contains(&d.kind))
                .map(|d| item(&d.name, completion_kind(d.kind), Some(&d.detail)))
                .collect()
        };
        let items = if word.starts_with('?') {
            let mut seen = std::collections::HashSet::new();
            document
                .analysis
                .definitions
                .iter()
                .filter(|d| {
                    d.scope
                        .as_ref()
                        .is_some_and(|s| s.start < offset && offset <= s.end)
                })
                .filter(|d| seen.insert(d.name.to_ascii_lowercase()))
                .map(|d| item(&d.name, CompletionItemKind::VARIABLE, Some(&d.detail)))
                .collect()
        } else if word.starts_with(':') {
            let parent = enclosing_lists(&tree.nodes, offset)
                .into_iter()
                .rev()
                .find(|list| list.start + 1 != start)
                .and_then(Node::head)
                .map(str::to_ascii_lowercase);
            let problem =
                context.is_some_and(|c| document.analysis.contexts[c].kind == Kind::Problem);
            let words = match parent.as_deref() {
                Some("define") if problem => PROBLEM_SECTIONS,
                Some("define") => DOMAIN_SECTIONS,
                Some(":action") => ACTION_KEYWORDS,
                Some(":durative-action") => DURATIVE_ACTION_KEYWORDS,
                Some(":requirements") => REQUIREMENTS,
                _ => &[],
            };
            words
                .iter()
                .map(|w| item(w, CompletionItemKind::KEYWORD, None))
                .collect()
        } else if before.ends_with('(') {
            let mut items = visible(&[Kind::Predicate, Kind::Function]);
            items.extend(
                FORMULA_WORDS
                    .iter()
                    .map(|w| item(w, CompletionItemKind::KEYWORD, None)),
            );
            items
        } else if before.ends_with(" -") || before.ends_with("\t-") || before.ends_with("\n-") {
            let mut items = visible(&[Kind::Type]);
            items.push(item("object", CompletionItemKind::CLASS, None));
            items
        } else {
            visible(&[Kind::Object, Kind::Constant])
        };
        Some(CompletionResponse::Array(items))
    }

    /// The parameters of the predicate or function whose arguments are being typed.
    fn signature_help(&self, params: SignatureHelpParams) -> Option<SignatureHelp> {
        let at = &params.text_document_position_params;
        let (document, offset) = self.locate(at)?;
        let tree = syntax::read(&document.text).ok()?;
        let lists = enclosing_lists(&tree.nodes, offset);
        let (list, target) = lists.iter().rev().find_map(|list| {
            let head = list.children()?.first()?;
            let index = document.analysis.references.iter().position(|r| {
                r.span.start == head.start && r.expect == Expect::PredicateOrFunction
            })?;
            Some((*list, self.workspace.resolve(&at.text_document.uri, index)?))
        })?;
        let definition = self.workspace.definition(&target)?;
        let children = list.children()?;
        let active = children[1..]
            .iter()
            .filter(|c| node_range(c).end < offset)
            .count();
        let parameters = definition
            .params
            .iter()
            .map(|p| ParameterInformation {
                label: ParameterLabel::Simple(p.clone()),
                documentation: None,
            })
            .collect();
        Some(SignatureHelp {
            signatures: vec![SignatureInformation {
                label: format!("({} {})", definition.name, definition.params.join(" ")),
                documentation: None,
                parameters: Some(parameters),
                active_parameter: u32::try_from(active).ok(),
            }],
            active_signature: Some(0),
            active_parameter: None,
        })
    }

    /// Formats the whole document, unless its parentheses are unbalanced.
    fn formatting(&self, params: DocumentFormattingParams) -> Option<Vec<TextEdit>> {
        let document = self.workspace.documents.get(&params.text_document.uri)?;
        let formatted = format(&document.text)?;
        if formatted == document.text {
            return Some(Vec::new());
        }
        let range = self.range(document, 0..document.text.len());
        Some(vec![TextEdit::new(range, formatted)])
    }

    /// Fixes for misspelled keywords within the requested range.
    fn code_actions(&self, params: CodeActionParams) -> Option<Vec<CodeActionOrCommand>> {
        let TextDocumentIdentifier { uri } = params.text_document;
        let document = self.workspace.documents.get(&uri)?;
        let start = document.lines.offset(&document.text, params.range.start);
        let end = document.lines.offset(&document.text, params.range.end);
        let actions = self
            .workspace
            .problems(&uri)
            .into_iter()
            .filter(|p| p.span.start <= end && start <= p.span.end)
            .filter_map(|problem| {
                let fix = problem.fix?;
                let range = self.range(document, problem.span);
                let edit = TextEdit::new(range, fix.clone());
                Some(CodeActionOrCommand::CodeAction(CodeAction {
                    title: format!("Replace with `{fix}`"),
                    kind: Some(CodeActionKind::QUICKFIX),
                    edit: Some(WorkspaceEdit::new(HashMap::from([(
                        uri.clone(),
                        vec![edit],
                    )]))),
                    is_preferred: Some(true),
                    ..CodeAction::default()
                }))
            })
            .collect();
        Some(actions)
    }

    /// The lists that span several lines.
    fn folding_ranges(&self, params: FoldingRangeParams) -> Option<Vec<FoldingRange>> {
        let document = self.workspace.documents.get(&params.text_document.uri)?;
        let tree = syntax::read(&document.text).ok()?;
        let mut ranges = Vec::new();
        let mut pending: Vec<&Node> = tree.nodes.iter().collect();
        while let Some(node) = pending.pop() {
            let Some(children) = node.children() else {
                continue;
            };
            let start = document.lines.position(&document.text, node.start).line;
            let end = document.lines.position(&document.text, node.end).line;
            if end > start {
                ranges.push(FoldingRange {
                    start_line: start,
                    end_line: end,
                    ..FoldingRange::default()
                });
            }
            pending.extend(children);
        }
        Some(ranges)
    }

    /// What each name is, for highlighting.
    fn semantic_tokens(&self, params: SemanticTokensParams) -> Option<SemanticTokensResult> {
        let uri = &params.text_document.uri;
        let document = self.workspace.documents.get(uri)?;
        let analysis = &document.analysis;
        let mut tokens: Vec<(Range<usize>, Kind, bool)> = Vec::new();
        for definition in &analysis.definitions {
            tokens.push((definition.span.clone(), definition.kind, true));
        }
        for (index, reference) in analysis.references.iter().enumerate() {
            let kind = self
                .workspace
                .resolve(uri, index)
                .and_then(|t| self.workspace.definition(&t).map(|d| d.kind))
                .unwrap_or(match reference.expect {
                    Expect::PredicateOrFunction => Kind::Predicate,
                    Expect::Term => Kind::Constant,
                    Expect::Type => Kind::Type,
                    Expect::Domain => Kind::Domain,
                });
            tokens.push((reference.span.clone(), kind, false));
        }
        for variable in &analysis.variables {
            tokens.push((variable.span.clone(), Kind::Variable, false));
        }
        tokens.sort_by_key(|(span, ..)| span.start);
        tokens.dedup_by_key(|(span, ..)| span.start);
        let mut data = Vec::with_capacity(tokens.len());
        let (mut line, mut column) = (0, 0);
        for (span, kind, declaration) in tokens {
            let start = document.lines.position(&document.text, span.start);
            let end = document.lines.position(&document.text, span.end);
            let delta_line = start.line - line;
            let delta_start = if delta_line == 0 {
                start.character - column
            } else {
                start.character
            };
            data.push(SemanticToken {
                delta_line,
                delta_start,
                length: end.character.saturating_sub(start.character),
                token_type: token_type(kind),
                token_modifiers_bitset: u32::from(declaration),
            });
            (line, column) = (start.line, start.character);
        }
        Some(SemanticTokensResult::Tokens(SemanticTokens {
            result_id: None,
            data,
        }))
    }
}

/// The lists containing `offset`, outermost first.
fn enclosing_lists<'n, 'a>(nodes: &'n [Node<'a>], offset: usize) -> Vec<&'n Node<'a>> {
    let mut found = Vec::new();
    let mut level = nodes;
    while let Some(list) = level
        .iter()
        .find(|n| n.children().is_some() && n.start < offset && offset <= n.end)
    {
        found.push(list);
        level = list.children().unwrap_or_default();
    }
    found
}

/// The outline symbol for a kind of definition.
const fn symbol_kind(kind: Kind) -> SymbolKind {
    match kind {
        Kind::Domain => SymbolKind::NAMESPACE,
        Kind::Problem => SymbolKind::MODULE,
        Kind::Type => SymbolKind::CLASS,
        Kind::Constant => SymbolKind::CONSTANT,
        Kind::Object => SymbolKind::OBJECT,
        Kind::Predicate => SymbolKind::FUNCTION,
        Kind::Function => SymbolKind::PROPERTY,
        Kind::Action => SymbolKind::METHOD,
        Kind::Variable => SymbolKind::VARIABLE,
    }
}

/// The completion item kind for a kind of definition.
const fn completion_kind(kind: Kind) -> CompletionItemKind {
    match kind {
        Kind::Domain | Kind::Problem => CompletionItemKind::MODULE,
        Kind::Type => CompletionItemKind::CLASS,
        Kind::Constant | Kind::Object => CompletionItemKind::CONSTANT,
        Kind::Predicate => CompletionItemKind::FUNCTION,
        Kind::Function => CompletionItemKind::PROPERTY,
        Kind::Action => CompletionItemKind::METHOD,
        Kind::Variable => CompletionItemKind::VARIABLE,
    }
}

/// The index of a kind's semantic token type in [`TOKEN_TYPES`].
const fn token_type(kind: Kind) -> u32 {
    match kind {
        Kind::Domain | Kind::Problem => 0,
        Kind::Type => 1,
        Kind::Constant | Kind::Object => 2,
        Kind::Predicate => 3,
        Kind::Function => 4,
        Kind::Action => 5,
        Kind::Variable => 6,
    }
}
