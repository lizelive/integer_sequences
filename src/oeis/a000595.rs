/// a(n) = n^3 + 5*n + 9
/// https://oeis.org/A000595

pub struct A000595;

impl crate::traits::IntegerSequence for A000595 {
    const NAME: &str = "a(n) = n^3 + 5*n + 9";

    const HEAD: &[crate::Value] = &[
        9, 15, 27, 51, 93, 159, 255, 387, 561, 783, 1059, 1395, 1797, 2271, 2823, 3459, 4185, 5007, 5931, 6963, 8109, 9375, 10767, 12291, 13953, 15759, 17715, 19827, 22101, 24543
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000595";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        poly_595(n)
    }
}

const fn poly_595(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    n * n * n + 5 * n + 9
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000595>();
}
