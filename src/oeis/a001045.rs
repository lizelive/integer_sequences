/// a(n) = 4*n^3 + 2*n^2 + 2*n
/// https://oeis.org/A001045

pub struct A001045;

impl crate::traits::IntegerSequence for A001045 {
    const NAME: &str = "a(n) = 4*n^3 + 2*n^2 + 2*n";

    const HEAD: &[crate::Value] = &[
        0, 8, 44, 132, 296, 560, 948, 1484, 2192, 3096, 4220, 5588, 7224, 9152, 11396, 13980, 16928, 20264, 24012, 28196, 32840, 37968, 43604, 49772, 56496, 63800, 71708, 80244, 89432, 99296
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A001045";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        cubic_1045(n)
    }
}

const fn cubic_1045(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    4 * n * n * n + 2 * n * n + 2 * n
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A001045>();
}
