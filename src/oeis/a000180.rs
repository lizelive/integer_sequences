/// a(n) = n^2 + 0*n + 3
/// https://oeis.org/A000180

pub struct A000180;

impl crate::traits::IntegerSequence for A000180 {
    const NAME: &str = "a(n) = n^2 + 0*n + 3";

    const HEAD: &[crate::Value] = &[
        3, 4, 7, 12, 19, 28, 39, 52, 67, 84, 103, 124, 147, 172, 199, 228, 259, 292, 327, 364, 403, 444, 487, 532, 579
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000180";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        poly_180(n)
    }
}

const fn poly_180(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    n * n + 0 * n + 3
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000180>();
}
