use crate::nock_4k_rc::nock_4k_rc::Noun;

impl Noun {
    /// Implements the Nock '=' operator, pronounced 'tis'
    ///
    /// The tis operator compares two Nouns for equality.
    /// In Nock, 0 represents true (equal) and 1 represents false (not equal).
    ///
    /// # Arguments
    ///
    /// * `noun1` - A reference to the first Noun to be compared
    /// * `noun2` - A reference to the second Noun to be compared
    ///
    /// # Returns
    ///
    /// * `Noun::Atom(0)` if the inputs are equal
    /// * `Noun::Atom(1)` if the inputs are not equal
    pub(super) fn tis(noun1: &Noun, noun2: &Noun) -> Noun {
        if noun1 == noun2 {
            Noun::Atom(0)
        } else {
            Noun::Atom(1)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    // Test the Nock '=' operator, pronounced 'tis'
    #[test]
    fn test_tis_equal_atoms() {
        let atom1 = noun![42];
        let atom2 = noun![42];
        assert_eq!(Noun::tis(&atom1, &atom2), noun![0]);
    }

    #[test]
    fn test_tis_unequal_atoms() {
        let atom1 = noun![42];
        let atom2 = noun![43];
        assert_eq!(Noun::tis(&atom1, &atom2), noun![1]);
    }

    #[test]
    fn test_tis_equal_cells() {
        let cell1 = noun![1 2];
        let cell2 = noun![1 2];
        assert_eq!(Noun::tis(&cell1, &cell2), noun![0]);
    }

    #[test]
    fn test_tis_unequal_cells() {
        let cell1 = noun![1 2];
        let cell2 = noun![1 3];
        assert_eq!(Noun::tis(&cell1, &cell2), noun![1]);
    }

    #[test]
    fn test_tis_unequal_atom_and_cell() {
        let atom = noun![42];
        let cell = noun![42 42];
        assert_eq!(Noun::tis(&atom, &cell), noun![1]);
    }

    #[test]
    fn test_tis_equal_nested_cells() {
        let cell1 = noun![1 [2 3]];
        let cell2 = noun![1 [2 3]];
        assert_eq!(Noun::tis(&cell1, &cell2), noun![0]);
    }
}
