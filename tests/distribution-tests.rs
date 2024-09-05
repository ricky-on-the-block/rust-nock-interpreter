#[cfg(test)]
mod tests {
    use nock_interpreter::nock_4k_rc::nock_4k_rc::Noun;

    #[test]
    fn test_tar_distribution_simple() {
        // Test case: *[42 [[0 1] [1 2]]]
        // This test demonstrates basic distribution behavior:
        // 1. [0 1] retrieves the subject (42)
        // 2. [1 2] is a constant producing 2
        // Expected result: [42 2]
        let subject = Noun::atom(42);
        let formula = Noun::cell(
            Noun::cell(Noun::atom(0), Noun::atom(1)),
            Noun::cell(Noun::atom(1), Noun::atom(2)),
        );
        let input = Noun::cell(subject.clone(), formula);
        let result = Noun::tar(input);
        assert_eq!(result, Noun::cell(subject, Noun::atom(2)));
    }

    #[test]
    fn test_tar_distribution_complex() {
        // Test case: *[[10 20] [[0 2] [0 3]]]
        // This test shows distribution with a cell subject:
        // 1. [0 2] retrieves the head of the subject (10)
        // 2. [0 3] retrieves the tail of the subject (20)
        // Expected result: [10 20]
        let subject = Noun::cell(Noun::atom(10), Noun::atom(20));
        let formula = Noun::cell(
            Noun::cell(Noun::atom(0), Noun::atom(2)),
            Noun::cell(Noun::atom(0), Noun::atom(3)),
        );
        let input = Noun::cell(subject, formula);
        let result = Noun::tar(input);
        assert_eq!(result, Noun::cell(Noun::atom(10), Noun::atom(20)));
    }

    #[test]
    fn test_tar_distribution_with_constant() {
        // Test case: *[42 [[1 100] [1 200]]]
        // This test demonstrates distribution with constant values:
        // 1. [1 100] is a constant producing 100
        // 2. [1 200] is a constant producing 200
        // The subject (42) is not used in this case
        // Expected result: [100 200]
        let subject = Noun::atom(42);
        let formula = Noun::cell(
            Noun::cell(Noun::atom(1), Noun::atom(100)),
            Noun::cell(Noun::atom(1), Noun::atom(200)),
        );
        let input = Noun::cell(subject, formula);
        let result = Noun::tar(input);
        assert_eq!(result, Noun::cell(Noun::atom(100), Noun::atom(200)));
    }
}
