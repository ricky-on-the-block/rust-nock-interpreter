mod rc_op_fas;
mod rc_op_hax;
mod rc_op_lus;
mod rc_op_tis;
mod rc_op_wut;

pub mod nock_4k_rc {
    use std::fmt;
    use std::rc::Rc;

    #[derive(Debug, PartialEq)]
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
        pub fn atom(value: u64) -> Rc<Self> {
            Rc::new(Noun::Atom(value))
        }

        // Helper function to create a Cell
        pub fn cell(left: Rc<Noun>, right: Rc<Noun>) -> Rc<Self> {
            Rc::new(Noun::Cell(left, right))
        }
    }

    #[macro_export]
    macro_rules! noun {
        // Rule 1: Match a single literal (base case for atoms)
        [$num:literal] => {
            Noun::atom($num)
        };

        // Rule 2: Match two literals (simple cell)
        [$num1:literal $num2:literal] => {
            Noun::cell(Noun::atom($num1), Noun::atom($num2))
        };

        // Rule 3: Match two nested structures
        [[$($left:tt)+] [$($right:tt)+]] => {
            Noun::cell(noun![$($left)+], noun![$($right)+])
        };

        // Rule 4: Match a nested structure on the left and a single token on the right
        [[$($left:tt)+] $right:tt] => {
            Noun::cell(noun![$($left)+], noun![$right])
        };

        // Rule 5: Match a single token on the left and a nested structure on the right
        [$left:tt [$($right:tt)+]] => {
            Noun::cell(noun![$left], noun![$($right)+])
        };
    }
    impl Noun {
        pub fn tar(noun: Rc<Noun>) -> Rc<Noun> {
            // println!("Evaluating tar with:");
            // println!(
            //     "  Subject: {}",
            //     match noun.as_ref() {
            //         Noun::Cell(subject, formula) =>
            //             format!("[subject:{}, formula:{}]", subject, formula),
            //         _ => "Invalid".to_string(),
            //     }
            // );

            match noun.as_ref() {
                Noun::Atom(_) => panic!("tar cannot be performed on an atom"),
                Noun::Cell(subject, formula) => match formula.as_ref() {
                    Noun::Cell(op, tail) => match (op.as_ref(), tail.as_ref()) {
                        // Distribution (cons)
                        (Noun::Cell(..), _) => {
                            Noun::cell(
                                Self::tar(Noun::cell(subject.clone(), op.clone())),
                                Self::tar(Noun::cell(subject.clone(), tail.clone()))
                            )
                        },
                        // Instructions
                        (Noun::Atom(0), _) => Self::fas(tail.clone(), subject.clone()).clone(),
                        (Noun::Atom(1), _) => tail.clone(),
                        (Noun::Atom(3), _) => Self::wut(Self::tar(Noun::cell(subject.clone(), tail.clone()))),
                        (Noun::Atom(4), _) => Self::lus(Self::tar(Noun::cell(subject.clone(), tail.clone()))),

                        // Operations that expect a cell as their tail
                        (Noun::Atom(2), Noun::Cell(b, c)) => Self::tar(Noun::cell(
                            Self::tar(Noun::cell(subject.clone(), b.clone())),
                            Self::tar(Noun::cell(subject.clone(), c.clone())))
                        ),
                        (Noun::Atom(5), Noun::Cell(b, c)) => Self::tis(
                            Self::tar(Noun::cell(subject.clone(), b.clone())),
                            Self::tar(Noun::cell(subject.clone(), c.clone()))
                        ),

                    // *[a 6 b c d]    *[a *[[c d] 0 *[[2 3] 0 *[a 4 4 b]]]]
                    (Noun::Atom(6), Noun::Cell(b, six_tail)) => match six_tail.as_ref() {
                        Noun::Cell(c, d) =>
                            Self::tar(Noun::cell(
                                subject.clone(),
                                Self::tar(Noun::cell(
                                    Noun::cell(c.clone(), d.clone()),
                                    Noun::cell(
                                        Noun::atom(0),
                                        Self::tar(
                                            Noun::cell(
                                                Noun::cell(Noun::atom(2), Noun::atom(3)),
                                                Noun::cell(
                                                    Noun::atom(0),
                                                    Self::tar(Noun::cell(
                                                        subject.clone(),
                                                        Noun::cell(
                                                            Noun::atom(4),
                                                            Noun::cell(Noun::atom(4), b.clone())
                                                        )
                                                    ))
                                                )
                                            )
                                        )
                                    )
                                ))
                            )),
                        _ => panic!("Invalid structure for instruction 6")
                    },
                    (Noun::Atom(7), Noun::Cell(b, c)) => Self::tar(Noun::cell(
                        Self::tar(Noun::cell(subject.clone(), b.clone())),
                        c.clone()
                    )),
                    // *[a 8 b c]    *[[*[a b] a] c]
                    (Noun::Atom(8), Noun::Cell(b, c)) => Self::tar(Noun::cell(
                        Noun::cell(
                            Self::tar(Noun::cell(subject.clone(), b.clone())),
                            subject.clone()
                        ),
                        c.clone()
                    )),
                    // *[a 9 b c]    *[*[a c] 2 [0 1] 0 b]
                    (Noun::Atom(9), Noun::Cell(b, c)) => Self::tar(Noun::cell(
                        Self::tar(Noun::cell(subject.clone(), c.clone())),
                        Noun::cell(
                            Noun::atom(2),
                            Noun::cell(
                                Noun::cell(Noun::atom(0), Noun::atom(1)),
                                Noun::cell(Noun::atom(0), b.clone())
                            )
                        )
                    )),
                    // *[a 10 [b c] d]     #[b *[a c] *[a d]]
                    (Noun::Atom(10), Noun::Cell(ten_head, d)) => match ten_head.as_ref() {
                        Noun::Cell(b, c) => Self::hax(
                            b.clone(),
                            Self::tar(Noun::cell(subject.clone(), c.clone())),
                            Self::tar(Noun::cell(subject.clone(), d.clone()))),
                        _ => panic!("Invalid structure for instruction 10")
                    },
                    (Noun::Atom(11), Noun::Cell(b, c)) => match &**b {
                        // *[a 11 b c]          *[a c]
                        Noun::Atom(_) => Self::tar(Noun::cell(subject.clone(), c.clone())),
                        // *[a 11 [b1 b2] c]    *[[[*[a b2] *[a c]] 0] 3]
                        Noun::Cell(_, b2) =>
                            Self::tar(
                                Noun::cell(
                                    Noun::cell(
                                        Noun::cell(
                                            Self::tar(Noun::cell(subject.clone(), b2.clone())),
                                            Self::tar(Noun::cell(subject.clone(), c.clone()))
                                        ),
                                        Noun::atom(0)
                                    ),
                                    Noun::atom(3)
                                )
                            )
                    },
                    // Catch case for operations 2 and 5-11 when the tail is not a cell
                    (Noun::Atom(2 | 5..=11), _) => {
                        panic!("Operation {} expects a cell as its argument", op)
                    },
                    _ => panic!("Unimplemented operation or invalid argument structure\nsubject: {}\nformula: {}", subject, formula),
                },
                _ => panic!("Formula must be a cell\nformula: {}", formula),
            },
        }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::nock_4k_rc::nock_4k_rc::Noun;

    #[test]
    fn test_nock_decrement() {
        // Construct the Nock expression for decrement:
        // *[42 [8 [1 0] 8 [1 6 [5 [0 7] 4 0 6] [0 6] 9 2 [0 2] [4 0 6] 0 7] 9 2 0 1]]
        let subject = Noun::atom(100);
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
                                                        Noun::cell(Noun::atom(0), Noun::atom(7)),
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

        let input = Noun::cell(subject, formula);

        // Execute the Nock computation
        let result = Noun::tar(input);
        assert_eq!(result, Noun::atom(100 - 1));
    }
}
