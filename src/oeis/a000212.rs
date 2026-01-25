/// a(n) = n^3 + 2*n^2 + 2*n + 0
/// https://oeis.org/A000212

pub struct A000212;

impl crate::traits::IntegerSequence for A000212 {
    const NAME: &str = "a(n) = n^3 + 2*n^2 + 2*n + 0";

    const HEAD: &[crate::Value] = &[
        0, 5, 20, 51, 104, 185, 300, 455, 656, 909, 1220, 1595, 2040, 2561, 3164, 3855, 4640, 5525, 6516, 7619, 8840, 10185, 11660, 13271, 15024
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000212";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        poly_212(n)
    }
}

const fn poly_212(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    n * n * n + 2 * n * n + 2 * n + 0
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000212>();
}
