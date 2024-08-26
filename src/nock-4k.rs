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

/// Implements the Nock '=' operator, pronounced 'tis'
///
/// The tis operator compares two Nouns for equality.
/// In Nock, 0 represents true (equal) and 1 represents false (not equal).
///
/// # Arguments
///
/// * `noun1` - A reference to the first Noun to be compared
/// * `noun2` - A reference to the second Noun to be compared
///
/// # Returns
///
/// * `Noun::Atom(0)` if the inputs are equal
/// * `Noun::Atom(1)` if the inputs are not equal
///
/// # Examples
///
/// ```
/// use nock_interpreter::{Noun, tis};
/// let atom1 = Noun::Atom(42);
/// let atom2 = Noun::Atom(42);
/// assert_eq!(tis(&atom1, &atom2), Noun::Atom(0));
///
/// let cell1 = Noun::Cell(Box::new(Noun::Atom(1)), Box::new(Noun::Atom(2)));
/// let cell2 = Noun::Cell(Box::new(Noun::Atom(1)), Box::new(Noun::Atom(3)));
/// assert_eq!(tis(&cell1, &cell2), Noun::Atom(1));
/// ```
pub fn tis(noun1: &Noun, noun2: &Noun) -> Noun {
    if noun1 == noun2 {
        Noun::Atom(0)
    } else {
        Noun::Atom(1)
    }
}

/// Implements the Nock '/' operator, pronounced 'fas'
///
/// The fas operator performs tree addressing in Nock.
/// The root of the tree is 1; the left child of any node n is 2n; the right child is 2n+1.
///
/// # Arguments
///
/// * `address` - A reference to the Noun representing the address
/// * `tree` - A reference to the Noun representing the tree to be accessed
///
/// # Returns
///
/// The Noun at the specified address in the tree
///
/// # Panics
///
/// - When the address is 0
/// - When the address is a Cell
/// - When no valid child is found at the given address
///
/// # Examples
///
/// ```
/// use nock_interpreter::{Noun, fas};
/// let tree = Noun::Cell(Box::new(Noun::Atom(1)), Box::new(Noun::Atom(2)));
/// assert_eq!(fas(&Noun::Atom(2), &tree), Noun::Atom(1));
/// ```
pub fn fas(address: &Noun, tree: &Noun) -> Noun {
    match address {
        Noun::Atom(0) => panic!("fas operation does not support 0 address"),
        Noun::Atom(1) => tree.clone(),
        Noun::Atom(n) if *n == 2 || *n == 3 => match tree {
            Noun::Cell(head, tail) => {
                if *n == 2 {
                    (**head).clone()
                } else {
                    (**tail).clone()
                }
            }
            Noun::Atom(_) => panic!("fas operation found no child at this address"),
        },
        Noun::Atom(n) => match tree {
            Noun::Cell(..) => fas(&Noun::Atom(2 + n % 2), &fas(&Noun::Atom(n / 2), tree)),
            Noun::Atom(_) => panic!("fas operation found no child at this address"),
        },
        Noun::Cell(..) => panic!("fas operation does not support cell address"),
    }
}

// okay its not enough to just pass head and tail, because i need to focus on the node
// if the number is greater than 3, and im doing things recursively, i only care what
// the values are from 4-7 inclusive. that tells me whether head or tail gets passed in
// so if its 4 or 5, i subtract 2 and pass in head
// if its 6 or 7, i subtract 4 and pass in tail
