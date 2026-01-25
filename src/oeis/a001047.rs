/// a(n) = 6*n^3 + 2*n^2 + 2*n
/// https://oeis.org/A001047

pub struct A001047;

impl crate::traits::IntegerSequence for A001047 {
    const NAME: &str = "a(n) = 6*n^3 + 2*n^2 + 2*n";

    const HEAD: &[crate::Value] = &[
        0, 10, 60, 186, 424, 810, 1380, 2170, 3216, 4554, 6220, 8250, 10680, 13546, 16884, 20730, 25120, 30090, 35676, 41914, 48840, 56490, 64900, 74106, 84144, 95050, 106860, 119610, 133336, 148074
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A001047";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        cubic_1047(n)
    }
}

const fn cubic_1047(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    6 * n * n * n + 2 * n * n + 2 * n
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A001047>();
}
