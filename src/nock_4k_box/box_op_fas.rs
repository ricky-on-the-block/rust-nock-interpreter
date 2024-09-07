use crate::nock_4k_box::nock_4k_box::Noun;

impl Noun {
    /// Implements the Nock '/' operator, pronounced 'fas'
    ///
    /// Performs tree addressing in Nock.
    /// The root of the tree is 1; the left child of any node n is 2n; the right child is 2n+1.
    ///
    /// # Arguments
    ///
    /// * `address` - A reference to the Noun representing the address
    /// * `tree` - A mutable reference to the Noun representing the tree to be accessed
    ///
    /// # Returns
    ///
    /// A mutable reference to the Noun at the specified address in the tree
    ///
    /// # Panics
    ///
    /// - When the address is 0
    /// - When the address is a Cell
    /// - When no valid child is found at the given address
    pub(super) fn fas(address: &Noun, tree: &Noun) -> Noun {
        match address {
            Noun::Atom(0) => panic!("fas operation does not support 0 address"),
            Noun::Atom(1) => tree.clone(),
            Noun::Atom(n) if *n == 2 || *n == 3 => match tree {
                Noun::Cell(op, tail) => {
                    if *n == 2 {
                        *op.clone()
                    } else {
                        *tail.clone()
                    }
                }
                Noun::Atom(_) => panic!("fas operation found no child at this address"),
            },
            Noun::Atom(n) => match tree {
                Noun::Cell(..) => Self::fas(
                    &Noun::Atom(2 + *n % 2),
                    &Self::fas(&Noun::Atom(*n / 2), tree),
                ),
                Noun::Atom(_) => panic!("fas operation found no child at this address"),
            },
            Noun::Cell(..) => panic!("fas operation does not support cell address"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    // Test the Nock '/' operator, pronounced 'fas'
    fn create_test_noun() -> Noun {
        noun![531 [25 99]]
    }

    #[test]
    fn test_fas_root() {
        let noun = create_test_noun();
        assert_eq!(Noun::fas(&noun![1], &noun), create_test_noun());
    }

    #[test]
    fn test_fas_left_child_node_1() {
        let noun = create_test_noun();
        assert_eq!(Noun::fas(&noun![2], &noun), noun![531]);
    }

    #[test]
    fn test_fas_right_child_node_1() {
        let noun = create_test_noun();
        let expected = noun![25 99];
        assert_eq!(Noun::fas(&noun![3], &noun), expected);
    }

    #[test]
    #[should_panic(expected = "fas operation found no child at this address")]
    fn test_fas_left_child_node_2() {
        let noun = create_test_noun();
        Noun::fas(&noun![4], &noun);
    }

    #[test]
    #[should_panic(expected = "fas operation found no child at this address")]
    fn test_fas_right_child_node_2() {
        let noun = create_test_noun();
        Noun::fas(&noun![5], &noun);
    }

    #[test]
    fn test_fas_left_child_node_3() {
        let noun = create_test_noun();
        assert_eq!(Noun::fas(&noun![6], &noun), noun![25]);
    }

    #[test]
    fn test_fas_right_child_node_3() {
        let noun = create_test_noun();
        assert_eq!(Noun::fas(&noun![7], &noun), noun![99]);
    }

    #[test]
    #[should_panic(expected = "fas operation found no child at this address")]
    fn test_fas_out_of_bounds() {
        let noun = create_test_noun();
        Noun::fas(&noun![12], &noun);
    }

    #[test]
    fn test_fas_on_atom() {
        let noun = noun![42];
        assert_eq!(Noun::fas(&noun![1], &noun), noun![42]);
    }

    #[test]
    #[should_panic(expected = "fas operation does not support 0 address")]
    fn test_fas_with_zero_address() {
        let noun = create_test_noun();
        Noun::fas(&noun![0], &noun);
    }

    #[test]
    #[should_panic(expected = "fas operation does not support cell address")]
    fn test_fas_with_cell_address() {
        let noun = create_test_noun();
        let address = noun![1 2];
        Noun::fas(&address, &noun);
    }
}
