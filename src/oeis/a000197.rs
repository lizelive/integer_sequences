/// a(n) = n^2 + 7*n + 4
/// https://oeis.org/A000197

pub struct A000197;

impl crate::traits::IntegerSequence for A000197 {
    const NAME: &str = "a(n) = n^2 + 7*n + 4";

    const HEAD: &[crate::Value] = &[
        4, 12, 22, 34, 48, 64, 82, 102, 124, 148, 174, 202, 232, 264, 298, 334, 372, 412, 454, 498, 544, 592, 642, 694, 748
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000197";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        poly_197(n)
    }
}

const fn poly_197(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    n * n + 7 * n + 4
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000197>();
}
