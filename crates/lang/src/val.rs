use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub enum Val {
    Bool(bool),
    Number(i32),
    Unit,
}

impl fmt::Display for Val {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Bool(b) => write!(f, "{}", b),
            Self::Number(n) => write!(f, "{}", n),
            Self::Unit => write!(f, "Unit"),
        }
    }
}
