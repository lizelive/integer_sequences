/// a(n) = 3*n^2 + 4*n + 4
/// https://oeis.org/A000773

pub struct A000773;

impl crate::traits::IntegerSequence for A000773 {
    const NAME: &str = "a(n) = 3*n^2 + 4*n + 4";

    const HEAD: &[crate::Value] = &[
        4, 11, 24, 43, 68, 99, 136, 179, 228, 283, 344, 411, 484, 563, 648, 739, 836, 939, 1048, 1163, 1284, 1411, 1544, 1683, 1828, 1979, 2136, 2299, 2468, 2643
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000773";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        quad_773(n)
    }
}

const fn quad_773(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    3 * n * n + 4 * n + 4
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000773>();
}
