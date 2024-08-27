use nock_interpreter::*;

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
    let mut noun = create_test_noun();
    let noun_clone = noun.clone();
    assert_eq!(*fas(&Noun::Atom(1), &mut noun), noun_clone);
}

#[test]
fn test_fas_left_child_node_1() {
    let mut noun = create_test_noun();
    assert_eq!(*fas(&Noun::Atom(2), &mut noun), Noun::Atom(531));
}

#[test]
fn test_fas_right_child_node_1() {
    let mut noun = create_test_noun();
    let expected = Noun::Cell(Box::new(Noun::Atom(25)), Box::new(Noun::Atom(99)));
    assert_eq!(*fas(&Noun::Atom(3), &mut noun), expected);
}

#[test]
#[should_panic(expected = "fas operation found no child at this address")]
fn test_fas_left_child_node_2() {
    let mut noun = create_test_noun();
    fas(&Noun::Atom(4), &mut noun);
}

#[test]
#[should_panic(expected = "fas operation found no child at this address")]
fn test_fas_right_child_node_2() {
    let mut noun = create_test_noun();
    fas(&Noun::Atom(5), &mut noun);
}

#[test]
fn test_fas_left_child_node_3() {
    let mut noun = create_test_noun();
    assert_eq!(*fas(&Noun::Atom(6), &mut noun), Noun::Atom(25));
}

#[test]
fn test_fas_right_child_node_3() {
    let mut noun = create_test_noun();
    assert_eq!(*fas(&Noun::Atom(7), &mut noun), Noun::Atom(99));
}

#[test]
#[should_panic(expected = "fas operation found no child at this address")]
fn test_fas_out_of_bounds() {
    let mut noun = create_test_noun();
    fas(&Noun::Atom(12), &mut noun);
}

#[test]
fn test_fas_on_atom() {
    let mut noun = Noun::Atom(42);
    assert_eq!(*fas(&Noun::Atom(1), &mut noun), Noun::Atom(42));
}

#[test]
#[should_panic(expected = "fas operation does not support 0 address")]
fn test_fas_with_zero_address() {
    let mut noun = create_test_noun();
    fas(&Noun::Atom(0), &mut noun);
}

#[test]
#[should_panic(expected = "fas operation does not support cell address")]
fn test_fas_with_cell_address() {
    let mut noun = create_test_noun();
    let address = Noun::Cell(Box::new(Noun::Atom(1)), Box::new(Noun::Atom(2)));
    fas(&address, &mut noun);
}
