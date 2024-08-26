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

/// Implements the Nock '?' operator, pronounced 'wut'
///
/// The wut operator tests whether a noun is a cell or an atom.
/// In Nock, 0 represents true and 1 represents false.
///
/// # Arguments
///
/// * `noun` - A reference to the Noun to be tested
///
/// # Returns
///
/// * `Noun::Atom(0)` if the input is a cell (true in Nock)
/// * `Noun::Atom(1)` if the input is an atom (false in Nock)
///
/// # Examples
///
/// ```
/// use nock_interpreter::{Noun, wut};
/// let atom = Noun::Atom(42);
/// assert_eq!(wut(&atom), Noun::Atom(1));
///
/// let cell = Noun::Cell(Box::new(Noun::Atom(1)), Box::new(Noun::Atom(2)));
/// assert_eq!(wut(&cell), Noun::Atom(0));
/// ```
pub fn wut(noun: &Noun) -> Noun {
    match noun {
        Noun::Atom(_) => Noun::Atom(1),
        Noun::Cell(_, _) => Noun::Atom(0),
    }
}

/// Implements the Nock '+' operator, pronounced 'lus'
///
/// The lus operator increments an atom by 1. It wraps around to 0 if the result exceeds u64::MAX.
/// This operation is only defined for atoms and will panic if given a cell.
///
/// # Arguments
///
/// * `noun` - A reference to the Noun to be incremented
///
/// # Returns
///
/// * `Noun::Atom` containing the incremented value
///
/// # Panics
///
/// Panics if the input is a Cell, as the operation is undefined for cells.
///
/// # Examples
///
/// ```
/// use nock_interpreter::{Noun, lus};
/// let atom = Noun::Atom(42);
/// assert_eq!(lus(&atom), Noun::Atom(43));
///
/// let max = Noun::Atom(u64::MAX);
/// assert_eq!(lus(&max), Noun::Atom(0));  // Wraps around
/// ```
pub fn lus(noun: &Noun) -> Noun {
    match noun {
        Noun::Atom(n) => Noun::Atom(n.wrapping_add(1)),
        Noun::Cell(_, _) => panic!("lus operation is not defined for cells, only atoms"),
    }
}
