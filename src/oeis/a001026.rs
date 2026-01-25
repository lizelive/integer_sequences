/// a(n) = 3*n^3 + 4*n^2 + 1*n
/// https://oeis.org/A001026

pub struct A001026;

impl crate::traits::IntegerSequence for A001026 {
    const NAME: &str = "a(n) = 3*n^3 + 4*n^2 + 1*n";

    const HEAD: &[crate::Value] = &[
        0, 8, 42, 120, 260, 480, 798, 1232, 1800, 2520, 3410, 4488, 5772, 7280, 9030, 11040, 13328, 15912, 18810, 22040, 25620, 29568, 33902, 38640, 43800, 49400, 55458, 61992, 69020, 76560
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A001026";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        cubic_1026(n)
    }
}

const fn cubic_1026(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    3 * n * n * n + 4 * n * n + 1 * n
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A001026>();
}
