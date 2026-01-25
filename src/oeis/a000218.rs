/// a(n) = n^3 + 3*n^2 + 3*n + 0
/// https://oeis.org/A000218

pub struct A000218;

impl crate::traits::IntegerSequence for A000218 {
    const NAME: &str = "a(n) = n^3 + 3*n^2 + 3*n + 0";

    const HEAD: &[crate::Value] = &[
        0, 7, 26, 63, 124, 215, 342, 511, 728, 999, 1330, 1727, 2196, 2743, 3374, 4095, 4912, 5831, 6858, 7999, 9260, 10647, 12166, 13823, 15624
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000218";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        poly_218(n)
    }
}

const fn poly_218(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    n * n * n + 3 * n * n + 3 * n + 0
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000218>();
}
