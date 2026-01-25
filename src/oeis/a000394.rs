/// a(n) = 5*T(n) + 4
/// https://oeis.org/A000394

pub struct A000394;

impl crate::traits::IntegerSequence for A000394 {
    const NAME: &str = "a(n) = 5*T(n) + 4";

    const HEAD: &[crate::Value] = &[
        4, 9, 19, 34, 54, 79, 109, 144, 184, 229, 279, 334, 394, 459, 529, 604, 684, 769, 859, 954, 1054, 1159, 1269, 1384, 1504
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000394";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        tri_394(n)
    }
}

const fn tri_394(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    5 * n * (n + 1) / 2 + 4
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000394>();
}
