use nock_interpreter::*;

#[test]
fn test_tar_op_0_slot_root() {
    let subject = Noun::Cell(Box::new(Noun::Atom(42)), Box::new(Noun::Atom(43)));
    let formula = Noun::Cell(Box::new(Noun::Atom(0)), Box::new(Noun::Atom(1)));
    let result = tar(&mut Noun::Cell(
        Box::new(subject.clone()),
        Box::new(formula),
    ));
    assert_eq!(result, subject);
}

#[test]
fn test_tar_op_0_slot_head() {
    let subject = Noun::Cell(Box::new(Noun::Atom(42)), Box::new(Noun::Atom(43)));
    let formula = Noun::Cell(Box::new(Noun::Atom(0)), Box::new(Noun::Atom(2)));
    let result = tar(&mut Noun::Cell(Box::new(subject), Box::new(formula)));
    assert_eq!(result, Noun::Atom(42));
}

#[test]
fn test_tar_op_0_slot_tail() {
    let subject = Noun::Cell(Box::new(Noun::Atom(42)), Box::new(Noun::Atom(43)));
    let formula = Noun::Cell(Box::new(Noun::Atom(0)), Box::new(Noun::Atom(3)));
    let result = tar(&mut Noun::Cell(Box::new(subject), Box::new(formula)));
    assert_eq!(result, Noun::Atom(43));
}

#[test]
fn test_tar_op_0_slot_nested() {
    let subject = Noun::Cell(
        Box::new(Noun::Cell(Box::new(Noun::Atom(1)), Box::new(Noun::Atom(2)))),
        Box::new(Noun::Cell(Box::new(Noun::Atom(3)), Box::new(Noun::Atom(4)))),
    );
    let formula = Noun::Cell(Box::new(Noun::Atom(0)), Box::new(Noun::Atom(7)));
    let result = tar(&mut Noun::Cell(Box::new(subject), Box::new(formula)));
    assert_eq!(result, Noun::Atom(4));
}

#[test]
#[should_panic(expected = "fas operation found no child at this address")]
fn test_tar_op_0_slot_invalid() {
    let subject = Noun::Cell(Box::new(Noun::Atom(42)), Box::new(Noun::Atom(43)));
    let formula = Noun::Cell(Box::new(Noun::Atom(0)), Box::new(Noun::Atom(4)));
    tar(&mut Noun::Cell(Box::new(subject), Box::new(formula)));
}
