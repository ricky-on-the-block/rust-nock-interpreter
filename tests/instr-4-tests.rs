#[cfg(test)]
mod tests {
    use nock_interpreter::nock_4k_rc::nock_4k_rc::Noun;
    use nock_interpreter::*;

    #[test]
    fn test_tar_operator_4_simple_increment() {
        let input = noun![42 [4 [0 1]]];
        let result = Noun::tar(input);
        assert_eq!(result, Noun::atom(43));
    }

    #[test]
    fn test_tar_operator_4_nested_increment() {
        let input = noun![[10 20] [4 [0 2]]];
        let result = Noun::tar(input);
        assert_eq!(result, Noun::atom(11));
    }

    #[test]
    fn test_tar_operator_4_double_increment() {
        let input = noun![30 [4 [4 [0 1]]]];
        let result = Noun::tar(input);
        assert_eq!(result, Noun::atom(32));
    }

    #[test]
    fn test_tar_operator_4_increment_zero() {
        let input = noun![0 [4 [0 1]]];
        let result = Noun::tar(input);
        assert_eq!(result, Noun::atom(1));
    }

    #[test]
    fn test_tar_operator_4_overflow() {
        let input = noun![18_446_744_073_709_551_615u64 [4 [0 1]]];
        let result = Noun::tar(input);
        assert_eq!(result, Noun::atom(0));
    }
}
