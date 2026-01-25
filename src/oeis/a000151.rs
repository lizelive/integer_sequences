/// a(n) = n^2 + 1*n + 0
/// https://oeis.org/A000151

pub struct A000151;

impl crate::traits::IntegerSequence for A000151 {
    const NAME: &str = "a(n) = n^2 + 1*n + 0";

    const HEAD: &[crate::Value] = &[
        0, 2, 6, 12, 20, 30, 42, 56, 72, 90, 110, 132, 156, 182, 210, 240, 272, 306, 342, 380, 420, 462, 506, 552, 600
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000151";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        poly_151(n)
    }
}

const fn poly_151(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    n * n + 1 * n + 0
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000151>();
}
