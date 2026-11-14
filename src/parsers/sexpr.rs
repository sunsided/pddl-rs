//! Reads PDDL text into a tree of s-expressions: lists in parentheses and the
//! atoms between them, each with its position in the input.
//!
//! This is the only place that looks at characters. The grammar works on the
//! tree, so every error it reports can point at a whole atom or list.

use crate::parsers::error::Error;

/// The deepest `(` nesting the parser accepts: each `(` still open counts once.
/// The grammar recurses once per level, and a stack overflow aborts the
/// process, so deeper input is an error instead. Real files nest about 8 deep.
pub(crate) const MAX_NESTING_DEPTH: usize = 64;

/// An atom or a parenthesized list, with its position in the input.
#[derive(Debug)]
pub(crate) struct Node<'a> {
    /// Byte offset of the atom or of the list's `(`.
    pub start: usize,
    /// For an atom, the byte offset just past it. For a list, the byte offset
    /// of its `)`, or the input length if the list is never closed.
    pub end: usize,
    /// `false` for a list that is never closed.
    pub closed: bool,
    /// Whether this is an atom or a list, with its content.
    pub kind: Kind<'a>,
}

/// The content of a [`Node`].
#[derive(Debug)]
pub(crate) enum Kind<'a> {
    /// Any run of characters other than whitespace, `(`, `)` and `;`.
    Atom(&'a str),
    /// A parenthesized list and its children, in order.
    List(Vec<Node<'a>>),
}

/// The top-level nodes of an input, and the first unbalanced parenthesis, if any.
#[derive(Debug)]
pub(crate) struct Forest<'a> {
    /// The nodes outside any parentheses, in order.
    pub nodes: Vec<Node<'a>>,
    /// An unclosed `(` or a stray `)`. Reported only after the grammar accepted
    /// the nodes before it, because a mistake inside a form is usually closer
    /// to the real cause than a missing parenthesis at the end of the input.
    pub unbalanced: Option<Error>,
}

/// Splits `input` into atoms and lists.
///
/// Only a nesting deeper than [`MAX_NESTING_DEPTH`] fails right away. A list that
/// is never closed ends at the end of the input, and reading stops at a stray
/// `)`; both are recorded in [`Forest::unbalanced`].
pub(crate) fn read(input: &str) -> Result<Forest<'_>, Error> {
    let bytes = input.as_bytes();
    // Lists still open: their `(` offset and the children read so far.
    let mut open: Vec<(usize, Vec<Node>)> = Vec::new();
    let mut top = Vec::new();
    let mut i = 0;
    while i < bytes.len() {
        match bytes[i] {
            b if b.is_ascii_whitespace() => i += 1,
            b';' => {
                while i < bytes.len() && bytes[i] != b'\n' && bytes[i] != b'\r' {
                    i += 1;
                }
            }
            b'(' => {
                if open.len() == MAX_NESTING_DEPTH {
                    return Err(Error::at(i, 1).reason("nesting too deep").fatal());
                }
                open.push((i, Vec::new()));
                i += 1;
            }
            b')' => {
                let Some((start, children)) = open.pop() else {
                    let unbalanced = Some(Error::at(i, 1));
                    return Ok(Forest {
                        nodes: top,
                        unbalanced,
                    });
                };
                let node = Node {
                    start,
                    end: i,
                    closed: true,
                    kind: Kind::List(children),
                };
                open.last_mut().map_or(&mut top, |(_, c)| c).push(node);
                i += 1;
            }
            _ => {
                let start = i;
                // No name, number or variable starts with an operator character, so an
                // operator such as `-` in `(-x)` or `?a -t` ends where its characters do.
                // A `?` or `:` can only start an atom (a variable or a keyword), so
                // one starts a new atom: `(on?x ?y)` reads as `(on ?x ?y)`.
                let in_atom: fn(u8) -> bool = if is_operator(bytes[i]) {
                    is_operator
                } else {
                    |b| !is_delimiter(b) && b != b'?' && b != b':'
                };
                i += 1;
                while i < bytes.len() && in_atom(bytes[i]) {
                    i += 1;
                }
                let node = Node {
                    start,
                    end: i,
                    closed: true,
                    kind: Kind::Atom(&input[start..i]),
                };
                open.last_mut().map_or(&mut top, |(_, c)| c).push(node);
            }
        }
    }
    // Close what is still open at the end of the input, innermost first.
    let unbalanced = innermost_keyword(&open).map(|keyword| {
        let error = Error::at(input.len(), 0).reason("expected `)`");
        match keyword {
            Some(keyword) => error.keyword(keyword.to_ascii_lowercase()),
            None => error,
        }
    });
    while let Some((start, children)) = open.pop() {
        let node = Node {
            start,
            end: input.len(),
            closed: false,
            kind: Kind::List(children),
        };
        open.last_mut().map_or(&mut top, |(_, c)| c).push(node);
    }
    Ok(Forest {
        nodes: top,
        unbalanced,
    })
}

