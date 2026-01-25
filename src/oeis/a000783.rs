/// a(n) = 4*n^2 + 4*n + 1
/// https://oeis.org/A000783

pub struct A000783;

impl crate::traits::IntegerSequence for A000783 {
    const NAME: &str = "a(n) = 4*n^2 + 4*n + 1";

    const HEAD: &[crate::Value] = &[
        1, 9, 25, 49, 81, 121, 169, 225, 289, 361, 441, 529, 625, 729, 841, 961, 1089, 1225, 1369, 1521, 1681, 1849, 2025, 2209, 2401, 2601, 2809, 3025, 3249, 3481
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000783";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        quad_783(n)
    }
}

const fn quad_783(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    4 * n * n + 4 * n + 1
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000783>();
}
