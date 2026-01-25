/// a(n) = 3*n^2 + 2*n + 0
/// https://oeis.org/A000751

pub struct A000751;

impl crate::traits::IntegerSequence for A000751 {
    const NAME: &str = "a(n) = 3*n^2 + 2*n + 0";

    const HEAD: &[crate::Value] = &[
        0, 5, 16, 33, 56, 85, 120, 161, 208, 261, 320, 385, 456, 533, 616, 705, 800, 901, 1008, 1121, 1240, 1365, 1496, 1633, 1776, 1925, 2080, 2241, 2408, 2581
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000751";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        quad_751(n)
    }
}

const fn quad_751(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    3 * n * n + 2 * n + 0
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000751>();
}
