use nock_interpreter::*;

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
                                                        Noun::cell(Noun::atom(0), Noun::atom(6)),
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

    let mut input = Noun::cell(subject, formula);

    // Execute the Nock computation
    let result = tar(&mut input);
    assert_eq!(result, Noun::Atom(100 - 1));
}
