#[cfg(test)]
mod tests {
    use nock_interpreter::nock_4k_rc::nock_4k_rc::Noun;
    use nock_interpreter::*;

    #[test]
    fn test_macro_simple_atom() {
        assert_eq!(noun![42], Noun::atom(42));
    }

    #[test]
    fn test_macro_simple_cell() {
        assert_eq!(noun![1 2], Noun::cell(Noun::atom(1), Noun::atom(2)));
    }

    #[test]
    fn test_macro_nested_cell_left() {
        assert_eq!(
            noun![[1 2] 3],
            Noun::cell(Noun::cell(Noun::atom(1), Noun::atom(2)), Noun::atom(3))
        );
    }

    #[test]
    fn test_macro_nested_cell_right() {
        assert_eq!(
            noun![1 [2 3]],
            Noun::cell(Noun::atom(1), Noun::cell(Noun::atom(2), Noun::atom(3)))
        );
    }

    #[test]
    fn test_macro_deeply_nested_cell_left() {
        assert_eq!(
            noun![[1 [2 3]] [4 5]],
            Noun::cell(
                Noun::cell(Noun::atom(1), Noun::cell(Noun::atom(2), Noun::atom(3))),
                Noun::cell(Noun::atom(4), Noun::atom(5))
            )
        );
    }

    #[test]
    fn test_macro_deeply_nested_cell_right() {
        assert_eq!(
            noun![1 [[[2 3] [44 56]] [6 7]]],
            Noun::cell(
                Noun::atom(1),
                Noun::cell(
                    Noun::cell(
                        Noun::cell(Noun::atom(2), Noun::atom(3)),
                        Noun::cell(Noun::atom(44), Noun::atom(56))
                    ),
                    Noun::cell(Noun::atom(6), Noun::atom(7))
                )
            )
        );
    }

    #[test]
    fn test_macro_complex_nested_structure() {
        assert_eq!(
            noun![[1 2] [[3 4] [5 6]]],
            Noun::cell(
                Noun::cell(Noun::atom(1), Noun::atom(2)),
                Noun::cell(
                    Noun::cell(Noun::atom(3), Noun::atom(4)),
                    Noun::cell(Noun::atom(5), Noun::atom(6))
                )
            )
        );
    }

    #[test]
    fn test_partial_eq_atoms() {
        assert_eq!(Noun::atom(42), Noun::atom(42));
        assert_ne!(Noun::atom(42), Noun::atom(43));
    }

    #[test]
    fn test_partial_eq_cells() {
        let cell1 = Noun::cell(Noun::atom(1), Noun::atom(2));
        let cell2 = Noun::cell(Noun::atom(1), Noun::atom(2));
        let cell3 = Noun::cell(Noun::atom(1), Noun::atom(3));

        assert_eq!(cell1, cell2);
        assert_ne!(cell1, cell3);
    }

    #[test]
    fn test_partial_eq_nested() {
        let nested1 = Noun::cell(Noun::atom(1), Noun::cell(Noun::atom(2), Noun::atom(3)));
        let nested2 = Noun::cell(Noun::atom(1), Noun::cell(Noun::atom(2), Noun::atom(3)));
        let nested3 = Noun::cell(Noun::atom(1), Noun::cell(Noun::atom(2), Noun::atom(4)));

        assert_eq!(nested1, nested2);
        assert_ne!(nested1, nested3);
    }

    #[test]
    fn test_clone_atom() {
        let atom = Noun::atom(42);
        let cloned_atom = atom.clone();
        assert_eq!(atom, cloned_atom);

        // Ensure it's a deep copy
        match cloned_atom {
            Noun::Atom(value) => assert_eq!(value, 42),
            _ => panic!("Expected atom"),
        }
    }

    #[test]
    fn test_clone_cell() {
        let cell = Noun::cell(Noun::atom(1), Noun::atom(2));
        let cloned_cell = cell.clone();
        assert_eq!(cell, cloned_cell);

        // Ensure it's a deep copy
        match cloned_cell {
            Noun::Cell(h, t) => {
                assert_eq!(*h, Noun::atom(1));
                assert_eq!(*t, Noun::atom(2));
            }
            _ => panic!("Expected cell"),
        }
    }

    #[test]
    fn test_clone_nested() {
        let nested = Noun::cell(Noun::atom(1), Noun::cell(Noun::atom(2), Noun::atom(3)));
        let cloned_nested = nested.clone();
        assert_eq!(nested, cloned_nested);

        // Ensure it's a deep copy
        match cloned_nested {
            Noun::Cell(h, t) => {
                assert_eq!(*h, Noun::Atom(1));
                match t.as_ref() {
                    Noun::Cell(inner_h, inner_t) => {
                        assert_eq!(**inner_h, Noun::Atom(2));
                        assert_eq!(**inner_t, Noun::Atom(3));
                    }
                    _ => panic!("Expected nested cell"),
                }
            }
            _ => panic!("Expected cell"),
        }
    }

    #[test]
    fn test_display_atom() {
        let atom = Noun::atom(42);
        assert_eq!(format!("{}", atom), "42");
    }

    #[test]
    fn test_display_simple_cell() {
        let cell = Noun::cell(Noun::atom(1), Noun::atom(2));
        assert_eq!(format!("{}", cell), "[1 2]");
    }

    #[test]
    fn test_display_nested_cell() {
        let nested = Noun::cell(Noun::atom(1), Noun::cell(Noun::atom(2), Noun::atom(3)));
        assert_eq!(format!("{}", nested), "[1 [2 3]]");
    }

    #[test]
    fn test_display_complex_nested_cell() {
        let complex = Noun::cell(
            Noun::cell(Noun::atom(1), Noun::atom(2)),
            Noun::cell(Noun::atom(3), Noun::atom(4)),
        );
        assert_eq!(format!("{}", complex), "[[1 2] [3 4]]");
    }
}
