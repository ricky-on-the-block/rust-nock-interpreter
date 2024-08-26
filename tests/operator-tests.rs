use nock_interpreter::*;

#[test]
fn test_wut_on_atom() {
    let atom = Noun::Atom(42);
    assert_eq!(wut(&atom), Noun::Atom(1));

    let zeroAtom = Noun::Atom(0);
    assert_eq!(wut(&zeroAtom), Noun::Atom(1));
}

#[test]
fn test_wut_on_cell() {
    let cell = Noun::Cell(Box::new(Noun::Atom(1)), Box::new(Noun::Atom(2)));
    assert_eq!(wut(&cell), Noun::Atom(0));
}

#[test]
fn test_wut_on_nested_cell() {
    let nested_cell = Noun::Cell(
        Box::new(Noun::Atom(1)),
        Box::new(Noun::Cell(Box::new(Noun::Atom(2)), Box::new(Noun::Atom(3)))),
    );
    assert_eq!(wut(&nested_cell), Noun::Atom(0));
}
