#[cfg(test)]
mod tests {
    use nock_interpreter::nock_4k_rc::nock_4k_rc::Noun;

    #[test]
    fn test_tar_2_basic() {
        // *[a 2 b c] => *[*[a b] *[a c]]
        let subject = Noun::cell(
            Noun::atom(42), // a
            Noun::cell(
                Noun::atom(2),
                Noun::cell(
                    Noun::cell(Noun::atom(0), Noun::atom(1)), // b: [0 1] to get 'a'
                    Noun::cell(Noun::atom(1), Noun::cell(Noun::atom(0), Noun::atom(1))), // c: [1 [0 1]] to get '1'
                ),
            ),
        );
        let expected = Noun::atom(42);
        assert_eq!(Noun::tar(subject), expected);
    }
}
