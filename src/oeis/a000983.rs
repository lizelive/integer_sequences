/// a(n) = 4*n^2 + 8
/// https://oeis.org/A000983

pub struct A000983;

impl crate::traits::IntegerSequence for A000983 {
    const NAME: &str = "a(n) = 4*n^2 + 8";

    const HEAD: &[crate::Value] = &[
        8, 12, 24, 44, 72, 108, 152, 204, 264, 332, 408, 492, 584, 684, 792, 908, 1032, 1164, 1304, 1452, 1608, 1772, 1944, 2124, 2312, 2508, 2712, 2924, 3144, 3372
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000983";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        sq_983(n)
    }
}

const fn sq_983(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    4 * n * n + 8 - 0 * n
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000983>();
}
