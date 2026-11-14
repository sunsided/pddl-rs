//! Provides parsers for variables (`<variable>`).

use crate::parsers::error::R;
use crate::parsers::is_name;
use crate::parsers::sexpr::Node;
use crate::types::{Name, Variable};

/// Parses a variable (`<variable>`), i.e. `?<name>`.
pub(crate) fn parse_variable(node: &Node) -> R<Variable> {
    match node.atom().and_then(|text| text.strip_prefix('?')) {
        Some(name) if is_name(name) => Ok(Variable::from(Name::from(name))),
        _ => Err(node.error()),
    }
}

crate::parsers::impl_parser!(Variable => parse_variable);

#[cfg(test)]
mod tests {
    use crate::parsers::UnwrapValue;
    use crate::{Parser, Variable};
    use assert2::check;

    #[test]
    fn test_parse() {
        check!(Variable::from_str("?abcde").is_value("abcde".into()));
        check!(Variable::from_str("?a-1_2").is_value("a-1_2".into()));
        check!(Variable::from_str("?Z01").is_value("Z01".into()));
        check!(Variable::from_str("?x-_-_").is_value("x-_-_".into()));
        check!(Variable::from_str("abcde").is_err());
        check!(Variable::from_str("?-").is_err());
        check!(Variable::from_str("?1").is_err());
        check!(Variable::from_str("?").is_err());
        check!(Variable::from_str("? x").is_err());
    }
}
