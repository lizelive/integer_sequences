/// a(n) = n^2 + 7*n + 3
/// https://oeis.org/A000187

pub struct A000187;

impl crate::traits::IntegerSequence for A000187 {
    const NAME: &str = "a(n) = n^2 + 7*n + 3";

    const HEAD: &[crate::Value] = &[
        3, 11, 21, 33, 47, 63, 81, 101, 123, 147, 173, 201, 231, 263, 297, 333, 371, 411, 453, 497, 543, 591, 641, 693, 747
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000187";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        poly_187(n)
    }
}

const fn poly_187(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    n * n + 7 * n + 3
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000187>();
}
