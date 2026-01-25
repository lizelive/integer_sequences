/// a(n) = 4*n^2 + 2*n + 4
/// https://oeis.org/A000796

pub struct A000796;

impl crate::traits::IntegerSequence for A000796 {
    const NAME: &str = "a(n) = 4*n^2 + 2*n + 4";

    const HEAD: &[crate::Value] = &[
        4, 10, 24, 46, 76, 114, 160, 214, 276, 346, 424, 510, 604, 706, 816, 934, 1060, 1194, 1336, 1486, 1644, 1810, 1984, 2166, 2356, 2554, 2760, 2974, 3196, 3426
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000796";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        quad_796(n)
    }
}

const fn quad_796(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    4 * n * n + 2 * n + 4
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000796>();
}
