/// a(n) = 9*n^2 + 0
/// https://oeis.org/A000908

pub struct A000908;

impl crate::traits::IntegerSequence for A000908 {
    const NAME: &str = "a(n) = 9*n^2 + 0";

    const HEAD: &[crate::Value] = &[
        0, 9, 36, 81, 144, 225, 324, 441, 576, 729, 900, 1089, 1296, 1521, 1764, 2025, 2304, 2601, 2916, 3249, 3600, 3969, 4356, 4761, 5184, 5625, 6084, 6561, 7056, 7569
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000908";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        sq_908(n)
    }
}

const fn sq_908(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    9 * n * n + 0 - 0 * n
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000908>();
}
