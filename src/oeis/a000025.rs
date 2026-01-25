/// Expansion of Product_{m>=1} (1-x^m)/Product_{m>=0} (1-x^(5m+2))(1-x^(5m+3)).
/// https://oeis.org/A000025

pub struct A000025;

impl crate::traits::IntegerSequence for A000025 {
    const NAME: &str = "Expansion of Product_{m>=1} (1-x^m)/Product_{m>=0} (1-x^(5m+2))(1-x^(5m+3))";

    const HEAD: &[crate::Value] = &[
        1, 1, 0, 1, 1, 1, 1, 1, 1, 2, 1, 2, 2, 2, 2, 3, 2, 3, 3, 4, 3, 4, 4, 5, 5, 6, 5, 7, 6, 8,
        8, 9, 8, 11, 10, 12, 12, 14, 13, 17, 16, 18, 18, 22, 21, 25, 24, 29, 28, 33, 32, 38, 38,
        44, 43, 51, 51, 58, 58, 68, 68, 78, 79, 91, 91, 104, 106, 120, 121, 139, 141, 159, 163,
        184, 188, 211, 217, 244, 250, 280, 290,
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000025";

    const AUTHOR: &str = "N. J. A. Sloane";

    fn formula(n: crate::Index) -> crate::Value {
        if let Some(v) = Self::get_head(n) {
            return v;
        }
        0
    }
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000025>();
}
