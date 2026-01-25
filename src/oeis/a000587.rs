/// a(n) = n^3 + 7*n + 8
/// https://oeis.org/A000587

pub struct A000587;

impl crate::traits::IntegerSequence for A000587 {
    const NAME: &str = "a(n) = n^3 + 7*n + 8";

    const HEAD: &[crate::Value] = &[
        8, 16, 30, 56, 100, 168, 266, 400, 576, 800, 1078, 1416, 1820, 2296, 2850, 3488, 4216, 5040, 5966, 7000, 8148, 9416, 10810, 12336, 14000, 15808, 17766, 19880, 22156, 24600
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000587";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        poly_587(n)
    }
}

const fn poly_587(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    n * n * n + 7 * n + 8
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000587>();
}
