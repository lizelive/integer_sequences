/// a(n) = n^3 + 4*n + 6
/// https://oeis.org/A000564

pub struct A000564;

impl crate::traits::IntegerSequence for A000564 {
    const NAME: &str = "a(n) = n^3 + 4*n + 6";

    const HEAD: &[crate::Value] = &[
        6, 11, 22, 45, 86, 151, 246, 377, 550, 771, 1046, 1381, 1782, 2255, 2806, 3441, 4166, 4987, 5910, 6941, 8086, 9351, 10742, 12265, 13926, 15731, 17686, 19797, 22070, 24511
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000564";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        poly_564(n)
    }
}

const fn poly_564(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    n * n * n + 4 * n + 6
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000564>();
}
