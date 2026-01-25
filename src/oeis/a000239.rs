/// a(n) = n^3 + 4*n^2 + 2*n + 1
/// https://oeis.org/A000239

pub struct A000239;

impl crate::traits::IntegerSequence for A000239 {
    const NAME: &str = "a(n) = n^3 + 4*n^2 + 2*n + 1";

    const HEAD: &[crate::Value] = &[
        1, 8, 29, 70, 137, 236, 373, 554, 785, 1072, 1421, 1838, 2329, 2900, 3557, 4306, 5153, 6104, 7165, 8342, 9641, 11068, 12629, 14330, 16177
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000239";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        poly_239(n)
    }
}

const fn poly_239(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    n * n * n + 4 * n * n + 2 * n + 1
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000239>();
}
