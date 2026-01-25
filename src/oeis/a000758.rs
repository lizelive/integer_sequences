/// a(n) = 3*n^2 + 4*n + 1
/// https://oeis.org/A000758

pub struct A000758;

impl crate::traits::IntegerSequence for A000758 {
    const NAME: &str = "a(n) = 3*n^2 + 4*n + 1";

    const HEAD: &[crate::Value] = &[
        1, 8, 21, 40, 65, 96, 133, 176, 225, 280, 341, 408, 481, 560, 645, 736, 833, 936, 1045, 1160, 1281, 1408, 1541, 1680, 1825, 1976, 2133, 2296, 2465, 2640
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000758";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        quad_758(n)
    }
}

const fn quad_758(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    3 * n * n + 4 * n + 1
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000758>();
}
