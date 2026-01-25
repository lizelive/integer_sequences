/// Number of primitive permutation groups of degree n.
/// https://oeis.org/A000019

pub struct A000019;

impl crate::traits::IntegerSequence for A000019 {
    const NAME: &str = "Number of primitive permutation groups of degree n";

    const HEAD: &[crate::Value] = &[
        1, 1, 2, 2, 5, 4, 7, 7, 11, 9, 8, 6, 9, 4, 6, 22, 10, 4, 8, 4, 9, 4, 7, 5, 28, 7, 15, 14,
        8, 4, 12, 7, 4, 2, 6, 22, 11, 4, 2, 8, 10, 4, 7, 4, 7, 6, 6, 4, 40, 9, 2, 6, 7, 4, 6, 5,
    ];

    const OFFSET: crate::Index = 1;

    const SOURCE: &str = "https://oeis.org/A000019";

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
    crate::tester::test_sequance_formula_matchces_head::<A000019>();
}
