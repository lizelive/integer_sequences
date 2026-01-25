/// a(n) = n^3 + 2*n + 6
/// https://oeis.org/A000562

pub struct A000562;

impl crate::traits::IntegerSequence for A000562 {
    const NAME: &str = "a(n) = n^3 + 2*n + 6";

    const HEAD: &[crate::Value] = &[
        6, 9, 18, 39, 78, 141, 234, 363, 534, 753, 1026, 1359, 1758, 2229, 2778, 3411, 4134, 4953, 5874, 6903, 8046, 9309, 10698, 12219, 13878, 15681, 17634, 19743, 22014, 24453
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000562";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        poly_562(n)
    }
}

const fn poly_562(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    n * n * n + 2 * n + 6
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000562>();
}
