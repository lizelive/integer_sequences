/// a(n) = n^2 + 4*n + 0
/// https://oeis.org/A000154

pub struct A000154;

impl crate::traits::IntegerSequence for A000154 {
    const NAME: &str = "a(n) = n^2 + 4*n + 0";

    const HEAD: &[crate::Value] = &[
        0, 5, 12, 21, 32, 45, 60, 77, 96, 117, 140, 165, 192, 221, 252, 285, 320, 357, 396, 437, 480, 525, 572, 621, 672
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000154";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        poly_154(n)
    }
}

const fn poly_154(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    n * n + 4 * n + 0
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000154>();
}
