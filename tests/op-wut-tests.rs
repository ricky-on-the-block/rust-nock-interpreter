// #[cfg(test)]
// mod tests {
//     use nock_interpreter::*;
//
//     // Test the Nock '?' operator, pronounced 'wut'
//     #[test]
//     fn test_wut_on_atom() {
//         let atom = noun![42];
//         assert_eq!(wut(&atom), noun![1]);
//
//         let zero_atom = noun![0];
//         assert_eq!(wut(&zero_atom), noun![1]);
//     }
//
//     #[test]
//     fn test_wut_on_cell() {
//         let cell = noun![1 2];
//         assert_eq!(wut(&cell), noun![0]);
//     }
//
//     #[test]
//     fn test_wut_on_nested_cell() {
//         let nested_cell = noun![1 [2 3]];
//         assert_eq!(wut(&nested_cell), noun![0]);
//     }
// }
