/// a(n) = 6*n^2 + 4
/// https://oeis.org/A000945

pub struct A000945;

impl crate::traits::IntegerSequence for A000945 {
    const NAME: &str = "a(n) = 6*n^2 + 4";

    const HEAD: &[crate::Value] = &[
        4, 10, 28, 58, 100, 154, 220, 298, 388, 490, 604, 730, 868, 1018, 1180, 1354, 1540, 1738, 1948, 2170, 2404, 2650, 2908, 3178, 3460, 3754, 4060, 4378, 4708, 5050
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000945";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        sq_945(n)
    }
}

const fn sq_945(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    6 * n * n + 4 - 0 * n
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000945>();
}
