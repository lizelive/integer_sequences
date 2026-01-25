/// a(n) = n^3 + 1*n + 6
/// https://oeis.org/A000561

pub struct A000561;

impl crate::traits::IntegerSequence for A000561 {
    const NAME: &str = "a(n) = n^3 + 1*n + 6";

    const HEAD: &[crate::Value] = &[
        6, 8, 16, 36, 74, 136, 228, 356, 526, 744, 1016, 1348, 1746, 2216, 2764, 3396, 4118, 4936, 5856, 6884, 8026, 9288, 10676, 12196, 13854, 15656, 17608, 19716, 21986, 24424
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000561";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        poly_561(n)
    }
}

const fn poly_561(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    n * n * n + 1 * n + 6
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000561>();
}
