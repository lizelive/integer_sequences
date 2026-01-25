/// a(n) = 10*n^2 + 1
/// https://oeis.org/A000919

pub struct A000919;

impl crate::traits::IntegerSequence for A000919 {
    const NAME: &str = "a(n) = 10*n^2 + 1";

    const HEAD: &[crate::Value] = &[
        1, 11, 41, 91, 161, 251, 361, 491, 641, 811, 1001, 1211, 1441, 1691, 1961, 2251, 2561, 2891, 3241, 3611, 4001, 4411, 4841, 5291, 5761, 6251, 6761, 7291, 7841, 8411
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000919";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        sq_919(n)
    }
}

const fn sq_919(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    10 * n * n + 1 - 0 * n
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000919>();
}
