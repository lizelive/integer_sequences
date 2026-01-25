/// a(n) = 5*n^3 + 3*n^2 + 2*n
/// https://oeis.org/A001052

pub struct A001052;

impl crate::traits::IntegerSequence for A001052 {
    const NAME: &str = "a(n) = 5*n^3 + 3*n^2 + 2*n";

    const HEAD: &[crate::Value] = &[
        0, 10, 56, 168, 376, 710, 1200, 1876, 2768, 3906, 5320, 7040, 9096, 11518, 14336, 17580, 21280, 25466, 30168, 35416, 41240, 47670, 54736, 62468, 70896, 80050, 89960, 100656, 112168, 124526
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A001052";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        cubic_1052(n)
    }
}

const fn cubic_1052(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    5 * n * n * n + 3 * n * n + 2 * n
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A001052>();
}
