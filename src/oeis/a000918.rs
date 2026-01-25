/// a(n) = 9*n^2 + 1
/// https://oeis.org/A000918

pub struct A000918;

impl crate::traits::IntegerSequence for A000918 {
    const NAME: &str = "a(n) = 9*n^2 + 1";

    const HEAD: &[crate::Value] = &[
        1, 10, 37, 82, 145, 226, 325, 442, 577, 730, 901, 1090, 1297, 1522, 1765, 2026, 2305, 2602, 2917, 3250, 3601, 3970, 4357, 4762, 5185, 5626, 6085, 6562, 7057, 7570
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000918";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        sq_918(n)
    }
}

const fn sq_918(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    9 * n * n + 1 - 0 * n
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000918>();
}
