/// The simplest sequence of positive numbers: the all 1's sequence.
/// https://oeis.org/A000012

pub struct A000012;

impl crate::traits::IntegerSequence for A000012 {
    const NAME: &str = "The all 1's sequence";

    const HEAD: &[crate::Value] = &[
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000012";

    const AUTHOR: &str = "N. J. A. Sloane";

    fn formula(_n: crate::Index) -> crate::Value {
        1
    }
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000012>();
}
