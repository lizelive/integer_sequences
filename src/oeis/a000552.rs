/// a(n) = n^3 + 2*n + 5
/// https://oeis.org/A000552

pub struct A000552;

impl crate::traits::IntegerSequence for A000552 {
    const NAME: &str = "a(n) = n^3 + 2*n + 5";

    const HEAD: &[crate::Value] = &[
        5, 8, 17, 38, 77, 140, 233, 362, 533, 752, 1025, 1358, 1757, 2228, 2777, 3410, 4133, 4952, 5873, 6902, 8045, 9308, 10697, 12218, 13877, 15680, 17633, 19742, 22013, 24452
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000552";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        poly_552(n)
    }
}

const fn poly_552(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    n * n * n + 2 * n + 5
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000552>();
}
