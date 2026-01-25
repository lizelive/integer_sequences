/// a(n) = 8*n^2 + 6
/// https://oeis.org/A000967

pub struct A000967;

impl crate::traits::IntegerSequence for A000967 {
    const NAME: &str = "a(n) = 8*n^2 + 6";

    const HEAD: &[crate::Value] = &[
        6, 14, 38, 78, 134, 206, 294, 398, 518, 654, 806, 974, 1158, 1358, 1574, 1806, 2054, 2318, 2598, 2894, 3206, 3534, 3878, 4238, 4614, 5006, 5414, 5838, 6278, 6734
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000967";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        sq_967(n)
    }
}

const fn sq_967(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    8 * n * n + 6 - 0 * n
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000967>();
}
