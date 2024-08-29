use nock_interpreter::*;

#[test]
fn test_tar_operator_3_with_atom() {
    let mut input = noun![42 [3 [0 1]]];
    let result = tar(&mut input);
    assert_eq!(result, noun![1]);
}

#[test]
fn test_tar_operator_3_with_cell() {
    let mut input = noun![[1 2] [3 [0 1]]];
    let result = tar(&mut input);
    assert_eq!(result, Noun::Atom(0));
}
