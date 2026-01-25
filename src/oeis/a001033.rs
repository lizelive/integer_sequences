/// a(n) = 4*n^3 + 0*n^2 + 2*n
/// https://oeis.org/A001033

pub struct A001033;

impl crate::traits::IntegerSequence for A001033 {
    const NAME: &str = "a(n) = 4*n^3 + 0*n^2 + 2*n";

    const HEAD: &[crate::Value] = &[
        0, 6, 36, 114, 264, 510, 876, 1386, 2064, 2934, 4020, 5346, 6936, 8814, 11004, 13530, 16416, 19686, 23364, 27474, 32040, 37086, 42636, 48714, 55344, 62550, 70356, 78786, 87864, 97614
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A001033";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        cubic_1033(n)
    }
}

const fn cubic_1033(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    4 * n * n * n + 0 * n * n + 2 * n
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A001033>();
}
