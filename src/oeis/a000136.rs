/// a(n) = 3 * C(n+2,3) = n*(n+1)*(n+2)/2
/// https://oeis.org/A000136

pub struct A000136;

impl crate::traits::IntegerSequence for A000136 {
    const NAME: &str = "Tetrahedral numbers times 3";

    const HEAD: &[crate::Value] = &[
        0, 3, 12, 30, 60, 105, 168, 252, 360, 495, 660, 858, 1092, 1365, 1680, 2040, 2448, 2907, 3420, 3990, 4620, 5313, 6072, 6900, 7800, 8775, 9828, 10962, 12180, 13485
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000136";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        triple_rising_half(n)
    }
}

const fn triple_rising_half(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    n * (n + 1) * (n + 2) / 2
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000136>();
}
