/// a(n) = n^3 + 0*n^2 + 4*n + 0
/// https://oeis.org/A000220

pub struct A000220;

impl crate::traits::IntegerSequence for A000220 {
    const NAME: &str = "a(n) = n^3 + 0*n^2 + 4*n + 0";

    const HEAD: &[crate::Value] = &[
        0, 5, 16, 39, 80, 145, 240, 371, 544, 765, 1040, 1375, 1776, 2249, 2800, 3435, 4160, 4981, 5904, 6935, 8080, 9345, 10736, 12259, 13920
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000220";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        poly_220(n)
    }
}

const fn poly_220(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    n * n * n + 0 * n * n + 4 * n + 0
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000220>();
}
