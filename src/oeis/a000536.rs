/// a(n) = n^3 + 6*n + 3
/// https://oeis.org/A000536

pub struct A000536;

impl crate::traits::IntegerSequence for A000536 {
    const NAME: &str = "a(n) = n^3 + 6*n + 3";

    const HEAD: &[crate::Value] = &[
        3, 10, 23, 48, 91, 158, 255, 388, 563, 786, 1063, 1400, 1803, 2278, 2831, 3468, 4195, 5018, 5943, 6976, 8123, 9390, 10783, 12308, 13971, 15778, 17735, 19848, 22123, 24566
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000536";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        poly_536(n)
    }
}

const fn poly_536(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    n * n * n + 6 * n + 3
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000536>();
}
