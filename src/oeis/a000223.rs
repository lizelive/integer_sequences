/// a(n) = n^3 + 3*n^2 + 4*n + 0
/// https://oeis.org/A000223

pub struct A000223;

impl crate::traits::IntegerSequence for A000223 {
    const NAME: &str = "a(n) = n^3 + 3*n^2 + 4*n + 0";

    const HEAD: &[crate::Value] = &[
        0, 8, 28, 66, 128, 220, 348, 518, 736, 1008, 1340, 1738, 2208, 2756, 3388, 4110, 4928, 5848, 6876, 8018, 9280, 10668, 12188, 13846, 15648
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000223";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        poly_223(n)
    }
}

const fn poly_223(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    n * n * n + 3 * n * n + 4 * n + 0
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000223>();
}
