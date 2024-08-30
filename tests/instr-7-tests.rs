use nock_interpreter::*;

#[test]
fn test_instruction_7_simple_composition() {
    // [42 [7 [4 0 1] [0 1]]] should increment the subject and then return it
    let subject = Noun::Atom(42);
    let formula = Noun::cell(
        Noun::Atom(7),
        Noun::cell(
            Noun::cell(Noun::Atom(4), Noun::cell(Noun::Atom(0), Noun::Atom(1))),
            Noun::cell(Noun::Atom(0), Noun::Atom(1)),
        ),
    );
    let result = tar(&mut Noun::cell(subject, formula));
    assert_eq!(result, Noun::Atom(43));
}

#[test]
fn test_instruction_7_nested_composition() {
    // [42 [7 [7 [4 0 1] [4 0 1]] [0 1]]] should increment the subject twice and then return it
    let subject = Noun::Atom(42);
    let formula = Noun::cell(
        Noun::Atom(7),
        Noun::cell(
            Noun::cell(
                Noun::Atom(7),
                Noun::cell(
                    Noun::cell(Noun::Atom(4), Noun::cell(Noun::Atom(0), Noun::Atom(1))),
                    Noun::cell(Noun::Atom(4), Noun::cell(Noun::Atom(0), Noun::Atom(1))),
                ),
            ),
            Noun::cell(Noun::Atom(0), Noun::Atom(1)),
        ),
    );
    let result = tar(&mut Noun::cell(subject, formula));
    assert_eq!(result, Noun::Atom(44));
}

#[test]
fn test_instruction_7_composition_with_constant() {
    // [42 [7 [1 100] [4 0 1]]] should replace the subject with 100, then increment it
    let subject = Noun::Atom(42);
    let formula = Noun::cell(
        Noun::Atom(7),
        Noun::cell(
            Noun::cell(Noun::Atom(1), Noun::Atom(100)),
            Noun::cell(Noun::Atom(4), Noun::cell(Noun::Atom(0), Noun::Atom(1))),
        ),
    );
    let result = tar(&mut Noun::cell(subject, formula));
    assert_eq!(result, Noun::Atom(101));
}

#[test]
fn test_instruction_7_complex_composition() {
    // [42 [7 [4 0 1] [7 [4 0 1] [3 0 1]]]] should increment the subject, then increment again and check if it's a cell
    let subject = Noun::Atom(42);
    let formula = Noun::cell(
        Noun::Atom(7),
        Noun::cell(
            Noun::cell(Noun::Atom(4), Noun::cell(Noun::Atom(0), Noun::Atom(1))),
            Noun::cell(
                Noun::Atom(7),
                Noun::cell(
                    Noun::cell(Noun::Atom(4), Noun::cell(Noun::Atom(0), Noun::Atom(1))),
                    Noun::cell(Noun::Atom(3), Noun::cell(Noun::Atom(0), Noun::Atom(1))),
                ),
            ),
        ),
    );
    let result = tar(&mut Noun::cell(subject, formula));
    assert_eq!(result, Noun::Atom(1)); // 1 because 44 is an atom, not a cell
}
