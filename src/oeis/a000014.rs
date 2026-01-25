/// Number of series-reduced trees with n nodes.
/// https://oeis.org/A000014

pub struct A000014;

impl crate::traits::IntegerSequence for A000014 {
    const NAME: &str = "Number of series-reduced trees with n nodes";

    const HEAD: &[crate::Value] = &[
        0, 1, 1, 0, 1, 1, 2, 3, 6, 11, 23, 46, 98, 207, 451, 983, 2179, 4850, 10905, 24631, 56011,
        127912, 293547, 676157, 1563372, 3626149, 8436379, 19680277, 46026618, 107890609, 253450711,
        596572387,
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000014";

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
    crate::tester::test_sequance_formula_matchces_head::<A000014>();
}
