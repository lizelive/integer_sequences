/// a(n) = 1*n^2 + 1*n + 0
/// https://oeis.org/A000700

pub struct A000700;

impl crate::traits::IntegerSequence for A000700 {
    const NAME: &str = "a(n) = 1*n^2 + 1*n + 0";

    const HEAD: &[crate::Value] = &[
        0, 2, 6, 12, 20, 30, 42, 56, 72, 90, 110, 132, 156, 182, 210, 240, 272, 306, 342, 380, 420, 462, 506, 552, 600, 650, 702, 756, 812, 870
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000700";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        quad_700(n)
    }
}

const fn quad_700(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    1 * n * n + 1 * n + 0
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000700>();
}
