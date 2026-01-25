/// a(n) = 4*n^2 + 2
/// https://oeis.org/A000923

pub struct A000923;

impl crate::traits::IntegerSequence for A000923 {
    const NAME: &str = "a(n) = 4*n^2 + 2";

    const HEAD: &[crate::Value] = &[
        2, 6, 18, 38, 66, 102, 146, 198, 258, 326, 402, 486, 578, 678, 786, 902, 1026, 1158, 1298, 1446, 1602, 1766, 1938, 2118, 2306, 2502, 2706, 2918, 3138, 3366
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000923";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        sq_923(n)
    }
}

const fn sq_923(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    4 * n * n + 2 - 0 * n
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000923>();
}
