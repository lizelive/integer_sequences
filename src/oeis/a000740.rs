/// a(n) = 2*n^2 + 1*n + 3
/// https://oeis.org/A000740

pub struct A000740;

impl crate::traits::IntegerSequence for A000740 {
    const NAME: &str = "a(n) = 2*n^2 + 1*n + 3";

    const HEAD: &[crate::Value] = &[
        3, 6, 13, 24, 39, 58, 81, 108, 139, 174, 213, 256, 303, 354, 409, 468, 531, 598, 669, 744, 823, 906, 993, 1084, 1179, 1278, 1381, 1488, 1599, 1714
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000740";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        quad_740(n)
    }
}

const fn quad_740(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    2 * n * n + 1 * n + 3
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000740>();
}
