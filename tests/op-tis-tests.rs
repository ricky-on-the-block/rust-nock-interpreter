// #[cfg(test)]
// mod tests {
//     use nock_interpreter::*;
//
//     // Test the Nock '=' operator, pronounced 'tis'
//     #[test]
//     fn test_tis_equal_atoms() {
//         let atom1 = noun![42];
//         let atom2 = noun![42];
//         assert_eq!(tis(&atom1, &atom2), noun![0]);
//     }
//
//     #[test]
//     fn test_tis_unequal_atoms() {
//         let atom1 = noun![42];
//         let atom2 = noun![43];
//         assert_eq!(tis(&atom1, &atom2), noun![1]);
//     }
//
//     #[test]
//     fn test_tis_equal_cells() {
//         let cell1 = noun![1 2];
//         let cell2 = noun![1 2];
//         assert_eq!(tis(&cell1, &cell2), noun![0]);
//     }
//
//     #[test]
//     fn test_tis_unequal_cells() {
//         let cell1 = noun![1 2];
//         let cell2 = noun![1 3];
//         assert_eq!(tis(&cell1, &cell2), noun![1]);
//     }
//
//     #[test]
//     fn test_tis_unequal_atom_and_cell() {
//         let atom = noun![42];
//         let cell = noun![42 42];
//         assert_eq!(tis(&atom, &cell), noun![1]);
//     }
//
//     #[test]
//     fn test_tis_equal_nested_cells() {
//         let cell1 = noun![1 [2 3]];
//         let cell2 = noun![1 [2 3]];
//         assert_eq!(tis(&cell1, &cell2), noun![0]);
//     }
// }
