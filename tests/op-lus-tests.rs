use nock_interpreter::*;

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
#[should_panic(expected = "lus operation is not defined for cells, only atoms")]
fn test_lus_on_cell() {
    let cell = Noun::Cell(Box::new(Noun::Atom(1)), Box::new(Noun::Atom(2)));
    lus(&cell);
}
