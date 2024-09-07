use crate::nock_4k_rc::nock_4k_rc::Noun;

impl Noun {
    /// Implements the Nock '?' operator, pronounced 'wut'
    ///
    /// The wut operator tests whether a noun is a cell or an atom.
    /// In Nock, 0 represents true and 1 represents false.
    ///
    /// # Arguments
    ///
    /// * `noun` - A reference to the Noun to be tested
    ///
    /// # Returns
    ///
    /// * `Noun::Atom(0)` if the input is a cell (true in Nock)
    /// * `Noun::Atom(1)` if the input is an atom (false in Nock)
    pub(super) fn wut(noun: &Noun) -> Noun {
        match noun {
            Noun::Atom(_) => Noun::Atom(1),
            Noun::Cell(_, _) => Noun::Atom(0),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    // Test the Nock '?' operator, pronounced 'wut'
    #[test]
    fn test_wut_on_atom() {
        let atom = noun![42];
        assert_eq!(Noun::wut(&atom), noun![1]);

        let zero_atom = noun![0];
        assert_eq!(Noun::wut(&zero_atom), noun![1]);
    }

    #[test]
    fn test_wut_on_cell() {
        let cell = noun![1 2];
        assert_eq!(Noun::wut(&cell), noun![0]);
    }

    #[test]
    fn test_wut_on_nested_cell() {
        let nested_cell = noun![1 [2 3]];
        assert_eq!(Noun::wut(&nested_cell), noun![0]);
    }
}
