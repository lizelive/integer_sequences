/// a(n) = n^3 + 3*n^2 + 4*n + 1
/// https://oeis.org/A000248

pub struct A000248;

impl crate::traits::IntegerSequence for A000248 {
    const NAME: &str = "a(n) = n^3 + 3*n^2 + 4*n + 1";

    const HEAD: &[crate::Value] = &[
        1, 9, 29, 67, 129, 221, 349, 519, 737, 1009, 1341, 1739, 2209, 2757, 3389, 4111, 4929, 5849, 6877, 8019, 9281, 10669, 12189, 13847, 15649
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000248";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        poly_248(n)
    }
}

const fn poly_248(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    n * n * n + 3 * n * n + 4 * n + 1
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000248>();
}
