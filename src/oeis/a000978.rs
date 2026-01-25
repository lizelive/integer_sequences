/// a(n) = 9*n^2 + 7
/// https://oeis.org/A000978

pub struct A000978;

impl crate::traits::IntegerSequence for A000978 {
    const NAME: &str = "a(n) = 9*n^2 + 7";

    const HEAD: &[crate::Value] = &[
        7, 16, 43, 88, 151, 232, 331, 448, 583, 736, 907, 1096, 1303, 1528, 1771, 2032, 2311, 2608, 2923, 3256, 3607, 3976, 4363, 4768, 5191, 5632, 6091, 6568, 7063, 7576
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000978";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        sq_978(n)
    }
}

const fn sq_978(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    9 * n * n + 7 - 0 * n
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000978>();
}
