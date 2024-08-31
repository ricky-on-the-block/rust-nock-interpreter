// #[cfg(test)]
// mod tests {
//     use nock_interpreter::nock_4k_rc::*;
//
//     // Test the Nock '+' operator, pronounced 'lus'
//     #[test]
//     fn test_lus_on_zero() {
//         let zero = noun![0];
//         assert_eq!(lus(&zero), noun![1]);
//     }
//
//     #[test]
//     fn test_lus_on_positive_integer() {
//         let num = noun![42];
//         assert_eq!(lus(&num), noun![43]);
//     }
//
//     // TODO: Determine what the desired behavior actually is on uint boundaries
//     #[test]
//     fn test_lus_on_max_u64() {
//         let max = noun![18_446_744_073_709_551_615u64];
//         assert_eq!(lus(&max), noun![0]); // Expect wrapping behavior
//     }
//
//     #[test]
//     #[should_panic(expected = "lus operation is not defined for cells, only atoms")]
//     fn test_lus_on_cell() {
//         let cell = noun![1 2];
//         lus(&cell);
//     }
// }
