/// Number of partitions of n into parts greater than 1.
/// https://oeis.org/A000056

pub struct A000056;

impl crate::traits::IntegerSequence for A000056 {
    const NAME: &str = "Number of partitions of n into parts greater than 1";

    const HEAD: &[crate::Value] = &[
        1, 0, 1, 1, 2, 2, 4, 4, 7, 8, 12, 14, 21, 24, 34, 41, 55, 66, 88, 105, 137, 165, 210, 253,
        320, 383, 478, 574, 708, 847, 1039, 1238, 1507, 1794, 2167, 2573, 3094, 3664, 4378, 5180,
        6153, 7259, 8591, 10108, 11905, 13980, 16395, 19204, 22453, 26241, 30562, 35651, 41395,
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000056";

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
    crate::tester::test_sequance_formula_matchces_head::<A000056>();
}
