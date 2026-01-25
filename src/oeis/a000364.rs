/// a(n) = 2*T(n) + 4
/// https://oeis.org/A000364

pub struct A000364;

impl crate::traits::IntegerSequence for A000364 {
    const NAME: &str = "a(n) = 2*T(n) + 4";

    const HEAD: &[crate::Value] = &[
        4, 6, 10, 16, 24, 34, 46, 60, 76, 94, 114, 136, 160, 186, 214, 244, 276, 310, 346, 384, 424, 466, 510, 556, 604
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000364";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        tri_364(n)
    }
}

const fn tri_364(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    2 * n * (n + 1) / 2 + 4
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000364>();
}
