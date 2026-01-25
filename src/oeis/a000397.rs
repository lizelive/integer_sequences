/// a(n) = 5*T(n) + 7
/// https://oeis.org/A000397

pub struct A000397;

impl crate::traits::IntegerSequence for A000397 {
    const NAME: &str = "a(n) = 5*T(n) + 7";

    const HEAD: &[crate::Value] = &[
        7, 12, 22, 37, 57, 82, 112, 147, 187, 232, 282, 337, 397, 462, 532, 607, 687, 772, 862, 957, 1057, 1162, 1272, 1387, 1507
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000397";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        tri_397(n)
    }
}

const fn tri_397(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    5 * n * (n + 1) / 2 + 7
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000397>();
}
