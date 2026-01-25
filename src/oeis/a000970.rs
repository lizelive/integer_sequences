/// a(n) = 1*n^2 + 7
/// https://oeis.org/A000970

pub struct A000970;

impl crate::traits::IntegerSequence for A000970 {
    const NAME: &str = "a(n) = 1*n^2 + 7";

    const HEAD: &[crate::Value] = &[
        7, 8, 11, 16, 23, 32, 43, 56, 71, 88, 107, 128, 151, 176, 203, 232, 263, 296, 331, 368, 407, 448, 491, 536, 583, 632, 683, 736, 791, 848
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000970";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        sq_970(n)
    }
}

const fn sq_970(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    1 * n * n + 7 - 0 * n
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000970>();
}
