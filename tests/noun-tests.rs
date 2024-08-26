use nock_interpreter::*;

#[test]
fn test_partial_eq_atoms() {
    assert_eq!(Noun::Atom(42), Noun::Atom(42));
    assert_ne!(Noun::Atom(42), Noun::Atom(43));
}

#[test]
fn test_partial_eq_cells() {
    let cell1 = Noun::Cell(Box::new(Noun::Atom(1)), Box::new(Noun::Atom(2)));
    let cell2 = Noun::Cell(Box::new(Noun::Atom(1)), Box::new(Noun::Atom(2)));
    let cell3 = Noun::Cell(Box::new(Noun::Atom(1)), Box::new(Noun::Atom(3)));

    assert_eq!(cell1, cell2);
    assert_ne!(cell1, cell3);
}

#[test]
fn test_partial_eq_nested() {
    let nested1 = Noun::Cell(
        Box::new(Noun::Atom(1)),
        Box::new(Noun::Cell(Box::new(Noun::Atom(2)), Box::new(Noun::Atom(3)))),
    );
    let nested2 = Noun::Cell(
        Box::new(Noun::Atom(1)),
        Box::new(Noun::Cell(Box::new(Noun::Atom(2)), Box::new(Noun::Atom(3)))),
    );
    let nested3 = Noun::Cell(
        Box::new(Noun::Atom(1)),
        Box::new(Noun::Cell(Box::new(Noun::Atom(2)), Box::new(Noun::Atom(4)))),
    );

    assert_eq!(nested1, nested2);
    assert_ne!(nested1, nested3);
}

#[test]
fn test_clone_atom() {
    let atom = Noun::Atom(42);
    let cloned_atom = atom.clone();
    assert_eq!(atom, cloned_atom);

    // Ensure it's a deep copy
    match cloned_atom {
        Noun::Atom(value) => assert_eq!(value, 42),
        _ => panic!("Expected Atom"),
    }
}

#[test]
fn test_clone_cell() {
    let cell = Noun::Cell(Box::new(Noun::Atom(1)), Box::new(Noun::Atom(2)));
    let cloned_cell = cell.clone();
    assert_eq!(cell, cloned_cell);

    // Ensure it's a deep copy
    match cloned_cell {
        Noun::Cell(h, t) => {
            assert_eq!(*h, Noun::Atom(1));
            assert_eq!(*t, Noun::Atom(2));
        }
        _ => panic!("Expected Cell"),
    }
}

#[test]
fn test_clone_nested() {
    let nested = Noun::Cell(
        Box::new(Noun::Atom(1)),
        Box::new(Noun::Cell(Box::new(Noun::Atom(2)), Box::new(Noun::Atom(3)))),
    );
    let cloned_nested = nested.clone();
    assert_eq!(nested, cloned_nested);

    // Ensure it's a deep copy
    match cloned_nested {
        Noun::Cell(h, t) => {
            assert_eq!(*h, Noun::Atom(1));
            match *t {
                Noun::Cell(inner_h, inner_t) => {
                    assert_eq!(*inner_h, Noun::Atom(2));
                    assert_eq!(*inner_t, Noun::Atom(3));
                }
                _ => panic!("Expected nested Cell"),
            }
        }
        _ => panic!("Expected Cell"),
    }
}

#[test]
fn test_display_atom() {
    let atom = Noun::Atom(42);
    assert_eq!(format!("{}", atom), "42");
}

#[test]
fn test_display_simple_cell() {
    let cell = Noun::Cell(Box::new(Noun::Atom(1)), Box::new(Noun::Atom(2)));
    assert_eq!(format!("{}", cell), "[1 2]");
}

#[test]
fn test_display_nested_cell() {
    let nested = Noun::Cell(
        Box::new(Noun::Atom(1)),
        Box::new(Noun::Cell(Box::new(Noun::Atom(2)), Box::new(Noun::Atom(3)))),
    );
    assert_eq!(format!("{}", nested), "[1 [2 3]]");
}

#[test]
fn test_display_complex_nested_cell() {
    let complex = Noun::Cell(
        Box::new(Noun::Cell(Box::new(Noun::Atom(1)), Box::new(Noun::Atom(2)))),
        Box::new(Noun::Cell(Box::new(Noun::Atom(3)), Box::new(Noun::Atom(4)))),
    );
    assert_eq!(format!("{}", complex), "[[1 2] [3 4]]");
}
