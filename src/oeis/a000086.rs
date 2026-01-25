/// Number of permutations of {1, 2, ..., n} with no 3-term increasing subsequence.
/// https://oeis.org/A000086

pub struct A000086;

impl crate::traits::IntegerSequence for A000086 {
    const NAME: &str = "Number of solutions to x^3 == 1 (mod n)";

    const HEAD: &[crate::Value] = &[
        1, 1, 1, 1, 1, 1, 3, 1, 3, 1, 1, 1, 3, 3, 1, 1, 1, 3, 3, 1, 3, 1, 1, 1, 1, 3, 3, 3, 1, 1,
        3, 1, 1, 1, 3, 3, 3, 3, 3, 1, 1, 3, 3, 1, 3, 1, 1, 1, 9, 1, 1, 3, 1, 3, 1, 3, 3, 1, 1, 1,
        3, 3, 9, 1, 3, 1, 3, 1, 1, 3, 3, 3, 3, 3, 1, 3, 3, 3, 3, 1, 9, 1, 1, 3, 1, 3, 3, 1, 1, 3,
    ];

    const OFFSET: crate::Index = 1;

    const SOURCE: &str = "https://oeis.org/A000086";

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
    crate::tester::test_sequance_formula_matchces_head::<A000086>();
}
