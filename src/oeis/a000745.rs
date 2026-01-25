/// a(n) = 2*n^2 + 1*n + 4
/// https://oeis.org/A000745

pub struct A000745;

impl crate::traits::IntegerSequence for A000745 {
    const NAME: &str = "a(n) = 2*n^2 + 1*n + 4";

    const HEAD: &[crate::Value] = &[
        4, 7, 14, 25, 40, 59, 82, 109, 140, 175, 214, 257, 304, 355, 410, 469, 532, 599, 670, 745, 824, 907, 994, 1085, 1180, 1279, 1382, 1489, 1600, 1715
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000745";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        quad_745(n)
    }
}

const fn quad_745(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    2 * n * n + 1 * n + 4
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000745>();
}
