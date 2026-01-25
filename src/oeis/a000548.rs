/// a(n) = n^3 + 8*n + 4
/// https://oeis.org/A000548

pub struct A000548;

impl crate::traits::IntegerSequence for A000548 {
    const NAME: &str = "a(n) = n^3 + 8*n + 4";

    const HEAD: &[crate::Value] = &[
        4, 13, 28, 55, 100, 169, 268, 403, 580, 805, 1084, 1423, 1828, 2305, 2860, 3499, 4228, 5053, 5980, 7015, 8164, 9433, 10828, 12355, 14020, 15829, 17788, 19903, 22180, 24625
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000548";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        poly_548(n)
    }
}

const fn poly_548(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    n * n * n + 8 * n + 4
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000548>();
}
