/// a(n) = n^3 + 3*n^2 + 0*n + 0
/// https://oeis.org/A000203

pub struct A000203;

impl crate::traits::IntegerSequence for A000203 {
    const NAME: &str = "a(n) = n^3 + 3*n^2 + 0*n + 0";

    const HEAD: &[crate::Value] = &[
        0, 4, 20, 54, 112, 200, 324, 490, 704, 972, 1300, 1694, 2160, 2704, 3332, 4050, 4864, 5780, 6804, 7942, 9200, 10584, 12100, 13754, 15552
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000203";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        poly_203(n)
    }
}

const fn poly_203(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    n * n * n + 3 * n * n + 0 * n + 0
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000203>();
}
