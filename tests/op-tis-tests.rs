use nock_interpreter::*;

// Test the Nock '=' operator, pronounced 'tis'
#[test]
fn test_tis_equal_atoms() {
    let atom1 = Noun::Atom(42);
    let atom2 = Noun::Atom(42);
    assert_eq!(tis(&atom1, &atom2), Noun::Atom(0));
}

#[test]
fn test_tis_unequal_atoms() {
    let atom1 = Noun::Atom(42);
    let atom2 = Noun::Atom(43);
    assert_eq!(tis(&atom1, &atom2), Noun::Atom(1));
}

#[test]
fn test_tis_equal_cells() {
    let cell1 = Noun::Cell(Box::new(Noun::Atom(1)), Box::new(Noun::Atom(2)));
    let cell2 = Noun::Cell(Box::new(Noun::Atom(1)), Box::new(Noun::Atom(2)));
    assert_eq!(tis(&cell1, &cell2), Noun::Atom(0));
}

#[test]
fn test_tis_unequal_cells() {
    let cell1 = Noun::Cell(Box::new(Noun::Atom(1)), Box::new(Noun::Atom(2)));
    let cell2 = Noun::Cell(Box::new(Noun::Atom(1)), Box::new(Noun::Atom(3)));
    assert_eq!(tis(&cell1, &cell2), Noun::Atom(1));
}

#[test]
fn test_tis_unequal_atom_and_cell() {
    let atom = Noun::Atom(42);
    let cell = Noun::Cell(Box::new(Noun::Atom(42)), Box::new(Noun::Atom(42)));
    assert_eq!(tis(&atom, &cell), Noun::Atom(1));
}

#[test]
fn test_tis_equal_nested_cells() {
    let cell1 = Noun::Cell(
        Box::new(Noun::Atom(1)),
        Box::new(Noun::Cell(Box::new(Noun::Atom(2)), Box::new(Noun::Atom(3)))),
    );
    let cell2 = Noun::Cell(
        Box::new(Noun::Atom(1)),
        Box::new(Noun::Cell(Box::new(Noun::Atom(2)), Box::new(Noun::Atom(3)))),
    );
    assert_eq!(tis(&cell1, &cell2), Noun::Atom(0));
}
