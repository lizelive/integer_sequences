/// a(n) = n^3 + 5*n + 7
/// https://oeis.org/A000575

pub struct A000575;

impl crate::traits::IntegerSequence for A000575 {
    const NAME: &str = "a(n) = n^3 + 5*n + 7";

    const HEAD: &[crate::Value] = &[
        7, 13, 25, 49, 91, 157, 253, 385, 559, 781, 1057, 1393, 1795, 2269, 2821, 3457, 4183, 5005, 5929, 6961, 8107, 9373, 10765, 12289, 13951, 15757, 17713, 19825, 22099, 24541
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000575";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        poly_575(n)
    }
}

const fn poly_575(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    n * n * n + 5 * n + 7
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000575>();
}
