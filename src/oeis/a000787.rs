/// a(n) = 4*n^2 + 3*n + 2
/// https://oeis.org/A000787

pub struct A000787;

impl crate::traits::IntegerSequence for A000787 {
    const NAME: &str = "a(n) = 4*n^2 + 3*n + 2";

    const HEAD: &[crate::Value] = &[
        2, 9, 24, 47, 78, 117, 164, 219, 282, 353, 432, 519, 614, 717, 828, 947, 1074, 1209, 1352, 1503, 1662, 1829, 2004, 2187, 2378, 2577, 2784, 2999, 3222, 3453
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000787";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        quad_787(n)
    }
}

const fn quad_787(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    4 * n * n + 3 * n + 2
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000787>();
}
