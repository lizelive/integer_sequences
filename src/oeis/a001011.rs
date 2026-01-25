/// a(n) = 6*n^3 + 1*n^2 + 1*n
/// https://oeis.org/A001011

pub struct A001011;

impl crate::traits::IntegerSequence for A001011 {
    const NAME: &str = "a(n) = 6*n^3 + 1*n^2 + 1*n";

    const HEAD: &[crate::Value] = &[
        0, 8, 54, 174, 404, 780, 1338, 2114, 3144, 4464, 6110, 8118, 10524, 13364, 16674, 20490, 24848, 29784, 35334, 41534, 48420, 56028, 64394, 73554, 83544, 94400, 106158, 118854, 132524, 147204
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A001011";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        cubic_1011(n)
    }
}

const fn cubic_1011(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    6 * n * n * n + 1 * n * n + 1 * n
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A001011>();
}
