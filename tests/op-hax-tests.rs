use nock_interpreter::*;

// Test the Nock '#' operator, pronounced 'hax'
#[test]
fn test_hax_replace_root() {
    let original = Noun::Cell(Box::new(Noun::Atom(22)), Box::new(Noun::Atom(33)));
    let replacement = Noun::Atom(11);
    let result = hax(&Noun::Atom(1), &replacement, &original);
    assert_eq!(result, replacement);
}

#[test]
fn test_hax_replace_atom_addr_2() {
    let original = Noun::Cell(Box::new(Noun::Atom(22)), Box::new(Noun::Atom(33)));
    let replacement = Noun::Atom(11);
    let result = hax(&Noun::Atom(2), &replacement, &original);
    let expected = Noun::Cell(Box::new(Noun::Atom(11)), Box::new(Noun::Atom(33)));
    assert_eq!(result, expected);
}

#[test]
fn test_hax_replace_atom_addr_3() {
    let original = Noun::Cell(Box::new(Noun::Atom(22)), Box::new(Noun::Atom(33)));
    let replacement = Noun::Atom(11);
    let result = hax(&Noun::Atom(3), &replacement, &original);
    let expected = Noun::Cell(Box::new(Noun::Atom(22)), Box::new(Noun::Atom(11)));
    assert_eq!(result, expected);
}

#[test]
fn test_hax_replace_cell_addr_3() {
    let original = Noun::Cell(
        Box::new(Noun::Atom(22)),
        Box::new(Noun::Cell(
            Box::new(Noun::Atom(33)),
            Box::new(Noun::Atom(44)),
        )),
    );
    let replacement = Noun::Cell(Box::new(Noun::Atom(55)), Box::new(Noun::Atom(66)));
    let result = hax(&Noun::Atom(3), &replacement, &original);
    let expected = Noun::Cell(Box::new(Noun::Atom(22)), Box::new(replacement.clone()));
    assert_eq!(result, expected);
}

#[test]
fn test_hax_replace_atom_with_cell_addr_3() {
    let original = Noun::Cell(Box::new(Noun::Atom(22)), Box::new(Noun::Atom(33)));
    let replacement = Noun::Cell(Box::new(Noun::Atom(44)), Box::new(Noun::Atom(55)));
    let result = hax(&Noun::Atom(3), &replacement, &original);
    let expected = Noun::Cell(Box::new(Noun::Atom(22)), Box::new(replacement.clone()));
    assert_eq!(result, expected);
}

fn test_hax_replace_cell_with_atom_addr_3() {
    let original = Noun::Cell(
        Box::new(Noun::Atom(22)),
        Box::new(Noun::Cell(
            Box::new(Noun::Atom(33)),
            Box::new(Noun::Atom(44)),
        )),
    );
    let replacement = Noun::Atom(11);
    let result = hax(&Noun::Atom(3), &replacement, &original);
    let expected = Noun::Cell(Box::new(Noun::Atom(22)), Box::new(replacement.clone()));
    assert_eq!(result, expected);
}

#[test]
fn test_hax_replace_atom_addr_4() {
    let original = Noun::Cell(
        Box::new(Noun::Cell(
            Box::new(Noun::Atom(22)),
            Box::new(Noun::Atom(33)),
        )),
        Box::new(Noun::Atom(44)),
    );
    let replacement = Noun::Atom(11);
    let result = hax(&Noun::Atom(4), &replacement, &original);
    let expected = Noun::Cell(
        Box::new(Noun::Cell(
            Box::new(Noun::Atom(11)),
            Box::new(Noun::Atom(33)),
        )),
        Box::new(Noun::Atom(44)),
    );
    assert_eq!(result, expected);
}

#[test]
fn test_hax_replace_atom_addr_5() {
    let original = Noun::Cell(
        Box::new(Noun::Cell(
            Box::new(Noun::Atom(22)),
            Box::new(Noun::Atom(33)),
        )),
        Box::new(Noun::Atom(44)),
    );
    let replacement = Noun::Atom(11);
    let result = hax(&Noun::Atom(5), &replacement, &original);
    let expected = Noun::Cell(
        Box::new(Noun::Cell(
            Box::new(Noun::Atom(22)),
            Box::new(Noun::Atom(11)),
        )),
        Box::new(Noun::Atom(44)),
    );
    assert_eq!(result, expected);
}
