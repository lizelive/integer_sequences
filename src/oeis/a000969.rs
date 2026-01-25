/// a(n) = 10*n^2 + 6
/// https://oeis.org/A000969

pub struct A000969;

impl crate::traits::IntegerSequence for A000969 {
    const NAME: &str = "a(n) = 10*n^2 + 6";

    const HEAD: &[crate::Value] = &[
        6, 16, 46, 96, 166, 256, 366, 496, 646, 816, 1006, 1216, 1446, 1696, 1966, 2256, 2566, 2896, 3246, 3616, 4006, 4416, 4846, 5296, 5766, 6256, 6766, 7296, 7846, 8416
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000969";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        sq_969(n)
    }
}

const fn sq_969(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    10 * n * n + 6 - 0 * n
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000969>();
}
