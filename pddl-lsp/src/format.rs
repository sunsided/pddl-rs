//! Formats PDDL text, keeping its comments.
//!
//! A list that fits on its line stays on one line, except for a `(define ...)`. Otherwise its head stays
//! after the `(`, each `:keyword value` pair and each nested list starts a new
//! line indented by two spaces, and atoms fill lines. A blank line between two
//! forms is kept, and so are comments, at the end of a line or on their own.

use std::ops::Range;

use pddl::syntax::{self, Node};

use crate::analysis::node_range;

/// The column a formatted line should not go past.
const WIDTH: usize = 80;
/// Spaces per nesting level.
const INDENT: usize = 2;
/// Heads after which the first argument stays on the head's line, even if it is a list.
const HUGGING: &[&str] = &["define", "forall", "exists", ":derived"];

/// Formats `text`, or returns `None` if its parentheses are unbalanced or it
/// nests too deep, since moving text around then could change what it means.
#[must_use]
pub fn format(text: &str) -> Option<String> {
    let tree = syntax::read(text).ok()?;
    let read_to = tree
        .nodes
        .last()
        .map(|n| node_range(n).end)
        .max(tree.comments.last().map(|c| c.end))
        .unwrap_or(0);
    if !text[read_to..].trim().is_empty() || !tree.nodes.iter().all(all_closed) {
        return None;
    }
    let mut writer = Writer {
        text,
        comments: &tree.comments,
        next_comment: 0,
        out: String::new(),
        column: 0,
        needs_break: false,
        previous_end: 0,
    };
    for (i, node) in tree.nodes.iter().enumerate() {
        writer.comments_before(node.start, 0);
        if i > 0 || writer.column > 0 || !writer.out.is_empty() {
            writer.line_break(node.start, 0);
        }
        writer.node(node);
    }
    writer.comments_before(text.len(), 0);
    let mut out = writer.out;
    if !out.is_empty() {
        out.push('\n');
    }
    Some(out)
}

/// Whether `node` and every list in it is closed.
fn all_closed(node: &Node) -> bool {
    node.closed && node.children().is_none_or(|c| c.iter().all(all_closed))
}

/// Writes the formatted text.
struct Writer<'t> {
    /// The input.
    text: &'t str,
    /// The comments of the input, in order.
    comments: &'t [Range<usize>],
    /// The first comment not written yet.
    next_comment: usize,
    /// The formatted text so far.
    out: String,
    /// The column at the end of `out`.
    column: usize,
    /// Whether a comment ended the current line, so nothing else may follow on it.
    needs_break: bool,
    /// The input offset just past the last thing written, to find blank lines.
    previous_end: usize,
}

