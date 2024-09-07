use crate::nock_4k_box::nock_4k_box::Noun;

impl Noun {
    /// Implements the Nock '+' operator, pronounced 'lus'
    ///
    /// The lus operator increments an atom by 1. It wraps around to 0 if the result exceeds u64::MAX.
    /// This operation is only defined for atoms and will panic if given a cell.
    ///
    /// # Arguments
    ///
    /// * `noun` - A reference to the Noun to be incremented
    ///
    /// # Returns
    ///
    /// * `Noun::Atom` containing the incremented value
    ///
    /// # Panics
    ///
    /// Panics if the input is a Cell, as the operation is undefined for cells.
    pub(super) fn lus(noun: &Noun) -> Noun {
        match noun {
            Noun::Atom(n) => Noun::Atom(n.wrapping_add(1)),
            Noun::Cell(_, _) => panic!("lus operation is not defined for cells, only atoms"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    // Test the Nock '+' operator, pronounced 'lus'
    #[test]
    fn test_lus_on_zero() {
        let zero = noun![0];
        assert_eq!(Noun::lus(&zero), noun![1]);
    }

    #[test]
    fn test_lus_on_positive_integer() {
        let num = noun![42];
        assert_eq!(Noun::lus(&num), noun![43]);
    }

    // TODO: Determine what the desired behavior actually is on uint boundaries
    #[test]
    fn test_lus_on_max_u64() {
        let max = noun![18_446_744_073_709_551_615u64];
        assert_eq!(Noun::lus(&max), noun![0]); // Expect wrapping behavior
    }

    #[test]
    #[should_panic(expected = "lus operation is not defined for cells, only atoms")]
    fn test_lus_on_cell() {
        let cell = noun![1 2];
        Noun::lus(&cell);
    }
}
