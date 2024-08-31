// #[cfg(test)]
// mod tests {
//     use nock_interpreter::*;
//
//     #[test]
//     fn test_instruction_8_simple_variable_declaration() {
//         // [8 [1 42] [0 2]] should create a new subject [42 original_subject] and return 42
//         let subject = Noun::Atom(10);
//         let formula = Noun::cell(
//             Noun::Atom(8),
//             Noun::cell(
//                 Noun::cell(Noun::Atom(1), Noun::Atom(42)),
//                 Noun::cell(Noun::Atom(0), Noun::Atom(2)),
//             ),
//         );
//         let result = tar(&mut Noun::cell(subject, formula));
//         assert_eq!(result, Noun::Atom(42));
//     }
//
//     #[test]
//     fn test_instruction_8_compute_and_use_variable() {
//         // [8 [4 0 1] [0 3]] should increment the subject, create a new subject [incremented_subject original_subject],
//         // and return the original subject
//         let subject = Noun::Atom(10);
//         let formula = Noun::cell(
//             Noun::Atom(8),
//             Noun::cell(
//                 Noun::cell(Noun::Atom(4), Noun::cell(Noun::Atom(0), Noun::Atom(1))),
//                 Noun::cell(Noun::Atom(0), Noun::Atom(3)),
//             ),
//         );
//         let result = tar(&mut Noun::cell(subject, formula));
//         assert_eq!(result, Noun::Atom(10));
//     }
// }
