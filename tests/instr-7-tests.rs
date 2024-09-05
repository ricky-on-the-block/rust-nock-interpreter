#[cfg(test)]
mod tests {
    use nock_interpreter::nock_4k_rc::nock_4k_rc::Noun;

    #[test]
    fn test_instruction_7_simple_composition() {
        // [42 [7 [4 0 1] [0 1]]] should increment the subject and then return it
        let subject = Noun::atom(42);
        let formula = Noun::cell(
            Noun::atom(7),
            Noun::cell(
                Noun::cell(Noun::atom(4), Noun::cell(Noun::atom(0), Noun::atom(1))),
                Noun::cell(Noun::atom(0), Noun::atom(1)),
            ),
        );
        let result = Noun::tar(Noun::cell(subject, formula));
        assert_eq!(result, Noun::atom(43));
    }

    #[test]
    fn test_instruction_7_nested_composition() {
        // [42 [7 [7 [4 0 1] [4 0 1]] [0 1]]] should increment the subject twice and then return it
        let subject = Noun::atom(42);
        let formula = Noun::cell(
            Noun::atom(7),
            Noun::cell(
                Noun::cell(
                    Noun::atom(7),
                    Noun::cell(
                        Noun::cell(Noun::atom(4), Noun::cell(Noun::atom(0), Noun::atom(1))),
                        Noun::cell(Noun::atom(4), Noun::cell(Noun::atom(0), Noun::atom(1))),
                    ),
                ),
                Noun::cell(Noun::atom(0), Noun::atom(1)),
            ),
        );
        let result = Noun::tar(Noun::cell(subject, formula));
        assert_eq!(result, Noun::atom(44));
    }

    #[test]
    fn test_instruction_7_composition_with_constant() {
        // [42 [7 [1 100] [4 0 1]]] should replace the subject with 100, then increment it
        let subject = Noun::atom(42);
        let formula = Noun::cell(
            Noun::atom(7),
            Noun::cell(
                Noun::cell(Noun::atom(1), Noun::atom(100)),
                Noun::cell(Noun::atom(4), Noun::cell(Noun::atom(0), Noun::atom(1))),
            ),
        );
        let result = Noun::tar(Noun::cell(subject, formula));
        assert_eq!(result, Noun::atom(101));
    }

    #[test]
    fn test_instruction_7_complex_composition() {
        // [42 [7 [4 0 1] [7 [4 0 1] [3 0 1]]]] should increment the subject, then increment again and check if it's a cell
        let subject = Noun::atom(42);
        let formula = Noun::cell(
            Noun::atom(7),
            Noun::cell(
                Noun::cell(Noun::atom(4), Noun::cell(Noun::atom(0), Noun::atom(1))),
                Noun::cell(
                    Noun::atom(7),
                    Noun::cell(
                        Noun::cell(Noun::atom(4), Noun::cell(Noun::atom(0), Noun::atom(1))),
                        Noun::cell(Noun::atom(3), Noun::cell(Noun::atom(0), Noun::atom(1))),
                    ),
                ),
            ),
        );
        let result = Noun::tar(Noun::cell(subject, formula));
        assert_eq!(result, Noun::atom(1)); // 1 because 44 is an atom, not a cell
    }
}
