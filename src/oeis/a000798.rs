/// a(n) = 4*n^2 + 4*n + 4
/// https://oeis.org/A000798

pub struct A000798;

impl crate::traits::IntegerSequence for A000798 {
    const NAME: &str = "a(n) = 4*n^2 + 4*n + 4";

    const HEAD: &[crate::Value] = &[
        4, 12, 28, 52, 84, 124, 172, 228, 292, 364, 444, 532, 628, 732, 844, 964, 1092, 1228, 1372, 1524, 1684, 1852, 2028, 2212, 2404, 2604, 2812, 3028, 3252, 3484
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000798";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        quad_798(n)
    }
}

const fn quad_798(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    4 * n * n + 4 * n + 4
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000798>();
}
