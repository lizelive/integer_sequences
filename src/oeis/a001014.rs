/// a(n) = 3*n^3 + 2*n^2 + 1*n
/// https://oeis.org/A001014

pub struct A001014;

impl crate::traits::IntegerSequence for A001014 {
    const NAME: &str = "a(n) = 3*n^3 + 2*n^2 + 1*n";

    const HEAD: &[crate::Value] = &[
        0, 6, 34, 102, 228, 430, 726, 1134, 1672, 2358, 3210, 4246, 5484, 6942, 8638, 10590, 12816, 15334, 18162, 21318, 24820, 28686, 32934, 37582, 42648, 48150, 54106, 60534, 67452, 74878
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A001014";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        cubic_1014(n)
    }
}

const fn cubic_1014(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    3 * n * n * n + 2 * n * n + 1 * n
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A001014>();
}
