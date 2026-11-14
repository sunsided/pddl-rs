//! Provides parsers for variables.

use crate::parsers::error::R;
use crate::parsers::is_name;
use crate::parsers::sexpr::Node;
use crate::types::{Name, Variable};

/// Parses a variable, i.e. `?<name>`.
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

    #[test]
    fn test_parse() {
        assert!(Variable::from_str("?abcde").is_value("abcde".into()));
        assert!(Variable::from_str("?a-1_2").is_value("a-1_2".into()));
        assert!(Variable::from_str("?Z01").is_value("Z01".into()));
        assert!(Variable::from_str("?x-_-_").is_value("x-_-_".into()));
        assert!(Variable::from_str("abcde").is_err());
        assert!(Variable::from_str("?-").is_err());
        assert!(Variable::from_str("?1").is_err());
        assert!(Variable::from_str("?").is_err());
        assert!(Variable::from_str("? x").is_err());
    }
}
