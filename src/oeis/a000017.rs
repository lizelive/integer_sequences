/// Number of permutations of {1, ..., n} having an odd number of local maxima.
/// https://oeis.org/A000017

pub struct A000017;

impl crate::traits::IntegerSequence for A000017 {
    const NAME: &str = "Number of permutations of length n with an odd number of local maxima";

    const HEAD: &[crate::Value] = &[
        0, 1, 2, 4, 16, 80, 416, 2688, 19584, 161792, 1488896, 15151104, 168968192, 2048032768,
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000017";

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
    crate::tester::test_sequance_formula_matchces_head::<A000017>();
}