/// `None` if no list is open; otherwise the `:keyword` heading the innermost
/// open list that has one, such as `:action`.
fn innermost_keyword<'a>(open: &[(usize, Vec<Node<'a>>)]) -> Option<Option<&'a str>> {
    if open.is_empty() {
        return None;
    }
    let keyword = open
        .iter()
        .rev()
        .find_map(|(_, children)| match children.first() {
            Some(Node {
                kind: Kind::Atom(head),
                ..
            }) if head.starts_with(':') => Some(*head),
            _ => None,
        });
    Some(keyword)
}

/// Bytes of the arithmetic and comparison operators `+ - * / < > <= >= =`.
const fn is_operator(b: u8) -> bool {
    matches!(b, b'+' | b'-' | b'*' | b'/' | b'<' | b'>' | b'=')
}

/// Bytes that end an atom. Everything else, including non-ASCII characters,
/// belongs to it; the grammar decides whether the atom is valid.
const fn is_delimiter(b: u8) -> bool {
    b.is_ascii_whitespace() || matches!(b, b'(' | b')' | b';')
}

impl<'a> Node<'a> {
    /// The atom's text, or `None` for a list.
    pub fn atom(&self) -> Option<&'a str> {
        match self.kind {
            Kind::Atom(text) => Some(text),
            Kind::List(_) => None,
        }
    }

    /// Whether this is the atom `word`, ignoring ASCII case.
    pub fn is(&self, word: &str) -> bool {
        self.atom().is_some_and(|a| a.eq_ignore_ascii_case(word))
    }

    /// The list's children, or `None` for an atom.
    pub fn children(&self) -> Option<&[Node<'a>]> {
        match &self.kind {
            Kind::List(children) => Some(children),
            Kind::Atom(_) => None,
        }
    }

    /// The list's first child if it is an atom, such as `and` in `(and ...)`.
    pub fn head(&self) -> Option<&'a str> {
        self.children()?.first()?.atom()
    }

    /// An error at this node. It covers an atom, or a list's `(` and head atom.
    pub fn error(&self) -> Error {
        let end = match &self.kind {
            Kind::Atom(_) => self.end,
            Kind::List(children) => match children.first() {
                Some(
                    first @ Node {
                        kind: Kind::Atom(_),
                        ..
                    },
                ) => first.end,
                _ => self.start + 1,
            },
        };
        Error::at(self.start, end - self.start)
    }

    /// An error at this list's `)`, or at the end of the input if it has none.
    pub fn error_at_close(&self) -> Error {
        Error::at(self.end, usize::from(self.closed))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert2::check;

    fn shape(nodes: &[Node]) -> String {
        nodes
            .iter()
            .map(|n| match &n.kind {
                Kind::Atom(a) => (*a).to_string(),
                Kind::List(c) => format!("[{}]", shape(c)),
            })
            .collect::<Vec<_>>()
            .join(" ")
    }

    #[test]
    fn reads_atoms_lists_and_comments() {
        let forest = read("(a ?b(c) ; (not this)\n :d-e 1.5)x").unwrap();
        check!(shape(&forest.nodes) == "[a ?b [c] :d-e 1.5] x");
        check!(forest.unbalanced.is_none());
    }

    #[test]
    fn operators_need_no_space_after_them() {
        let forest = read("(+x 1) (-x) (>=(f) 1) (=a b) (?a -t) (a-b ?c-d)").unwrap();
        check!(shape(&forest.nodes) == "[+ x 1] [- x] [>= [f] 1] [= a b] [?a - t] [a-b ?c-d]");
    }

    #[test]
    fn a_question_mark_or_colon_starts_a_new_atom() {
        let forest = read("(on?x ?y?z ?duration :requirements:strips)").unwrap();
        check!(shape(&forest.nodes) == "[on ?x ?y ?z ?duration :requirements :strips]");
    }

    #[test]
    fn positions_cover_atoms_and_parentheses() {
        let forest = read(" (ab (c))").unwrap();
        let outer = &forest.nodes[0];
        check!((outer.start, outer.end) == (1, 8));
        let inner = &outer.children().unwrap()[1];
        check!((inner.start, inner.end) == (5, 7));
        let atom = &outer.children().unwrap()[0];
        check!((atom.start, atom.end) == (2, 4));
    }

    #[test]
    fn unclosed_lists_end_at_the_end_of_input() {
        let forest = read("(:action a (p)").unwrap();
        check!(shape(&forest.nodes) == "[:action a [p]]");
        let error = forest.unbalanced.unwrap();
        check!(error.offset == 14);
        check!(error.keyword.as_deref() == Some(":action"));
        check!(error.reason == Some("expected `)`"));
    }

    #[test]
    fn reading_stops_at_a_stray_close() {
        let forest = read("(a)) (b)").unwrap();
        check!(shape(&forest.nodes) == "[a]");
        check!(forest.unbalanced.unwrap().offset == 3);
    }

    #[test]
    fn nesting_is_limited() {
        let ok = "(".repeat(MAX_NESTING_DEPTH) + &")".repeat(MAX_NESTING_DEPTH);
        check!(read(&ok).is_ok());
        let deep = "(".repeat(MAX_NESTING_DEPTH + 1);
        let error = read(&deep).unwrap_err();
        check!(error.offset == MAX_NESTING_DEPTH);
        check!(error.reason == Some("nesting too deep"));
    }
}
