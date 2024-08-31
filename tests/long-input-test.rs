// use nock_interpreter::*;
//
// #[test]
// fn test_long_input() {
//     // Construct the Nock expression for decrement:
//     // *[[43 42] [8 [1 0] [8 [1 [6 [5 [0 6] [0 14]] [1 0] [6 [5 [0 6] [0 15]] [1 1] [9 2 [[0 2] [4 0 6] [0 7]]]]]] [9 2 0 1]]]]
//     let mut input = Noun::cell(
//         Noun::cell(Noun::atom(43), Noun::atom(42)),
//         Noun::cell(
//             Noun::atom(8),
//             Noun::cell(
//                 Noun::cell(Noun::atom(1), Noun::atom(0)),
//                 Noun::cell(
//                     Noun::atom(8),
//                     Noun::cell(
//                         Noun::cell(
//                             Noun::atom(1),
//                             Noun::cell(
//                                 Noun::atom(6),
//                                 Noun::cell(
//                                     Noun::cell(
//                                         Noun::atom(5),
//                                         Noun::cell(
//                                             Noun::cell(Noun::atom(0), Noun::atom(6)),
//                                             Noun::cell(Noun::atom(0), Noun::atom(14))
//                                         )
//                                     ),
//                                     Noun::cell(
//                                         Noun::cell(Noun::atom(1), Noun::atom(0)),
//                                         Noun::cell(
//                                             Noun::atom(6),
//                                             Noun::cell(
//                                                 Noun::cell(
//                                                     Noun::atom(5),
//                                                     Noun::cell(
//                                                         Noun::cell(Noun::atom(0), Noun::atom(6)),
//                                                         Noun::cell(Noun::atom(0), Noun::atom(15))
//                                                     )
//                                                 ),
//                                                 Noun::cell(
//                                                     Noun::cell(Noun::atom(1), Noun::atom(1)),
//                                                     Noun::cell(
//                                                         Noun::atom(9),
//                                                         Noun::cell(
//                                                             Noun::atom(2),
//                                                             Noun::cell(
//                                                                 Noun::cell(
//                                                                     Noun::cell(Noun::atom(0), Noun::atom(2)),
//                                                                     Noun::cell(
//                                                                         Noun::cell(Noun::atom(4), Noun::atom(0)),
//                                                                         Noun::atom(6)
//                                                                     )
//                                                                 ),
//                                                                 Noun::cell(Noun::atom(0), Noun::atom(7))
//                                                             )
//                                                         )
//                                                     )
//                                                 )
//                                             )
//                                         )
//                                     )
//                                 )
//                             )
//                         ),
//                         Noun::cell(
//                             Noun::atom(9),
//                             Noun::cell(
//                                 Noun::atom(2),
//                                 Noun::cell(Noun::atom(0), Noun::atom(1))
//                             )
//                         )
//                     )
//                 )
//             )
//         )
//     );
//
//     // Execute the Nock computation
//     let result = tar(&mut input);
//     assert_eq!(result, Noun::Atom(0));
// }
