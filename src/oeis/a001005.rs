/// a(n) = 6*n^3 + 0*n^2 + 1*n
/// https://oeis.org/A001005

pub struct A001005;

impl crate::traits::IntegerSequence for A001005 {
    const NAME: &str = "a(n) = 6*n^3 + 0*n^2 + 1*n";

    const HEAD: &[crate::Value] = &[
        0, 7, 50, 165, 388, 755, 1302, 2065, 3080, 4383, 6010, 7997, 10380, 13195, 16478, 20265, 24592, 29495, 35010, 41173, 48020, 55587, 63910, 73025, 82968, 93775, 105482, 118125, 131740, 146363
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A001005";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        cubic_1005(n)
    }
}

const fn cubic_1005(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    6 * n * n * n + 0 * n * n + 1 * n
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A001005>();
}
