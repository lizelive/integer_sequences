/// a(n) = 5*n^3 + 0*n^2 + 2*n
/// https://oeis.org/A001034

pub struct A001034;

impl crate::traits::IntegerSequence for A001034 {
    const NAME: &str = "a(n) = 5*n^3 + 0*n^2 + 2*n";

    const HEAD: &[crate::Value] = &[
        0, 7, 44, 141, 328, 635, 1092, 1729, 2576, 3663, 5020, 6677, 8664, 11011, 13748, 16905, 20512, 24599, 29196, 34333, 40040, 46347, 53284, 60881, 69168, 78175, 87932, 98469, 109816, 122003
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A001034";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        cubic_1034(n)
    }
}

const fn cubic_1034(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    5 * n * n * n + 0 * n * n + 2 * n
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A001034>();
}
