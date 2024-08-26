use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Noun {
    Atom(u64),
    Cell(Box<Noun>, Box<Noun>),
}

impl fmt::Display for Noun {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Noun::Atom(value) => write!(f, "{}", value),
            Noun::Cell(head, tail) => write!(f, "[{} {}]", head, tail),
        }
    }
}