/// a(n) = n^3 + 1*n + 5
/// https://oeis.org/A000551

pub struct A000551;

impl crate::traits::IntegerSequence for A000551 {
    const NAME: &str = "a(n) = n^3 + 1*n + 5";

    const HEAD: &[crate::Value] = &[
        5, 7, 15, 35, 73, 135, 227, 355, 525, 743, 1015, 1347, 1745, 2215, 2763, 3395, 4117, 4935, 5855, 6883, 8025, 9287, 10675, 12195, 13853, 15655, 17607, 19715, 21985, 24423
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000551";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        poly_551(n)
    }
}

const fn poly_551(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    n * n * n + 1 * n + 5
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000551>();
}
