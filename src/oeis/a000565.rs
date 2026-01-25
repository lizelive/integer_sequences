/// a(n) = n^3 + 5*n + 6
/// https://oeis.org/A000565

pub struct A000565;

impl crate::traits::IntegerSequence for A000565 {
    const NAME: &str = "a(n) = n^3 + 5*n + 6";

    const HEAD: &[crate::Value] = &[
        6, 12, 24, 48, 90, 156, 252, 384, 558, 780, 1056, 1392, 1794, 2268, 2820, 3456, 4182, 5004, 5928, 6960, 8106, 9372, 10764, 12288, 13950, 15756, 17712, 19824, 22098, 24540
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000565";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        poly_565(n)
    }
}

const fn poly_565(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    n * n * n + 5 * n + 6
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000565>();
}
