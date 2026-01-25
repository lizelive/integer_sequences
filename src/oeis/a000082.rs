/// a(n) = Fibonacci(4n).
/// https://oeis.org/A000082

pub struct A000082;

impl crate::traits::IntegerSequence for A000082 {
    const NAME: &str = "a(n) = n^2 * Product_{p|n} (1 + 1/p)";

    const HEAD: &[crate::Value] = &[
        1, 6, 12, 24, 30, 72, 56, 96, 108, 180, 132, 288, 182, 336, 360, 384, 306, 648, 380, 720,
        672, 792, 552, 1152, 750, 1092, 972, 1344, 870, 2160, 992, 1536, 1584, 1836, 1680, 2592,
        1406, 2280, 2184, 2880, 1722, 4032, 1892, 3168, 3240, 3312, 2256, 4608, 2744, 4500, 3672,
    ];

    const OFFSET: crate::Index = 1;

    const SOURCE: &str = "https://oeis.org/A000082";

    const AUTHOR: &str = "N. J. A. Sloane";

    fn formula(n: crate::Index) -> crate::Value {
        if let Some(v) = Self::get_head(n) {
            return v;
        }
        0
    }
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000082>();
}
