//! Building blocks the grammar rules share.

use crate::parsers::error::{Error, R};
use crate::parsers::sexpr::Node;

/// The children of a list, read one at a time.
pub(crate) struct Args<'n, 'a> {
    /// The list being read, for errors at its `)`.
    list: &'n Node<'a>,
    /// The children not read yet.
    items: &'n [Node<'a>],
}

impl<'n, 'a> Args<'n, 'a> {
    /// Reads `items`, the children of `list` (possibly without its head).
    fn new(list: &'n Node<'a>, items: &'n [Node<'a>]) -> Self {
        Self { list, items }
    }

    /// The next child, or an error at the list's `)` if there is none.
    pub fn next(&mut self) -> R<&'n Node<'a>> {
        let (first, rest) = self
            .items
            .split_first()
            .ok_or_else(|| self.list.error_at_close())?;
        self.items = rest;
        Ok(first)
    }

    /// The next child, without taking it.
    pub fn peek(&self) -> Option<&'n Node<'a>> {
        self.items.first()
    }

    /// Takes the atom `word`, or fails at whatever is there instead,
    /// suggesting `word` if that looks like a misspelling of it.
    pub fn expect(&mut self, word: &'static str) -> R<()> {
        let node = self.next()?;
        if node.is(word) {
            return Ok(());
        }
        let error = node.error();
        Err(match node.atom() {
            Some(atom) => error.suggest(atom, &[word]),
            None => error,
        })
    }

    /// Fails at a leftover `:keyword` that looks like a misspelling of one of
    /// `keywords`, with the same error as an unread child but a suggestion.
    pub fn reject_misspelled(&self, keywords: &[&'static str]) -> R<()> {
        let Some(node) = self.peek() else {
            return Ok(());
        };
        match node.atom() {
            Some(atom) if atom.starts_with(':') => {
                let error = node.error().reason("expected `)`").suggest(atom, keywords);
                if error.suggestion.is_some() {
                    Err(error)
                } else {
                    Ok(())
                }
            }
            _ => Ok(()),
        }
    }

    /// Takes all remaining children.
    pub fn rest(&mut self) -> &'n [Node<'a>] {
        std::mem::take(&mut self.items)
    }

    /// Parses each remaining child with `rule`.
    pub fn list0<T>(&mut self, rule: impl Fn(&'n Node<'a>) -> R<T>) -> R<Vec<T>> {
        self.rest().iter().map(rule).collect()
    }

    /// Like [`list0`](Self::list0), but at least one child must remain.
    pub fn list1<T>(&mut self, rule: impl Fn(&'n Node<'a>) -> R<T>) -> R<Vec<T>> {
        if self.items.is_empty() {
            return Err(self.list.error_at_close());
        }
        self.list0(rule)
    }

    /// An error at the list's `)`, for a form that ends too early.
    pub fn error_at_close(&self) -> Error {
        self.list.error_at_close()
    }

    /// Fails at the first child nothing has read.
    fn finish(self) -> R<()> {
        match self.items.first() {
            None => Ok(()),
            Some(extra) => Err(extra.error().reason("expected `)`")),
        }
    }
}

/// Parses a list with `body`, which must read all of its children.
pub(crate) fn parens<'n, 'a, T>(
    node: &'n Node<'a>,
    body: impl FnOnce(&mut Args<'n, 'a>) -> R<T>,
) -> R<T> {
    let items = node.children().ok_or_else(|| node.error())?;
    let mut args = Args::new(node, items);
    let value = body(&mut args)?;
    args.finish()?;
    Ok(value)
}

/// Parses a list `(head ...)` with `body`, which must read all children after `head`.
///
/// A `:keyword` such as `:action` can't start anything else, so once it matched,
/// a failure inside the form is fatal and names the keyword, instead of letting
/// the grammar try other alternatives. Other heads such as `at` or `=` stay open
/// to alternatives: `(at a b)` is also an ordinary atomic formula.
pub(crate) fn prefix<'n, 'a, T>(
    node: &'n Node<'a>,
    head: &'static str,
    body: impl FnOnce(&mut Args<'n, 'a>) -> R<T>,
) -> R<T> {
    let items = match node.children() {
        Some([first, rest @ ..]) if first.is(head) => rest,
        _ => return Err(node.error()),
    };
    let mut args = Args::new(node, items);
    let result = body(&mut args).and_then(|value| args.finish().map(|()| value));
    if head.starts_with(':') {
        result.map_err(|mut e| {
            e.keyword.get_or_insert(head.into());
            e.fatal()
        })
    } else {
        result
    }
}

