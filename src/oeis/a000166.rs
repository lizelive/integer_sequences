/// a(n) = n^2 + 6*n + 1
/// https://oeis.org/A000166

pub struct A000166;

impl crate::traits::IntegerSequence for A000166 {
    const NAME: &str = "a(n) = n^2 + 6*n + 1";

    const HEAD: &[crate::Value] = &[
        1, 8, 17, 28, 41, 56, 73, 92, 113, 136, 161, 188, 217, 248, 281, 316, 353, 392, 433, 476, 521, 568, 617, 668, 721
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000166";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        poly_166(n)
    }
}

const fn poly_166(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    n * n + 6 * n + 1
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000166>();
}
