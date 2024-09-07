#[macro_export]
macro_rules! noun {
    // Rule 1: Match a single literal (base case for atoms)
    [$num:literal] => {
        Noun::atom($num)
    };

    // Rule 2: Match two literals (simple cell)
    [$num1:literal $num2:literal] => {
        Noun::cell(Noun::atom($num1), Noun::atom($num2))
    };

    // Rule 3: Match two nested structures
    [[$($left:tt)+] [$($right:tt)+]] => {
        Noun::cell(noun![$($left)+], noun![$($right)+])
    };

    // Rule 4: Match a nested structure on the left and a single token on the right
    [[$($left:tt)+] $right:tt] => {
        Noun::cell(noun![$($left)+], noun![$right])
    };

    // Rule 5: Match a single token on the left and a nested structure on the right
    [$left:tt [$($right:tt)+]] => {
        Noun::cell(noun![$left], noun![$($right)+])
    };
}
