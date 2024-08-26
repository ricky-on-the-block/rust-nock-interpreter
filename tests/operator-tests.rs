use nock_interpreter::*;

// Test the Nock '?' operator, pronounced 'wut'
#[test]
fn test_wut_on_atom() {
    let atom = Noun::Atom(42);
    assert_eq!(wut(&atom), Noun::Atom(1));

    let zero_atom = Noun::Atom(0);
    assert_eq!(wut(&zero_atom), Noun::Atom(1));
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
#[should_panic(expected = "lus operation is not defined for cells, only atoms")]
fn test_lus_on_cell() {
    let cell = Noun::Cell(Box::new(Noun::Atom(1)), Box::new(Noun::Atom(2)));
    lus(&cell);
}

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

// Test the Nock '/' operator, pronounced 'fas'
fn create_test_noun() -> Noun {
    Noun::Cell(
        Box::new(Noun::Atom(531)),
        Box::new(Noun::Cell(
            Box::new(Noun::Atom(25)),
            Box::new(Noun::Atom(99)),
        )),
    )
}

#[test]
fn test_fas_root() {
    let noun = create_test_noun();
    assert_eq!(fas(&Noun::Atom(1), &noun), noun);
}

#[test]
fn test_fas_left_child_node_1() {
    let noun = create_test_noun();
    assert_eq!(fas(&Noun::Atom(2), &noun), Noun::Atom(531));
}

#[test]
fn test_fas_right_child_node_1() {
    let noun = create_test_noun();
    let expected = Noun::Cell(Box::new(Noun::Atom(25)), Box::new(Noun::Atom(99)));
    assert_eq!(fas(&Noun::Atom(3), &noun), expected);
}

#[test]
#[should_panic(expected = "fas operation found no child at this address")]
fn test_fas_left_child_node_2() {
    let noun = create_test_noun();
    fas(&Noun::Atom(4), &noun);
}

#[test]
#[should_panic(expected = "fas operation found no child at this address")]
fn test_fas_right_child_node_2() {
    let noun = create_test_noun();
    fas(&Noun::Atom(5), &noun);
}

#[test]
fn test_fas_left_child_node_3() {
    let noun = create_test_noun();
    assert_eq!(fas(&Noun::Atom(6), &noun), Noun::Atom(25));
}

#[test]
fn test_fas_right_child_node_3() {
    let noun = create_test_noun();
    assert_eq!(fas(&Noun::Atom(7), &noun), Noun::Atom(99));
}

#[test]
#[should_panic(expected = "fas operation found no child at this address")]
fn test_fas_out_of_bounds() {
    let noun = create_test_noun();
    fas(&Noun::Atom(12), &noun);
}

#[test]
fn test_fas_on_atom() {
    let noun = Noun::Atom(42);
    assert_eq!(fas(&Noun::Atom(1), Noun::Atom(42)));
}

#[test]
#[should_panic(expected = "fas operation does not support 0 address")]
fn test_fas_with_zero_address() {
    let noun = create_test_noun();
    fas(&Noun::Atom(0), &noun);
}

#[test]
#[should_panic(expected = "fas operation does not support cell address")]
fn test_fas_with_cell_address() {
    let noun = create_test_noun();
    let address = Noun::Cell(Box::new(Noun::Atom(1)), Box::new(Noun::Atom(2)));
    fas(&address, &noun);
}
