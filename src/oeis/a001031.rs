/// a(n) = 2*n^3 + 0*n^2 + 2*n
/// https://oeis.org/A001031

pub struct A001031;

impl crate::traits::IntegerSequence for A001031 {
    const NAME: &str = "a(n) = 2*n^3 + 0*n^2 + 2*n";

    const HEAD: &[crate::Value] = &[
        0, 4, 20, 60, 136, 260, 444, 700, 1040, 1476, 2020, 2684, 3480, 4420, 5516, 6780, 8224, 9860, 11700, 13756, 16040, 18564, 21340, 24380, 27696, 31300, 35204, 39420, 43960, 48836
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A001031";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        cubic_1031(n)
    }
}

const fn cubic_1031(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    2 * n * n * n + 0 * n * n + 2 * n
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A001031>();
}
