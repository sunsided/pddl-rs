use std::fmt::{Display, Formatter};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum MultiOp {
    Multiplication,
    Addition,
}

impl Display for MultiOp {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            MultiOp::Multiplication => write!(f, "*"),
            MultiOp::Addition => write!(f, "+"),
        }
    }
}

impl TryFrom<&str> for MultiOp {
    type Error = ParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "*" => Ok(Self::Multiplication),
            "+" => Ok(Self::Addition),
            _ => Err(ParseError::InvalidOperation),
        }
    }
}

#[derive(Debug, Clone, thiserror::Error)]
pub enum ParseError {
    #[error("Invalid operation")]
    InvalidOperation,
}
