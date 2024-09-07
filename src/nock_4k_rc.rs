mod rc_op_fas;
mod rc_op_hax;
mod rc_op_lus;
mod rc_op_tis;
mod rc_op_wut;
pub mod nock_4k_rc {
    use std::fmt;
    use std::rc::Rc;

    #[derive(Debug, Clone, PartialEq)]
    pub enum Noun {
        Atom(u64),
        Cell(Rc<Noun>, Rc<Noun>),
    }

    impl fmt::Display for Noun {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                Noun::Atom(value) => write!(f, "{}", value),
                Noun::Cell(op, tail) => write!(f, "[{} {}]", op, tail),
            }
        }
    }

    impl Noun {
        // Helper function to create an Atom
        #[inline]
        pub fn atom(value: u64) -> Self {
            Noun::Atom(value)
        }

        // Helper function to create a Cell
        pub fn cell(left: Noun, right: Noun) -> Self {
            Noun::Cell(Rc::new(left), Rc::new(right))
        }

        pub fn tar(noun: Noun) -> Noun {
            fn tar_trampoline(subject: &Noun, formula: &Noun) -> Noun {
                match formula {
                    Noun::Atom(_) => panic!("tar cannot be performed on an atom"),
                    // Noun::Cell(mut subject, mut formula) => match formula.as_mut() {
                    Noun::Cell(op, tail) => match (op.as_ref(), tail.as_ref()) {
                        // Distribution (cons)
                        (distribute_cell @ Noun::Cell(..), d) => {
                            Noun::cell(tar_trampoline(subject, distribute_cell), tar_trampoline(subject, d))
                        }
                        (Noun::Atom(0), _) => Noun::fas(tail, subject),
                        (Noun::Atom(1), one_tail) => one_tail.clone(),
                        (Noun::Atom(3), _) => Noun::wut(&tar_trampoline(subject, tail)),
                        (Noun::Atom(4), _) => Noun::lus(&tar_trampoline(subject, tail)),
                        // Operations that expect a cell as their tail
                        (Noun::Atom(2), Noun::Cell(b, c)) => {
                            tar_trampoline(&tar_trampoline(subject, b), &tar_trampoline(subject, c))
                        }
                        (Noun::Atom(5), Noun::Cell(b, c)) => {
                            Noun::tis(&tar_trampoline(subject, b), &tar_trampoline(subject, c))
                        }
                        // *[a 6 b c d]    *[a *[[c d] 0 *[[2 3] 0 *[a 4 4 b]]]]
                        (Noun::Atom(6), Noun::Cell(b, six_tail)) => {
                            tar_trampoline(
                                subject,
                                &tar_trampoline(
                                    six_tail,
                                    &Noun::cell(
                                        Noun::Atom(0),
                                        tar_trampoline(
                                            subject,
                                            &Noun::cell(
                                                Noun::Atom(4),
                                                Noun::cell(Noun::Atom(4), b.as_ref().clone()),
                                            ),
                                        ),
                                    ),
                                ),
                            )
                        }
                        (Noun::Atom(7), Noun::Cell(b, c)) => tar_trampoline(&tar_trampoline(subject, b), c),
                        // *[a 8 b c]    *[[*[a b] a] c]
                        (Noun::Atom(8), Noun::Cell(b, c)) => tar_trampoline(
                            &Noun::cell(tar_trampoline(subject, b), subject.clone()),
                            c,
                        ),
                        // *[a 9 b c]    *[*[a c] 2 [0 1] 0 b]
                        (Noun::Atom(9), Noun::Cell(b, c)) => tar_trampoline(
                            &tar_trampoline(subject, c),
                            &Noun::cell(
                                Noun::Atom(2),
                                Noun::cell(
                                    Noun::cell(Noun::Atom(0), Noun::Atom(1)),
                                    Noun::cell(Noun::Atom(0), b.as_ref().clone()),
                                ),
                            ),
                        ),
                        // *[a 10 [b c] d]     #[b *[a c] *[a d]]
                        (Noun::Atom(10), Noun::Cell(head, d)) => match head.as_ref() {
                            Noun::Cell(b, c) => Noun::hax(
                                b,
                                &tar_trampoline(subject, c),
                                &tar_trampoline(subject, d),
                            ),
                            _ => panic!("Invalid structure for instruction 10"),
                        },
                        (Noun::Atom(11), Noun::Cell(b, c)) => match b.as_ref() {
                            // *[a 11 b c]          *[a c]
                            Noun::Atom(_) => tar_trampoline(subject, c),
                            // *[a 11 [b1 b2] c]    *[[[*[a b2] *[a c]] 0] 3]
                            Noun::Cell(_, b2) => tar_trampoline(
                                &Noun::cell(
                                    Noun::cell(tar_trampoline(subject, b2), tar_trampoline(subject, c)),
                                    Noun::Atom(0),
                                ),
                                &Noun::Atom(3),
                            ),
                        },
                        // Catch case for operations 2 and 5-11 when the tail is not a cell
                        (Noun::Atom(2 | 5..=11), _) => {
                            panic!("Operation {} expects a cell as its argument", op)
                        },
                        // Add more operations here as needed
                        _ => panic!("Unimplemented operation or invalid argument structure\nsubject: {}\nformula: {}", subject, formula)
                    }
                }
            }

            match noun {
                Noun::Atom(_) => panic!("tar cannot be performed on an atom"),
                Noun::Cell(subject, formula) => tar_trampoline(subject.as_ref(), formula.as_ref()),
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_nock_box_decrement() {
            const NUM_TO_DECREMENT: u64 = 100;
            // Construct the Nock expression for decrement:
            // *[42 [8 [1 0] 8 [1 6 [5 [0 7] 4 0 6] [0 6] 9 2 [0 2] [4 0 6] 0 7] 9 2 0 1]]
            let subject = Noun::atom(NUM_TO_DECREMENT);
            let formula = Noun::cell(
                Noun::atom(8),
                Noun::cell(
                    Noun::cell(Noun::atom(1), Noun::atom(0)),
                    Noun::cell(
                        Noun::atom(8),
                        Noun::cell(
                            Noun::cell(
                                Noun::atom(1),
                                Noun::cell(
                                    Noun::atom(6),
                                    Noun::cell(
                                        Noun::cell(
                                            Noun::atom(5),
                                            Noun::cell(
                                                Noun::cell(Noun::atom(0), Noun::atom(7)),
                                                Noun::cell(
                                                    Noun::atom(4),
                                                    Noun::cell(Noun::atom(0), Noun::atom(6)),
                                                ),
                                            ),
                                        ),
                                        Noun::cell(
                                            Noun::cell(Noun::atom(0), Noun::atom(6)),
                                            Noun::cell(
                                                Noun::atom(9),
                                                Noun::cell(
                                                    Noun::atom(2),
                                                    Noun::cell(
                                                        Noun::cell(Noun::atom(0), Noun::atom(2)),
                                                        Noun::cell(
                                                            Noun::cell(
                                                                Noun::atom(4),
                                                                Noun::cell(
                                                                    Noun::atom(0),
                                                                    Noun::atom(6),
                                                                ),
                                                            ),
                                                            Noun::cell(
                                                                Noun::atom(0),
                                                                Noun::atom(7),
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                            ),
                                        ),
                                    ),
                                ),
                            ),
                            Noun::cell(
                                Noun::atom(9),
                                Noun::cell(Noun::atom(2), Noun::cell(Noun::atom(0), Noun::atom(1))),
                            ),
                        ),
                    ),
                ),
            );

            // Execute the Nock computation
            let result = Noun::tar(Noun::cell(subject, formula));
            assert_eq!(result, Noun::atom(NUM_TO_DECREMENT - 1));
        }
    }
}
