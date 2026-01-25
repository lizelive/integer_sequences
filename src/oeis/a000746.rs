/// a(n) = 2*n^2 + 2*n + 4
/// https://oeis.org/A000746

pub struct A000746;

impl crate::traits::IntegerSequence for A000746 {
    const NAME: &str = "a(n) = 2*n^2 + 2*n + 4";

    const HEAD: &[crate::Value] = &[
        4, 8, 16, 28, 44, 64, 88, 116, 148, 184, 224, 268, 316, 368, 424, 484, 548, 616, 688, 764, 844, 928, 1016, 1108, 1204, 1304, 1408, 1516, 1628, 1744
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000746";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        quad_746(n)
    }
}

const fn quad_746(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    2 * n * n + 2 * n + 4
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000746>();
}
