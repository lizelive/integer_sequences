/// a(n) = 10*n^2 + 5
/// https://oeis.org/A000959

pub struct A000959;

impl crate::traits::IntegerSequence for A000959 {
    const NAME: &str = "a(n) = 10*n^2 + 5";

    const HEAD: &[crate::Value] = &[
        5, 15, 45, 95, 165, 255, 365, 495, 645, 815, 1005, 1215, 1445, 1695, 1965, 2255, 2565, 2895, 3245, 3615, 4005, 4415, 4845, 5295, 5765, 6255, 6765, 7295, 7845, 8415
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000959";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        sq_959(n)
    }
}

const fn sq_959(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    10 * n * n + 5 - 0 * n
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000959>();
}
