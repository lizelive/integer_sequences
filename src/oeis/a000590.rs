/// a(n) = n^3 + 0*n + 9
/// https://oeis.org/A000590

pub struct A000590;

impl crate::traits::IntegerSequence for A000590 {
    const NAME: &str = "a(n) = n^3 + 0*n + 9";

    const HEAD: &[crate::Value] = &[
        9, 10, 17, 36, 73, 134, 225, 352, 521, 738, 1009, 1340, 1737, 2206, 2753, 3384, 4105, 4922, 5841, 6868, 8009, 9270, 10657, 12176, 13833, 15634, 17585, 19692, 21961, 24398
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000590";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        poly_590(n)
    }
}

const fn poly_590(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    n * n * n + 0 * n + 9
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000590>();
}
