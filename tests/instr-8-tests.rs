#[cfg(test)]
mod tests {
    use nock_interpreter::nock_4k_rc::nock_4k_rc::Noun;

    #[test]
    fn test_instruction_8_simple_variable_declaration() {
        // [8 [1 42] [0 2]] should create a new subject [42 original_subject] and return 42
        let subject = Noun::atom(10);
        let formula = Noun::cell(
            Noun::atom(8),
            Noun::cell(
                Noun::cell(Noun::atom(1), Noun::atom(42)),
                Noun::cell(Noun::atom(0), Noun::atom(2)),
            ),
        );
        let result = Noun::tar(Noun::cell(subject, formula));
        assert_eq!(result, Noun::atom(42));
    }

    #[test]
    fn test_instruction_8_compute_and_use_variable() {
        // [8 [4 0 1] [0 3]] should increment the subject, create a new subject [incremented_subject original_subject],
        // and return the original subject
        let subject = Noun::atom(10);
        let formula = Noun::cell(
            Noun::atom(8),
            Noun::cell(
                Noun::cell(Noun::atom(4), Noun::cell(Noun::atom(0), Noun::atom(1))),
                Noun::cell(Noun::atom(0), Noun::atom(3)),
            ),
        );
        let result = Noun::tar(Noun::cell(subject, formula));
        assert_eq!(result, Noun::atom(10));
    }
}
