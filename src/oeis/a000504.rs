/// a(n) = n^3 + 4*n + 0
/// https://oeis.org/A000504

pub struct A000504;

impl crate::traits::IntegerSequence for A000504 {
    const NAME: &str = "a(n) = n^3 + 4*n + 0";

    const HEAD: &[crate::Value] = &[
        0, 5, 16, 39, 80, 145, 240, 371, 544, 765, 1040, 1375, 1776, 2249, 2800, 3435, 4160, 4981, 5904, 6935, 8080, 9345, 10736, 12259, 13920, 15725, 17680, 19791, 22064, 24505
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000504";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        poly_504(n)
    }
}

const fn poly_504(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    n * n * n + 4 * n + 0
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000504>();
}
