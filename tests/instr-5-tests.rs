// #[cfg(test)]
// mod tests {
//     use nock_interpreter::*;
//
//     #[test]
//     fn test_tar_operator_5_equal_atoms() {
//         let mut input = Noun::cell(
//             Noun::atom(42),
//             Noun::cell(
//                 Noun::atom(5),
//                 Noun::cell(
//                     Noun::cell(Noun::atom(0), Noun::atom(1)),
//                     Noun::cell(Noun::atom(1), Noun::atom(42)),
//                 ),
//             ),
//         );
//         let result = tar(&mut input);
//         assert_eq!(result, Noun::atom(0)); // 0 means equal
//     }
//
//     #[test]
//     fn test_tar_operator_5_unequal_atoms() {
//         let mut input = Noun::cell(
//             Noun::atom(42),
//             Noun::cell(
//                 Noun::atom(5),
//                 Noun::cell(
//                     Noun::cell(Noun::atom(0), Noun::atom(1)),
//                     Noun::cell(Noun::atom(1), Noun::atom(43)),
//                 ),
//             ),
//         );
//         let result = tar(&mut input);
//         assert_eq!(result, Noun::atom(1)); // 1 means not equal
//     }
//
//     #[test]
//     fn test_tar_operator_5_equal_cells() {
//         let mut input = Noun::cell(
//             Noun::cell(Noun::atom(1), Noun::atom(2)),
//             Noun::cell(
//                 Noun::atom(5),
//                 Noun::cell(
//                     Noun::cell(Noun::atom(0), Noun::atom(1)),
//                     Noun::cell(Noun::atom(1), Noun::cell(Noun::atom(1), Noun::atom(2))),
//                 ),
//             ),
//         );
//         let result = tar(&mut input);
//         assert_eq!(result, Noun::atom(0)); // 0 means equal
//     }
//
//     #[test]
//     fn test_tar_operator_5_unequal_cells() {
//         let mut input = Noun::cell(
//             Noun::cell(Noun::atom(1), Noun::atom(2)),
//             Noun::cell(
//                 Noun::atom(5),
//                 Noun::cell(
//                     Noun::cell(Noun::atom(0), Noun::atom(1)),
//                     Noun::cell(Noun::atom(1), Noun::cell(Noun::atom(2), Noun::atom(1))),
//                 ),
//             ),
//         );
//         let result = tar(&mut input);
//         assert_eq!(result, Noun::atom(1)); // 1 means not equal
//     }
//
//     #[test]
//     fn test_tar_operator_5_atom_vs_cell() {
//         let mut input = Noun::cell(
//             Noun::atom(42),
//             Noun::cell(
//                 Noun::atom(5),
//                 Noun::cell(
//                     Noun::cell(Noun::atom(0), Noun::atom(1)),
//                     Noun::cell(Noun::atom(1), Noun::cell(Noun::atom(1), Noun::atom(2))),
//                 ),
//             ),
//         );
//         let result = tar(&mut input);
//         assert_eq!(result, Noun::atom(1)); // 1 means not equal
//     }
//
//     #[test]
//     fn test_tar_operator_5_equal_nested_cells() {
//         let mut input = Noun::cell(
//             Noun::cell(Noun::atom(1), Noun::cell(Noun::atom(2), Noun::atom(3))),
//             Noun::cell(
//                 Noun::atom(5),
//                 Noun::cell(
//                     Noun::cell(Noun::atom(0), Noun::atom(1)),
//                     Noun::cell(
//                         Noun::atom(1),
//                         Noun::cell(Noun::atom(1), Noun::cell(Noun::atom(2), Noun::atom(3))),
//                     ),
//                 ),
//             ),
//         );
//         let result = tar(&mut input);
//         assert_eq!(result, Noun::atom(0)); // 0 means equal
//     }
// }
