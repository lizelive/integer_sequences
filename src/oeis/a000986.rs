/// a(n) = 7*n^2 + 8
/// https://oeis.org/A000986

pub struct A000986;

impl crate::traits::IntegerSequence for A000986 {
    const NAME: &str = "a(n) = 7*n^2 + 8";

    const HEAD: &[crate::Value] = &[
        8, 15, 36, 71, 120, 183, 260, 351, 456, 575, 708, 855, 1016, 1191, 1380, 1583, 1800, 2031, 2276, 2535, 2808, 3095, 3396, 3711, 4040, 4383, 4740, 5111, 5496, 5895
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000986";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        sq_986(n)
    }
}

const fn sq_986(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    7 * n * n + 8 - 0 * n
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000986>();
}
