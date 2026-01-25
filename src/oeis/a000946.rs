/// a(n) = 7*n^2 + 4
/// https://oeis.org/A000946

pub struct A000946;

impl crate::traits::IntegerSequence for A000946 {
    const NAME: &str = "a(n) = 7*n^2 + 4";

    const HEAD: &[crate::Value] = &[
        4, 11, 32, 67, 116, 179, 256, 347, 452, 571, 704, 851, 1012, 1187, 1376, 1579, 1796, 2027, 2272, 2531, 2804, 3091, 3392, 3707, 4036, 4379, 4736, 5107, 5492, 5891
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000946";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        sq_946(n)
    }
}

const fn sq_946(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    7 * n * n + 4 - 0 * n
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000946>();
}
