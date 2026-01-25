/// a(n) = 4*n^2 + 1*n + 0
/// https://oeis.org/A000775

pub struct A000775;

impl crate::traits::IntegerSequence for A000775 {
    const NAME: &str = "a(n) = 4*n^2 + 1*n + 0";

    const HEAD: &[crate::Value] = &[
        0, 5, 18, 39, 68, 105, 150, 203, 264, 333, 410, 495, 588, 689, 798, 915, 1040, 1173, 1314, 1463, 1620, 1785, 1958, 2139, 2328, 2525, 2730, 2943, 3164, 3393
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000775";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        quad_775(n)
    }
}

const fn quad_775(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    4 * n * n + 1 * n + 0
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000775>();
}
