#[cfg(test)]
mod tests {
    use nock_interpreter::nock_4k_rc::nock_4k_rc::Noun;

    #[test]
    fn test_tar_constant_atom() {
        let subject = Noun::atom(42);
        let formula = Noun::cell(Noun::atom(1), Noun::atom(123));
        let input = Noun::cell(subject, formula);
        let result = Noun::tar(input);
        assert_eq!(result, Noun::atom(123));
    }

    #[test]
    fn test_tar_constant_cell() {
        let subject = Noun::atom(42);
        let constant = Noun::cell(Noun::atom(1), Noun::atom(2));
        let formula = Noun::cell(Noun::atom(1), constant.clone());
        let input = Noun::cell(subject, formula);
        let result = Noun::tar(input);
        assert_eq!(result, constant);
    }

    #[test]
    fn test_tar_constant_nested_cell() {
        let subject = Noun::atom(42);
        let constant = Noun::cell(Noun::cell(Noun::atom(1), Noun::atom(2)), Noun::atom(3));
        let formula = Noun::cell(Noun::atom(1), constant.clone());
        let input = Noun::cell(subject, formula);
        let result = Noun::tar(input);
        assert_eq!(result, constant);
    }

    #[test]
    fn test_tar_constant_ignores_subject() {
        let subject1 = Noun::atom(42);
        let subject2 = Noun::atom(43);
        let formula = Noun::cell(Noun::atom(1), Noun::atom(123));
        let input1 = Noun::cell(subject1, formula.clone());
        let input2 = Noun::cell(subject2, formula);
        let result1 = Noun::tar(input1);
        let result2 = Noun::tar(input2);
        assert_eq!(result1, result2);
        assert_eq!(result1, Noun::atom(123));
    }
}
