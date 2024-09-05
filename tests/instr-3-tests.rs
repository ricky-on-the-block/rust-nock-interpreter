#[cfg(test)]
mod tests {
    use nock_interpreter::nock_4k_rc::nock_4k_rc::Noun;
    use nock_interpreter::*;

    #[test]
    fn test_tar_operator_3_with_atom() {
        let input = noun![42 [3 [0 1]]];
        let result = Noun::tar(input);
        assert_eq!(result, noun![1]);
    }

    #[test]
    fn test_tar_operator_3_with_cell() {
        let input = noun![[1 2] [3 [0 1]]];
        let result = Noun::tar(input);
        assert_eq!(result, noun![0]);
    }
}
