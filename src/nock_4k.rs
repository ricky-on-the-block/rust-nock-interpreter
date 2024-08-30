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
            Noun::Cell(op, tail) => write!(f, "[{} {}]", op, tail),
        }
    }
}

impl Noun {
    // Helper function to create an Atom
    pub fn atom(value: u64) -> Self {
        Noun::Atom(value)
    }

    // Helper function to create a Cell
    pub fn cell(left: Noun, right: Noun) -> Self {
        Noun::Cell(Box::new(left), Box::new(right))
    }
}

#[macro_export]
macro_rules! noun {
    // Rule 1: Match a single literal (base case for atoms)
    [$num:literal] => {
        Noun::Atom($num)
    };

    // Rule 2: Match two literals (simple cell)
    [$num1:literal $num2:literal] => {
        Noun::cell(Noun::Atom($num1), Noun::Atom($num2))
    };

    // Rule 3: Match two nested structures
    [[$($left:tt)+] [$($right:tt)+]] => {
        Noun::cell(noun![$($left)+], noun![$($right)+])
    };

    // Rule 4: Match a nested structure on the left and a single token on the right
    [[$($left:tt)+] $right:tt] => {
        Noun::cell(noun![$($left)+], noun![$right])
    };

    // Rule 5: Match a single token on the left and a nested structure on the right
    [$left:tt [$($right:tt)+]] => {
        Noun::cell(noun![$left], noun![$($right)+])
    };
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
/// Performs tree addressing in Nock.
/// The root of the tree is 1; the left child of any node n is 2n; the right child is 2n+1.
///
/// # Arguments
///
/// * `address` - A reference to the Noun representing the address
/// * `tree` - A mutable reference to the Noun representing the tree to be accessed
///
/// # Returns
///
/// A mutable reference to the Noun at the specified address in the tree
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
/// let mut tree = Noun::Cell(Box::new(Noun::Atom(1)), Box::new(Noun::Atom(2)));
/// assert_eq!(*fas(&Noun::Atom(2), &mut tree), Noun::Atom(1));
/// ```
pub fn fas<'a>(address: &Noun, tree: &'a mut Noun) -> &'a mut Noun {
    match address {
        Noun::Atom(0) => panic!("fas operation does not support 0 address"),
        Noun::Atom(1) => tree,
        Noun::Atom(n) if *n == 2 || *n == 3 => match tree {
            Noun::Cell(op, tail) => {
                if *n == 2 {
                    op
                } else {
                    tail
                }
            }
            Noun::Atom(_) => panic!("fas operation found no child at this address"),
        },
        Noun::Atom(n) => match tree {
            Noun::Cell(..) => fas(&Noun::Atom(2 + n % 2), fas(&Noun::Atom(n / 2), tree)),
            Noun::Atom(_) => panic!("fas operation found no child at this address"),
        },
        Noun::Cell(..) => panic!("fas operation does not support cell address"),
    }
}

/// Implements the Nock '#' operator, pronounced 'hax'
///
/// Replaces a part of a Noun tree at a specified address with a replacement Noun.
/// Modifies the original tree in place and returns a clone of the modified tree.
///
/// # Arguments
///
/// * `address` - A reference to a Noun representing the address in the tree where the replacement should occur
/// * `replacement` - A reference to the Noun that will replace the existing Noun at the specified address
/// * `tree` - A mutable reference to the Noun tree that will be modified
///
/// # Returns
///
/// A clone of the modified Noun tree
///
/// # Panics
///
/// This function will panic in the same cases as the `fas` function:
/// - When the address is 0
/// - When the address is a Cell
/// - When no valid child is found at the given address
///
/// # Examples
///
/// ```
/// use nock_interpreter::{Noun, hax};
/// let mut tree = Noun::Cell(Box::new(Noun::Atom(1)), Box::new(Noun::Atom(2)));
/// let result = hax(&Noun::Atom(2), &Noun::Atom(3), &mut tree);
/// assert_eq!(result, Noun::Cell(Box::new(Noun::Atom(3)), Box::new(Noun::Atom(2))));
/// ```
///
/// # Note
///
/// This function modifies the original tree and also returns a clone of the modified tree.
pub fn hax(address: &Noun, replacement: &Noun, tree: &mut Noun) -> Noun {
    let tree_at_addr = fas(address, tree);
    *tree_at_addr = replacement.clone();
    tree.clone()
}

