/// Hexagonal numbers: a(n) = n*(2n-1).
/// https://oeis.org/A000384

pub struct A000384;

impl crate::traits::IntegerSequence for A000384 {
    const NAME: &str = "Hexagonal numbers: n*(2n-1)";

    const HEAD: &[crate::Value] = &[
        0, 1, 6, 15, 28, 45, 66, 91, 120, 153, 190, 231, 276, 325, 378, 435, 496, 561, 630, 703,
        780, 861, 946, 1035, 1128, 1225, 1326, 1431, 1540, 1653, 1770, 1891, 2016, 2145, 2278,
        2415, 2556, 2701, 2850, 3003, 3160, 3321, 3486, 3655, 3828, 4005, 4186, 4371, 4560, 4753,
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000384";

    const AUTHOR: &str = "N. J. A. Sloane";

    fn formula(n: crate::Index) -> crate::Value {
        n * (2 * n - 1)
    }
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000384>();
}
