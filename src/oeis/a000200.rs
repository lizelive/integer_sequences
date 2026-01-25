/// a(n) = n^3 + 0*n^2 + 0*n + 0
/// https://oeis.org/A000200

pub struct A000200;

impl crate::traits::IntegerSequence for A000200 {
    const NAME: &str = "a(n) = n^3 + 0*n^2 + 0*n + 0";

    const HEAD: &[crate::Value] = &[
        0, 1, 8, 27, 64, 125, 216, 343, 512, 729, 1000, 1331, 1728, 2197, 2744, 3375, 4096, 4913, 5832, 6859, 8000, 9261, 10648, 12167, 13824
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000200";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        poly_200(n)
    }
}

const fn poly_200(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    n * n * n + 0 * n * n + 0 * n + 0
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000200>();
}
