/// a(n) = n^3 + 1*n + 4
/// https://oeis.org/A000541

pub struct A000541;

impl crate::traits::IntegerSequence for A000541 {
    const NAME: &str = "a(n) = n^3 + 1*n + 4";

    const HEAD: &[crate::Value] = &[
        4, 6, 14, 34, 72, 134, 226, 354, 524, 742, 1014, 1346, 1744, 2214, 2762, 3394, 4116, 4934, 5854, 6882, 8024, 9286, 10674, 12194, 13852, 15654, 17606, 19714, 21984, 24422
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000541";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        poly_541(n)
    }
}

const fn poly_541(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    n * n * n + 1 * n + 4
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000541>();
}
