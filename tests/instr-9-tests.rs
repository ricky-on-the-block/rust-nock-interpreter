// #[cfg(test)]
// mod tests {
//     use nock_interpreter::nock_4k_rc::nock_4k_rc::Noun;
//
//     #[test]
//     fn test_instruction_9_invoke() {
//         // We'll create a core that increments its data and then invoke it
//
//         // 1. Set up the initial subject (this could be any noun)
//         let initial_subject = Noun::atom(100);
//
//         // 2. Create the formula for instruction 9
//         //    [9 2 1 42 [4 0 3]]
//         //    This means:
//         //    - 9: Use instruction 9
//         //    - 2: The address of the arm to run (in this case, the entire tail of the core)
//         //    - 1 42 [4 0 3]: This creates our core [42 [4 0 3]]
//         //      - 42 is the core's data
//         //      - [4 0 3] is the core's code (increment the value at address 3, which is the core's data)
//         let formula = Noun::cell(
//             Noun::atom(9),
//             Noun::cell(
//                 Noun::atom(2),
//                 Noun::cell(
//                     Noun::atom(1),
//                     Noun::cell(
//                         Noun::atom(42),
//                         Noun::cell(Noun::atom(4), Noun::cell(Noun::atom(0), Noun::atom(3))),
//                     ),
//                 ),
//             ),
//         );
//
//         // 3. Combine the subject and formula
//         let nock_input = Noun::cell(initial_subject, formula);
//
//         // 4. Execute the Nock computation
//         let result = Noun::tar(nock_input);
//
//         // 5. Check the result
//         // The expected result should be 43 because:
//         // - The core [42 [4 0 3]] is created
//         // - The arm at address 2 ([4 0 3]) is evaluated with the core as its subject
//         // - This arm increments the value at address 3 of its subject (42)
//         // - So the final result is 42 + 1 = 43
//         assert_eq!(result, Noun::atom(43));
//
//         // Note: In a real Nock interpreter, each step of this process would be evaluated:
//         // 1. *[a c] would create the core [42 [4 0 3]]
//         // 2. This core would become the subject for the next evaluation
//         // 3. [2 [0 1] 0 b] would be evaluated with this new subject
//         //    - [0 1] gets the entire subject (the core)
//         //    - 0 b selects the arm from the core (in this case, [4 0 3])
//         // 4. Finally, [4 0 3] would be evaluated with the core as its subject
//     }
// }
//
// // a = 100
// // b = 2
// // c = [1 [42 [4 [0 3]]]]]]
// // d = [42 [4 [0 3]]], result of *[a c]
// // e = 42 (address 2 of d)
// // ---- test_instruction_9_invoke stdout ----
// // Evaluating tar with:
// //   Subject: [subject:100, formula:[9 [2 [1 [42 [4 [0 3]]]]]]] -> instr 9
// // Evaluating tar with:
// //   Subject: [subject:100, formula:[1 [42 [4 [0 3]]]]] -> *[a c]
// // Evaluating tar with:
// //   Subject: [subject:[42 [4 [0 3]]], formula:[2 [[0 1] [0 2]]]] --> outer instr 9 tar
// // Evaluating tar with:
// //   Subject: [subject:[42 [4 [0 3]]], formula:[0 1]]
// // Evaluating tar with:
// //   Subject: [subject:[42 [4 [0 3]]], formula:[0 2]]
// // Evaluating tar with:
// //   Subject: [subject:[42 [4 [0 3]]], formula:42]
// // thread 'test_instruction_9_invoke' panicked at src/nock_4k.rs:315:18:
// // Formula must be a cell
// // formula: 42
//
// // Pseudo-code for *[a *[[c d] 0 *[[2 3] 0 *[a 4 4 b]]]]
//
// // Step 1: Evaluate *[a 4 4 b]
// // let step1 = tar(
// //     cell(
// //         subject.clone(),
// //         cell(
// //             atom(4),
// //             cell(atom(4), b.clone())
// //         )
// //     )
// // );
//
// // // Step 2: Evaluate *[[2 3] 0 step1]
// // let step2 = tar(
// //     cell(
// //         cell(atom(2), atom(3)),
// //         cell(atom(0), step1)
// //     )
// // );
//
// // // Step 3: Construct [[c d] 0 step2]
// // let step3 = cell(
// //     cell(c.clone(), d.clone()),
// //     cell(atom(0), step2)
// // );
//
// // // Final step: Evaluate *[a step3]
// // let result = tar(
// //     cell(subject, step3)
// // );
//
// // The result is the final output
