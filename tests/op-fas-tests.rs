// #[cfg(test)]
// mod tests {
//     use nock_interpreter::nock_4k_rc::*;
//     use nock_interpreter::*;
//
//     // Test the Nock '/' operator, pronounced 'fas'
//     fn create_test_noun() -> Noun {
//         noun![531 [25 99]]
//     }
//
//     #[test]
//     fn test_fas_root() {
//         let noun = create_test_noun();
//         assert_eq!(fas(&noun![1], &noun), create_test_noun());
//     }
//
//     #[test]
//     fn test_fas_left_child_node_1() {
//         let noun = create_test_noun();
//         assert_eq!(fas(&noun![2], &noun), noun![531]);
//     }
//
//     #[test]
//     fn test_fas_right_child_node_1() {
//         let noun = create_test_noun();
//         let expected = noun![25 99];
//         assert_eq!(fas(&noun![3], &noun), expected);
//     }
//
//     #[test]
//     #[should_panic(expected = "fas operation found no child at this address")]
//     fn test_fas_left_child_node_2() {
//         let noun = create_test_noun();
//         fas(&noun![4], &noun);
//     }
//
//     #[test]
//     #[should_panic(expected = "fas operation found no child at this address")]
//     fn test_fas_right_child_node_2() {
//         let noun = create_test_noun();
//         fas(&noun![5], &noun);
//     }
//
//     #[test]
//     fn test_fas_left_child_node_3() {
//         let noun = create_test_noun();
//         assert_eq!(fas(&noun![6], &noun), noun![25]);
//     }
//
//     #[test]
//     fn test_fas_right_child_node_3() {
//         let noun = create_test_noun();
//         assert_eq!(fas(&noun![7], &noun), noun![99]);
//     }
//
//     #[test]
//     #[should_panic(expected = "fas operation found no child at this address")]
//     fn test_fas_out_of_bounds() {
//         let noun = create_test_noun();
//         fas(&noun![12], &noun);
//     }
//
//     #[test]
//     fn test_fas_on_atom() {
//         let noun = noun![42];
//         assert_eq!(fas(&noun![1], &noun), noun![42]);
//     }
//
//     #[test]
//     #[should_panic(expected = "fas operation does not support 0 address")]
//     fn test_fas_with_zero_address() {
//         let noun = create_test_noun();
//         fas(&noun![0], &noun);
//     }
//
//     #[test]
//     #[should_panic(expected = "fas operation does not support cell address")]
//     fn test_fas_with_cell_address() {
//         let noun = create_test_noun();
//         let address = noun![1 2];
//         fas(&address, &noun);
//     }
// }
