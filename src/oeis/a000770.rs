/// a(n) = 3*n^2 + 1*n + 4
/// https://oeis.org/A000770

pub struct A000770;

impl crate::traits::IntegerSequence for A000770 {
    const NAME: &str = "a(n) = 3*n^2 + 1*n + 4";

    const HEAD: &[crate::Value] = &[
        4, 8, 18, 34, 56, 84, 118, 158, 204, 256, 314, 378, 448, 524, 606, 694, 788, 888, 994, 1106, 1224, 1348, 1478, 1614, 1756, 1904, 2058, 2218, 2384, 2556
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000770";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        quad_770(n)
    }
}

const fn quad_770(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    3 * n * n + 1 * n + 4
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000770>();
}
