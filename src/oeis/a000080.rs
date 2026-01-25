/// Number of distinct values obtained by 2^2^...^2 (n 2's and parentheses inserted in all possible ways).
/// https://oeis.org/A000080

pub struct A000080;

impl crate::traits::IntegerSequence for A000080 {
    const NAME: &str = "Number of nonisomorphic minimal T0-topologies on n points";

    const HEAD: &[crate::Value] = &[
        1, 1, 1, 2, 5, 9, 20, 56, 216, 1070, 6649, 50104, 442647, 4481042, 51018825, 645309938,
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000080";

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
    crate::tester::test_sequance_formula_matchces_head::<A000080>();
}
