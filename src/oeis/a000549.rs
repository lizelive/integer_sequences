/// a(n) = n^3 + 9*n + 4
/// https://oeis.org/A000549

pub struct A000549;

impl crate::traits::IntegerSequence for A000549 {
    const NAME: &str = "a(n) = n^3 + 9*n + 4";

    const HEAD: &[crate::Value] = &[
        4, 14, 30, 58, 104, 174, 274, 410, 588, 814, 1094, 1434, 1840, 2318, 2874, 3514, 4244, 5070, 5998, 7034, 8184, 9454, 10850, 12378, 14044, 15854, 17814, 19930, 22208, 24654
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000549";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        poly_549(n)
    }
}

const fn poly_549(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    n * n * n + 9 * n + 4
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000549>();
}
