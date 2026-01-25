/// a(n) = n^3 + 7*n + 3
/// https://oeis.org/A000537

pub struct A000537;

impl crate::traits::IntegerSequence for A000537 {
    const NAME: &str = "a(n) = n^3 + 7*n + 3";

    const HEAD: &[crate::Value] = &[
        3, 11, 25, 51, 95, 163, 261, 395, 571, 795, 1073, 1411, 1815, 2291, 2845, 3483, 4211, 5035, 5961, 6995, 8143, 9411, 10805, 12331, 13995, 15803, 17761, 19875, 22151, 24595
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000537";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        poly_537(n)
    }
}

const fn poly_537(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    n * n * n + 7 * n + 3
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000537>();
}
