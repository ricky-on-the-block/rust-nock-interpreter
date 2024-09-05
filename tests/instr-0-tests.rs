#[cfg(test)]
mod tests {
    use nock_interpreter::nock_4k_rc::nock_4k_rc::Noun;

    #[test]
    fn test_tar_op_0_slot_root() {
        let subject = Noun::cell(Noun::atom(42), Noun::atom(43));
        let formula = Noun::cell(Noun::atom(0), Noun::atom(1));
        let result = Noun::tar(Noun::cell(subject.clone(), formula));
        assert_eq!(result, subject);
    }

    #[test]
    fn test_tar_op_0_slot_head() {
        let subject = Noun::cell(Noun::atom(42), Noun::atom(43));
        let formula = Noun::cell(Noun::atom(0), Noun::atom(2));
        let result = Noun::tar(Noun::cell(subject, formula));
        assert_eq!(result, Noun::atom(42));
    }

    #[test]
    fn test_tar_op_0_slot_tail() {
        let subject = Noun::cell(Noun::atom(42), Noun::atom(43));
        let formula = Noun::cell(Noun::atom(0), Noun::atom(3));
        let result = Noun::tar(Noun::cell(subject, formula));
        assert_eq!(result, Noun::atom(43));
    }

    #[test]
    fn test_tar_op_0_slot_nested() {
        let subject = Noun::cell(
            Noun::cell(Noun::atom(1), Noun::atom(2)),
            Noun::cell(Noun::atom(3), Noun::atom(4)),
        );
        let formula = Noun::cell(Noun::atom(0), Noun::atom(7));
        let result = Noun::tar(Noun::cell(subject, formula));
        assert_eq!(result, Noun::atom(4));
    }

    #[test]
    #[should_panic(expected = "fas operation found no child at this address")]
    fn test_tar_op_0_slot_invalid() {
        let subject = Noun::cell(Noun::atom(42), Noun::atom(43));
        let formula = Noun::cell(Noun::atom(0), Noun::atom(4));
        Noun::tar(Noun::cell(subject, formula));
    }
}