impl Writer<'_> {
    /// Writes `s`, which has no line break.
    fn write(&mut self, s: &str) {
        self.out.push_str(s);
        self.column += s.len();
    }

    /// Starts a new line at `indent`, keeping one blank line if the input had
    /// one between the last thing written and `next`.
    fn line_break(&mut self, next: usize, indent: usize) {
        let gap = &self.text[self.previous_end.min(next)..next];
        let blank = gap.matches('\n').count() >= 2;
        while self.out.ends_with(' ') {
            self.out.pop();
        }
        self.out.push('\n');
        if blank {
            self.out.push('\n');
        }
        self.out.extend(std::iter::repeat_n(' ', indent));
        self.column = indent;
        self.needs_break = false;
    }

    /// Writes the comments that start before `offset`. One on the same input line
    /// as what came before it stays at the end of that line; others get their own.
    fn comments_before(&mut self, offset: usize, indent: usize) {
        while let Some(comment) = self.comments.get(self.next_comment) {
            if comment.start >= offset {
                break;
            }
            let gap = &self.text[self.previous_end.min(comment.start)..comment.start];
            let trailing = !self.out.is_empty() && !gap.contains('\n');
            if trailing {
                self.write(" ");
            } else if !self.out.is_empty() {
                self.line_break(comment.start, indent);
            }
            let text = self.text[comment.clone()].trim_end();
            self.write(text);
            self.previous_end = comment.end;
            self.needs_break = true;
            self.next_comment += 1;
        }
    }

    /// Whether a comment starts inside `range`.
    fn has_comment(&self, range: &Range<usize>) -> bool {
        let first = self.comments.partition_point(|c| c.start < range.start);
        self.comments
            .get(first)
            .is_some_and(|c| c.start < range.end)
    }

    /// `node` on one line, or `None` if it has a comment inside.
    fn flat(&self, node: &Node) -> Option<String> {
        match node.children() {
            None => Some(node.atom().unwrap_or_default().to_string()),
            Some(_) if self.has_comment(&node_range(node)) => None,
            Some(children) => {
                let parts: Option<Vec<_>> = children.iter().map(|c| self.flat(c)).collect();
                Some(format!("({})", parts?.join(" ")))
            }
        }
    }

    /// Writes `node` at the current column.
    fn node(&mut self, node: &Node) {
        // A domain or problem always gets a line per section.
        let define = node
            .head()
            .is_some_and(|h| h.eq_ignore_ascii_case("define"));
        if let Some(flat) = self.flat(node).filter(|_| !define) {
            if self.column + flat.len() <= WIDTH || node.atom().is_some() {
                self.write(&flat);
                self.previous_end = node_range(node).end;
                return;
            }
        }
        let children = node.children().unwrap_or_default();
        let open = self.column;
        let inner = open + INDENT;
        self.write("(");
        self.previous_end = node.start + 1;
        let hug = node
            .head()
            .is_some_and(|h| HUGGING.iter().any(|w| h.eq_ignore_ascii_case(w)));
        let mut i = 0;
        while i < children.len() {
            let child = &children[i];
            self.comments_before(child.start, inner);
            let keyword_pair = child.atom().is_some_and(|a| a.starts_with(':'))
                && children
                    .get(i + 1)
                    .is_some_and(|next| !next.atom().is_some_and(|a| a.starts_with(':')));
            let fits = child
                .atom()
                .is_some_and(|a| self.column + 1 + a.len() <= WIDTH);
            let on_head_line = i == 0 || (i == 1 && hug);
            if self.needs_break
                || (i > 0 && !on_head_line && (keyword_pair || child.atom().is_none() || !fits))
            {
                self.line_break(child.start, inner);
            } else if i > 0 {
                self.write(" ");
            }
            self.node(child);
            if keyword_pair && i > 0 {
                let value = &children[i + 1];
                self.comments_before(value.start, inner);
                if self.needs_break {
                    self.line_break(value.start, inner + INDENT);
                } else {
                    self.write(" ");
                }
                self.node(value);
                i += 1;
            }
            i += 1;
        }
        self.comments_before(node.end, inner);
        if self.needs_break {
            self.line_break(node.end, open);
        }
        self.write(")");
        self.previous_end = node.end + 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert2::check;

    #[test]
    fn formats_a_domain() {
        let input = "(define (domain d) ; the domain
(:requirements :strips :typing)


(:predicates (at ?x - obj ?y - loc) ; where
 (clear ?x))
(:action move :parameters (?x - obj ?from ?to - loc) :precondition (and (at ?x ?from) (clear ?to)) :effect (and (at ?x ?to) (not (at ?x ?from)))))";
        let expected = "(define (domain d) ; the domain
  (:requirements :strips :typing)

  (:predicates
    (at ?x - obj ?y - loc) ; where
    (clear ?x))
  (:action move
    :parameters (?x - obj ?from ?to - loc)
    :precondition (and (at ?x ?from) (clear ?to))
    :effect (and (at ?x ?to) (not (at ?x ?from)))))
";
        let formatted = format(input).unwrap();
        check!(formatted == expected);
        check!(format(&formatted).unwrap() == formatted);
    }

    #[test]
    fn keeps_comments_on_their_own_lines() {
        let input = "; header\n\n(define (problem p)\n  ; objects\n  (:objects a b) (:init) ; end\n  )\n; footer\n";
        let formatted = format(input).unwrap();
        check!(
            formatted
                == "; header\n\n(define (problem p)\n  ; objects\n  (:objects a b)\n  (:init) ; end\n)\n; footer\n"
        );
        check!(format(&formatted).unwrap() == formatted);
    }

    #[test]
    fn refuses_unbalanced_input() {
        check!(format("(define (domain d)").is_none());
        check!(format("(a)) (b)").is_none());
        check!(format("").as_deref() == Some(""));
    }
}
