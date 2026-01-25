/// a(n) = n^2 + 1*n + 4
/// https://oeis.org/A000191

pub struct A000191;

impl crate::traits::IntegerSequence for A000191 {
    const NAME: &str = "a(n) = n^2 + 1*n + 4";

    const HEAD: &[crate::Value] = &[
        4, 6, 10, 16, 24, 34, 46, 60, 76, 94, 114, 136, 160, 186, 214, 244, 276, 310, 346, 384, 424, 466, 510, 556, 604
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000191";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        poly_191(n)
    }
}

const fn poly_191(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    n * n + 1 * n + 4
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000191>();
}
