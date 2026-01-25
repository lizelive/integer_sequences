/// a(n) = n^2 + 8*n + 4
/// https://oeis.org/A000198

pub struct A000198;

impl crate::traits::IntegerSequence for A000198 {
    const NAME: &str = "a(n) = n^2 + 8*n + 4";

    const HEAD: &[crate::Value] = &[
        4, 13, 24, 37, 52, 69, 88, 109, 132, 157, 184, 213, 244, 277, 312, 349, 388, 429, 472, 517, 564, 613, 664, 717, 772
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000198";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        poly_198(n)
    }
}

const fn poly_198(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    n * n + 8 * n + 4
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000198>();
}
