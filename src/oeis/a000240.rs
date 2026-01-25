/// a(n) = n^3 + 0*n^2 + 3*n + 1
/// https://oeis.org/A000240

pub struct A000240;

impl crate::traits::IntegerSequence for A000240 {
    const NAME: &str = "a(n) = n^3 + 0*n^2 + 3*n + 1";

    const HEAD: &[crate::Value] = &[
        1, 5, 15, 37, 77, 141, 235, 365, 537, 757, 1031, 1365, 1765, 2237, 2787, 3421, 4145, 4965, 5887, 6917, 8061, 9325, 10715, 12237, 13897
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000240";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        poly_240(n)
    }
}

const fn poly_240(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    n * n * n + 0 * n * n + 3 * n + 1
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000240>();
}
