/// a(n) = 3*n^2 + 2*n + 4
/// https://oeis.org/A000771

pub struct A000771;

impl crate::traits::IntegerSequence for A000771 {
    const NAME: &str = "a(n) = 3*n^2 + 2*n + 4";

    const HEAD: &[crate::Value] = &[
        4, 9, 20, 37, 60, 89, 124, 165, 212, 265, 324, 389, 460, 537, 620, 709, 804, 905, 1012, 1125, 1244, 1369, 1500, 1637, 1780, 1929, 2084, 2245, 2412, 2585
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000771";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        quad_771(n)
    }
}

const fn quad_771(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    3 * n * n + 2 * n + 4
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000771>();
}