pub fn tar(noun: &mut Noun) -> Noun {
    println!("Evaluating tar with:");
    println!(
        "  Subject: {}",
        match noun {
            Noun::Cell(subject, formula) => format!("[subject:{}, formula:{}]", subject, formula),
            _ => "Invalid".to_string(),
        }
    );

    match noun {
        Noun::Atom(_) => panic!("tar cannot be performed on an atom"),
        Noun::Cell(subject, formula) => match &**formula {
            Noun::Cell(op, tail) => match (&**op, &**tail) {
                // Distribution (cons)
                (distribute_cell @ Noun::Cell(..), d) => {
                    Noun::cell(
                        tar(&mut Noun::Cell(subject.clone(), Box::new(distribute_cell.clone()))),
                        tar(&mut Noun::Cell(subject.clone(), Box::new(d.clone())))
                    )
                },
                // Instructions
                (Noun::Atom(0), _) => fas(tail, subject).clone(),
                (Noun::Atom(1), _) => *tail.clone(),
                (Noun::Atom(3), _) => wut(&tar(&mut Noun::Cell(subject.clone(), tail.clone()))),
                (Noun::Atom(4), _) => lus(&tar(&mut Noun::Cell(subject.clone(), tail.clone()))),
                // Operations that expect a cell as their tail
                (Noun::Atom(2), Noun::Cell(b, c)) => tar(&mut Noun::cell(
                    tar(&mut Noun::Cell(subject.clone(), b.clone())),
                    tar(&mut Noun::Cell(subject.clone(), c.clone())))
                ),
                (Noun::Atom(5), Noun::Cell(b, c)) => tis(
                    &tar(&mut Noun::Cell(subject.clone(), b.clone())),
                    &tar(&mut Noun::Cell(subject.clone(), c.clone())),
                ),
                // *[a 6 b c d]    *[a *[[c d] 0 *[[2 3] 0 *[a 4 4 b]]]]
                (Noun::Atom(6), Noun::Cell(b, six_tail)) => match &**six_tail {
                    Noun::Cell(c, d) =>
                    tar(&mut Noun::cell(*subject.clone(),
                        tar(&mut Noun::cell(
                        Noun::cell(*c.clone(), *d.clone()),
                        Noun::cell(
                            Noun::Atom(0),
                            tar(&mut Noun::cell(
                                Noun::cell(Noun::Atom(2), Noun::Atom(3)),
                                Noun::cell(
                                    Noun::atom(0),
                                    tar(&mut Noun::cell(
                                        *subject.clone(),
                                        Noun::cell(
                                            Noun::Atom(4),
                                            Noun::cell(Noun::Atom(4), *b.clone())
                                        )
                                    ))
                                )
                            )))
                        )))),
                    _ => panic!("Invalid structure for instruction 6")
                },
                (Noun::Atom(7), Noun::Cell(b, c)) => tar(&mut Noun::cell(
                    tar(&mut Noun::Cell(subject.clone(), b.clone())),
                    *c.clone()
                )),
                // *[a 8 b c]    *[[*[a b] a] c]
                (Noun::Atom(8), Noun::Cell(b, c)) => tar(&mut Noun::cell(
                    Noun::cell(
                        tar(&mut Noun::Cell(subject.clone(), b.clone())),
                        *subject.clone()
                    ),
                    *c.clone()
                )),
                // *[a 9 b c]    *[*[a c] 2 [0 1] 0 b]
                (Noun::Atom(9), Noun::Cell(b, c)) => tar(&mut Noun::cell(
                    tar(&mut Noun::Cell(subject.clone(), c.clone())),
                    Noun::cell(
                        Noun::Atom(2),
                        Noun::cell(
                            Noun::cell(Noun::Atom(0), Noun::Atom(1)),
                            Noun::cell(Noun::Atom(0), *b.clone())
                        )
                    )
                )),
                (Noun::Atom(10), Noun::Cell(_b, _c)) => todo!("Implement instruction 10"),
                (Noun::Atom(11), Noun::Cell(_b, _c)) => todo!("Implement instruction 11"),
                // Catch case for operations 2 and 5-11 when the tail is not a cell
                (Noun::Atom(2 | 5..=11), _) => {
                    panic!("Operation {} expects a cell as its argument", op)
                }
                // Add more operations here as needed
                _ => panic!("Unimplemented operation or invalid argument structure\nsubject: {}\nformula: {}", subject, formula),
            },
            _ => panic!("Formula must be a cell\nformula: {}", formula),
        },
    }
}
