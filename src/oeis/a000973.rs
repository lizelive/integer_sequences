/// a(n) = 4*n^2 + 7
/// https://oeis.org/A000973

pub struct A000973;

impl crate::traits::IntegerSequence for A000973 {
    const NAME: &str = "a(n) = 4*n^2 + 7";

    const HEAD: &[crate::Value] = &[
        7, 11, 23, 43, 71, 107, 151, 203, 263, 331, 407, 491, 583, 683, 791, 907, 1031, 1163, 1303, 1451, 1607, 1771, 1943, 2123, 2311, 2507, 2711, 2923, 3143, 3371
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000973";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        sq_973(n)
    }
}

const fn sq_973(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    4 * n * n + 7 - 0 * n
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000973>();
}
