/// a(n) = n^3 + 9*n + 5
/// https://oeis.org/A000559

pub struct A000559;

impl crate::traits::IntegerSequence for A000559 {
    const NAME: &str = "a(n) = n^3 + 9*n + 5";

    const HEAD: &[crate::Value] = &[
        5, 15, 31, 59, 105, 175, 275, 411, 589, 815, 1095, 1435, 1841, 2319, 2875, 3515, 4245, 5071, 5999, 7035, 8185, 9455, 10851, 12379, 14045, 15855, 17815, 19931, 22209, 24655
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000559";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        poly_559(n)
    }
}

const fn poly_559(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    n * n * n + 9 * n + 5
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000559>();
}
