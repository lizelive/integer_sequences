/// a(n) = n^3 + 2*n + 9
/// https://oeis.org/A000592

pub struct A000592;

impl crate::traits::IntegerSequence for A000592 {
    const NAME: &str = "a(n) = n^3 + 2*n + 9";

    const HEAD: &[crate::Value] = &[
        9, 12, 21, 42, 81, 144, 237, 366, 537, 756, 1029, 1362, 1761, 2232, 2781, 3414, 4137, 4956, 5877, 6906, 8049, 9312, 10701, 12222, 13881, 15684, 17637, 19746, 22017, 24456
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000592";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        poly_592(n)
    }
}

const fn poly_592(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    n * n * n + 2 * n + 9
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000592>();
}
