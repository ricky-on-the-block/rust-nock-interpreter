use crate::nock_4k_rc::nock_4k_rc::Noun;

impl Noun {
    /// Implements the Nock '#' operator, pronounced 'hax'
    ///
    /// Replaces a part of a Noun tree at a specified address with a replacement Noun.
    /// Modifies the original tree in place and returns a clone of the modified tree.
    ///
    /// # Arguments
    ///
    /// * `address` - A reference to a Noun representing the address in the tree where the replacement should occur
    /// * `replacement` - A reference to the Noun that will replace the existing Noun at the specified address
    /// * `tree` - A reference to the Noun tree that will be modified / replaced
    ///
    /// # Returns
    ///
    /// A clone of the modified Noun tree
    ///
    /// # Panics
    ///
    /// This function will panic in the same cases as the `fas` function:
    /// - When the address is 0
    /// - When the address is a Cell
    /// - When no valid child is found at the given address
    pub(super) fn hax(address: &Noun, replacement: &Noun, tree: &Noun) -> Noun {
        match address {
            // #[1 a b]          a
            Noun::Atom(1) => replacement.clone(),
            // #[(a + a) b c]    #[a [b /[(a + a + 1) c]] c]  -  even case
            Noun::Atom(a) if *a % 2 == 0 => Self::hax(
                &Noun::Atom(*a / 2),
                &Noun::cell(replacement.clone(), Self::fas(&Noun::Atom(*a + 1), tree)),
                tree,
            ),
            // #[(a + a + 1) b c]    #[a [/[(a + a) c] b] c]  -  odd case
            Noun::Atom(a) if *a % 2 == 1 => Self::hax(
                &Noun::Atom(*a / 2), // this div floors to get back to base a
                &Noun::cell(Self::fas(&Noun::atom(*a - 1), tree), replacement.clone()),
                tree,
            ),
            _ => panic!("hax: ERROR 3"),
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    // Test the Nock '#' operator, pronounced 'hax'
    #[test]
    fn test_hax_replace_root() {
        let original = noun![22 33];
        let replacement = noun![11];
        let result = Noun::hax(&noun![1], &replacement, &original);
        assert_eq!(result, replacement);
    }

    #[test]
    fn test_hax_replace_atom_addr_2() {
        let original = noun![22 33];
        let replacement = noun![11];
        let result = Noun::hax(&noun![2], &replacement, &original);
        let expected = noun![11 33];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_hax_replace_atom_addr_3() {
        let original = noun![22 33];
        let replacement = noun![11];
        let result = Noun::hax(&noun![3], &replacement, &original);
        let expected = noun![22 11];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_hax_replace_cell_addr_3() {
        let original = noun![22 [33 44]];
        let replacement = noun![55 66];
        let result = Noun::hax(&noun![3], &replacement, &original);
        let expected = noun![22 [55 66]];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_hax_replace_atom_with_cell_addr_3() {
        let original = noun![22 33];
        let replacement = noun![44 55];
        let result = Noun::hax(&noun![3], &replacement, &original);
        let expected = noun![22 [44 55]];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_hax_replace_cell_with_atom_addr_3() {
        let original = noun![22 [33 44]];
        let replacement = noun![11];
        let result = Noun::hax(&noun![3], &replacement, &original);
        let expected = noun![22 11];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_hax_replace_atom_addr_4() {
        let original = noun![[22 33] 44];
        let replacement = noun![11];
        let result = Noun::hax(&noun![4], &replacement, &original);
        let expected = noun![[11 33] 44];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_hax_replace_atom_addr_5() {
        let original = noun![[22 33] 44];
        let replacement = noun![11];
        let result = Noun::hax(&noun![5], &replacement, &original);
        let expected = noun![[22 11] 44];
        assert_eq!(result, expected);
    }
}
