/// a(n) = n^3 + 4*n^2 + 0*n + 1
/// https://oeis.org/A000229

pub struct A000229;

impl crate::traits::IntegerSequence for A000229 {
    const NAME: &str = "a(n) = n^3 + 4*n^2 + 0*n + 1";

    const HEAD: &[crate::Value] = &[
        1, 6, 25, 64, 129, 226, 361, 540, 769, 1054, 1401, 1816, 2305, 2874, 3529, 4276, 5121, 6070, 7129, 8304, 9601, 11026, 12585, 14284, 16129
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000229";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        poly_229(n)
    }
}

const fn poly_229(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    n * n * n + 4 * n * n + 0 * n + 1
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000229>();
}
