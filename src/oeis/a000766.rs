/// a(n) = 3*n^2 + 2*n + 3
/// https://oeis.org/A000766

pub struct A000766;

impl crate::traits::IntegerSequence for A000766 {
    const NAME: &str = "a(n) = 3*n^2 + 2*n + 3";

    const HEAD: &[crate::Value] = &[
        3, 8, 19, 36, 59, 88, 123, 164, 211, 264, 323, 388, 459, 536, 619, 708, 803, 904, 1011, 1124, 1243, 1368, 1499, 1636, 1779, 1928, 2083, 2244, 2411, 2584
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000766";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        quad_766(n)
    }
}

const fn quad_766(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    3 * n * n + 2 * n + 3
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000766>();
}
