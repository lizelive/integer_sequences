/// a(n) = 3*n^2 + 2*n + 1
/// https://oeis.org/A000756

pub struct A000756;

impl crate::traits::IntegerSequence for A000756 {
    const NAME: &str = "a(n) = 3*n^2 + 2*n + 1";

    const HEAD: &[crate::Value] = &[
        1, 6, 17, 34, 57, 86, 121, 162, 209, 262, 321, 386, 457, 534, 617, 706, 801, 902, 1009, 1122, 1241, 1366, 1497, 1634, 1777, 1926, 2081, 2242, 2409, 2582
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000756";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        quad_756(n)
    }
}

const fn quad_756(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    3 * n * n + 2 * n + 1
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000756>();
}
