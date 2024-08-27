use nock_interpreter::*;

#[test]
fn test_tar_constant_atom() {
    let subject = Noun::Atom(42);
    let formula = Noun::Cell(Box::new(Noun::Atom(1)), Box::new(Noun::Atom(123)));
    let mut input = Noun::Cell(Box::new(subject), Box::new(formula));
    let result = tar(&mut input);
    assert_eq!(result, Noun::Atom(123));
}

#[test]
fn test_tar_constant_cell() {
    let subject = Noun::Atom(42);
    let constant = Noun::Cell(Box::new(Noun::Atom(1)), Box::new(Noun::Atom(2)));
    let formula = Noun::Cell(Box::new(Noun::Atom(1)), Box::new(constant.clone()));
    let mut input = Noun::Cell(Box::new(subject), Box::new(formula));
    let result = tar(&mut input);
    assert_eq!(result, constant);
}

#[test]
fn test_tar_constant_nested_cell() {
    let subject = Noun::Atom(42);
    let constant = Noun::Cell(
        Box::new(Noun::Cell(Box::new(Noun::Atom(1)), Box::new(Noun::Atom(2)))),
        Box::new(Noun::Atom(3)),
    );
    let formula = Noun::Cell(Box::new(Noun::Atom(1)), Box::new(constant.clone()));
    let mut input = Noun::Cell(Box::new(subject), Box::new(formula));
    let result = tar(&mut input);
    assert_eq!(result, constant);
}

#[test]
fn test_tar_constant_ignores_subject() {
    let subject1 = Noun::Atom(42);
    let subject2 = Noun::Atom(43);
    let formula = Noun::Cell(Box::new(Noun::Atom(1)), Box::new(Noun::Atom(123)));
    let mut input1 = Noun::Cell(Box::new(subject1), Box::new(formula.clone()));
    let mut input2 = Noun::Cell(Box::new(subject2), Box::new(formula));
    let result1 = tar(&mut input1);
    let result2 = tar(&mut input2);
    assert_eq!(result1, result2);
    assert_eq!(result1, Noun::Atom(123));
}
