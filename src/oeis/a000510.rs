/// a(n) = n^3 + 0*n + 1
/// https://oeis.org/A000510

pub struct A000510;

impl crate::traits::IntegerSequence for A000510 {
    const NAME: &str = "a(n) = n^3 + 0*n + 1";

    const HEAD: &[crate::Value] = &[
        1, 2, 9, 28, 65, 126, 217, 344, 513, 730, 1001, 1332, 1729, 2198, 2745, 3376, 4097, 4914, 5833, 6860, 8001, 9262, 10649, 12168, 13825, 15626, 17577, 19684, 21953, 24390
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000510";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        poly_510(n)
    }
}

const fn poly_510(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    n * n * n + 0 * n + 1
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000510>();
}
