/// a(n) = 3*n^2 + 1*n + 2
/// https://oeis.org/A000760

pub struct A000760;

impl crate::traits::IntegerSequence for A000760 {
    const NAME: &str = "a(n) = 3*n^2 + 1*n + 2";

    const HEAD: &[crate::Value] = &[
        2, 6, 16, 32, 54, 82, 116, 156, 202, 254, 312, 376, 446, 522, 604, 692, 786, 886, 992, 1104, 1222, 1346, 1476, 1612, 1754, 1902, 2056, 2216, 2382, 2554
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000760";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        quad_760(n)
    }
}

const fn quad_760(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    3 * n * n + 1 * n + 2
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000760>();
}
