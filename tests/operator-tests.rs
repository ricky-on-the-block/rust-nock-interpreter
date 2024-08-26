use nock_interpreter::*;

// Test the Nock '?' operator, pronounced 'wut'
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

// Test the Nock '+' operator, pronounced 'lus'
#[test]
fn test_lus_on_zero() {
    let zero = Noun::Atom(0);
    assert_eq!(lus(&zero), Noun::Atom(1));
}

#[test]
fn test_lus_on_positive_integer() {
    let num = Noun::Atom(42);
    assert_eq!(lus(&num), Noun::Atom(43));
}

// TODO: Determine what the desired behavior actually is on uint boundaries
#[test]
fn test_lus_on_max_u64() {
    let max = Noun::Atom(u64::MAX);
    assert_eq!(lus(&max), Noun::Atom(0)); // Expect wrapping behavior
}

#[test]
#[should_panic()]
fn test_lus_on_cell() {
    let cell = Noun::Cell(Box::new(Noun::Atom(1)), Box::new(Noun::Atom(2)));
    lus(&cell);
}
