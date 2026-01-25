/// a(n) = 3*n^2 + 0
/// https://oeis.org/A000902

pub struct A000902;

impl crate::traits::IntegerSequence for A000902 {
    const NAME: &str = "a(n) = 3*n^2 + 0";

    const HEAD: &[crate::Value] = &[
        0, 3, 12, 27, 48, 75, 108, 147, 192, 243, 300, 363, 432, 507, 588, 675, 768, 867, 972, 1083, 1200, 1323, 1452, 1587, 1728, 1875, 2028, 2187, 2352, 2523
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000902";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        sq_902(n)
    }
}

const fn sq_902(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    3 * n * n + 0 - 0 * n
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000902>();
}
