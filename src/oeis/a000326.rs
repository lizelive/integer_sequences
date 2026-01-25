/// Pentagonal numbers: a(n) = n*(3n-1)/2.
/// https://oeis.org/A000326

pub struct A000326;

impl crate::traits::IntegerSequence for A000326 {
    const NAME: &str = "Pentagonal numbers: n*(3n-1)/2";

    const HEAD: &[crate::Value] = &[
        0, 1, 5, 12, 22, 35, 51, 70, 92, 117, 145, 176, 210, 247, 287, 330, 376, 425, 477, 532,
        590, 651, 715, 782, 852, 925, 1001, 1080, 1162, 1247, 1335, 1426, 1520, 1617, 1717, 1820,
        1926, 2035, 2147, 2262, 2380, 2501, 2625, 2752, 2882, 3015, 3151, 3290, 3432, 3577, 3725,
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000326";

    const AUTHOR: &str = "N. J. A. Sloane";

    fn formula(n: crate::Index) -> crate::Value {
        n * (3 * n - 1) / 2
    }
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000326>();
}
