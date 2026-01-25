/// a(n) = n^3 + 3*n^2 + 3*n + 1
/// https://oeis.org/A000243

pub struct A000243;

impl crate::traits::IntegerSequence for A000243 {
    const NAME: &str = "a(n) = n^3 + 3*n^2 + 3*n + 1";

    const HEAD: &[crate::Value] = &[
        1, 8, 27, 64, 125, 216, 343, 512, 729, 1000, 1331, 1728, 2197, 2744, 3375, 4096, 4913, 5832, 6859, 8000, 9261, 10648, 12167, 13824, 15625
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000243";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        poly_243(n)
    }
}

const fn poly_243(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    n * n * n + 3 * n * n + 3 * n + 1
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000243>();
}
