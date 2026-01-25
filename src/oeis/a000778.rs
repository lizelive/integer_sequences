/// a(n) = 4*n^2 + 4*n + 0
/// https://oeis.org/A000778

pub struct A000778;

impl crate::traits::IntegerSequence for A000778 {
    const NAME: &str = "a(n) = 4*n^2 + 4*n + 0";

    const HEAD: &[crate::Value] = &[
        0, 8, 24, 48, 80, 120, 168, 224, 288, 360, 440, 528, 624, 728, 840, 960, 1088, 1224, 1368, 1520, 1680, 1848, 2024, 2208, 2400, 2600, 2808, 3024, 3248, 3480
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000778";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        quad_778(n)
    }
}

const fn quad_778(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    4 * n * n + 4 * n + 0
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000778>();
}
