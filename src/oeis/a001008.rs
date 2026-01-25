/// a(n) = 3*n^3 + 1*n^2 + 1*n
/// https://oeis.org/A001008

pub struct A001008;

impl crate::traits::IntegerSequence for A001008 {
    const NAME: &str = "a(n) = 3*n^3 + 1*n^2 + 1*n";

    const HEAD: &[crate::Value] = &[
        0, 5, 30, 93, 212, 405, 690, 1085, 1608, 2277, 3110, 4125, 5340, 6773, 8442, 10365, 12560, 15045, 17838, 20957, 24420, 28245, 32450, 37053, 42072, 47525, 53430, 59805, 66668, 74037
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A001008";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        cubic_1008(n)
    }
}

const fn cubic_1008(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    3 * n * n * n + 1 * n * n + 1 * n
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A001008>();
}
