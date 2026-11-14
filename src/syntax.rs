//! The untyped syntax tree of PDDL text: atoms, parenthesized lists and
//! comments, each with its byte range in the input.
//!
//! The [`Parser`](crate::Parser) reads every input into this tree before it
//! builds the typed AST. The tree keeps what the AST doesn't: where each name
//! was written, and the comments. It suits tools that work on the text, such as
//! editors, formatters and linters.
//!
//! ## Example
//! ```
//! use pddl::syntax::{self, Kind};
//!
//! let input = "(define (domain d)) ; done";
//! let tree = syntax::read(input).unwrap();
//! let define = &tree.nodes[0];
//! assert_eq!(define.head(), Some("define"));
//! let domain = &define.children().unwrap()[1];
//! assert_eq!(&input[domain.start..=domain.end], "(domain d)");
//! assert_eq!(&input[tree.comments[0].clone()], "; done");
//! ```

use std::ops::Range;

use crate::parsers::sexpr;
pub use crate::parsers::sexpr::{Kind, Node};
use crate::ParseError;

/// The top-level nodes of an input and its comments.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SyntaxTree<'a> {
    /// The nodes outside any parentheses, in order.
    ///
    /// A list that is never closed ends at the end of the input, with
    /// [`Node::closed`] set to `false`. Reading stops at a `)` that closes
    /// nothing, so the nodes after it are missing.
    pub nodes: Vec<Node<'a>>,
    /// The byte ranges of the `;` comments, in order, without their line break.
    pub comments: Vec<Range<usize>>,
}

/// Reads `input` into a [`SyntaxTree`].
///
/// Unbalanced parentheses are no error here; [`parsers::check`](crate::parsers::check)
/// reports them.
///
/// ## Errors
///
/// Fails only if lists are nested deeper than the parser accepts.
pub fn read(input: &str) -> Result<SyntaxTree<'_>, ParseError> {
    let forest = sexpr::read(input).map_err(|e| ParseError::new(input, &e))?;
    Ok(SyntaxTree {
        nodes: forest.nodes,
        comments: forest.comments,
    })
}
