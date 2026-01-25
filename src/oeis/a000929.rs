/// a(n) = 10*n^2 + 2
/// https://oeis.org/A000929

pub struct A000929;

impl crate::traits::IntegerSequence for A000929 {
    const NAME: &str = "a(n) = 10*n^2 + 2";

    const HEAD: &[crate::Value] = &[
        2, 12, 42, 92, 162, 252, 362, 492, 642, 812, 1002, 1212, 1442, 1692, 1962, 2252, 2562, 2892, 3242, 3612, 4002, 4412, 4842, 5292, 5762, 6252, 6762, 7292, 7842, 8412
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000929";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        sq_929(n)
    }
}

const fn sq_929(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    10 * n * n + 2 - 0 * n
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000929>();
}