/// Parses `<keyword> <inner>` inside a form, e.g. `:precondition (and ...)`.
///
/// Like [`prefix`] for `:keywords`, it commits once the keyword matched, so a
/// mistake in `inner` is fatal and tagged with the keyword.
pub(crate) fn keyword_arg<'n, 'a, T>(
    args: &mut Args<'n, 'a>,
    keyword: &'static str,
    inner: impl FnOnce(&'n Node<'a>) -> R<T>,
) -> R<T> {
    args.expect(keyword)?;
    args.next().and_then(inner).map_err(|mut e| {
        e.keyword.get_or_insert(keyword.into());
        e.fatal()
    })
}

/// Like [`keyword_arg`], but `None` if the next child isn't `keyword`.
pub(crate) fn opt_keyword_arg<'n, 'a, T>(
    args: &mut Args<'n, 'a>,
    keyword: &'static str,
    inner: impl FnOnce(&'n Node<'a>) -> R<T>,
) -> R<Option<T>> {
    if args.peek().is_some_and(|n| n.is(keyword)) {
        keyword_arg(args, keyword, inner).map(Some)
    } else {
        Ok(None)
    }
}

/// Tries each rule in order and returns the first success.
///
/// A fatal error ends the search. If every rule fails, returns the error that
/// got furthest, see [`Error::or`].
pub(crate) fn alt<'n, 'a, T>(node: &'n Node<'a>, rules: &[&dyn Fn(&'n Node<'a>) -> R<T>]) -> R<T> {
    let mut error: Option<Error> = None;
    for rule in rules {
        match rule(node) {
            Ok(value) => return Ok(value),
            Err(e) if e.fatal => return Err(e),
            Err(e) => error = Some(error.map_or(e.clone(), |prev| prev.or(e))),
        }
    }
    Err(error.unwrap_or_else(|| node.error()))
}

/// `None` for an empty list `()`, otherwise `rule` applied to `node`.
pub(crate) fn empty_or<'n, 'a, T>(
    node: &'n Node<'a>,
    rule: impl FnOnce(&'n Node<'a>) -> R<T>,
) -> R<Option<T>> {
    match node.children() {
        Some([]) => Ok(None),
        _ => rule(node).map(Some),
    }
}

/// Stores a section's `value`, failing at `section` if that section was already given.
pub(crate) fn set_once<T>(slot: &mut Option<T>, value: T, section: &Node) -> R<()> {
    if slot.is_some() {
        return Err(section.error().reason("duplicate section").fatal());
    }
    *slot = Some(value);
    Ok(())
}

/// Maps the atom `node` through `words`, ignoring ASCII case.
pub(crate) fn word<T: Clone>(node: &Node, words: &[(&'static str, T)]) -> R<T> {
    if let Some((_, value)) = words.iter().find(|(w, _)| node.is(w)) {
        return Ok(value.clone());
    }
    let error = node.error();
    Err(match node.atom() {
        Some(atom) => {
            let candidates: Vec<&'static str> = words.iter().map(|(w, _)| *w).collect();
            error.suggest(atom, &candidates)
        }
        None => error,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parsers::sexpr::read;
    use assert2::check;

    #[test]
    fn prefix_commits_on_keywords_only() {
        let forest = read("(:k a b) (k a b)").unwrap();
        let one = |args: &mut Args| args.next().map(|_| ());
        let error = prefix(&forest.nodes[0], ":k", one).unwrap_err();
        check!(error.fatal);
        check!(error.keyword.as_deref() == Some(":k"));
        check!(error.reason == Some("expected `)`"));
        check!(error.offset == 6);
        let error = prefix(&forest.nodes[1], "k", one).unwrap_err();
        check!(!error.fatal);
        check!(error.keyword == None);
    }

    #[test]
    fn alt_returns_the_furthest_error() {
        let forest = read("(a b c)").unwrap();
        let node = &forest.nodes[0];
        let near = |_: &Node| -> R<()> { Err(Error::at(1, 1)) };
        let far = |_: &Node| -> R<()> { Err(Error::at(5, 1)) };
        check!(alt(node, &[&far, &near]).unwrap_err().offset == 5);
        check!(alt(node, &[&near, &far]).unwrap_err().offset == 5);
        let fatal = |_: &Node| -> R<()> { Err(Error::at(0, 1).fatal()) };
        let ok = |_: &Node| -> R<()> { Ok(()) };
        check!(alt(node, &[&fatal, &ok]).is_err());
    }

    #[test]
    fn missing_arguments_are_reported_at_the_close() {
        let forest = read("(k)").unwrap();
        let error = prefix(&forest.nodes[0], "k", |args| args.next().map(|_| ())).unwrap_err();
        check!((error.offset, error.len) == (2, 1));
    }
}
