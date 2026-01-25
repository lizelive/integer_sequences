/// a(n) = n^3 + 2*n^2 + 0*n + 0
/// https://oeis.org/A000202

pub struct A000202;

impl crate::traits::IntegerSequence for A000202 {
    const NAME: &str = "a(n) = n^3 + 2*n^2 + 0*n + 0";

    const HEAD: &[crate::Value] = &[
        0, 3, 16, 45, 96, 175, 288, 441, 640, 891, 1200, 1573, 2016, 2535, 3136, 3825, 4608, 5491, 6480, 7581, 8800, 10143, 11616, 13225, 14976
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000202";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        poly_202(n)
    }
}

const fn poly_202(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    n * n * n + 2 * n * n + 0 * n + 0
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000202>();
}
