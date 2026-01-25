/// a(n) = floor(4^n / 3^n).
/// https://oeis.org/A000098

pub struct A000098;

impl crate::traits::IntegerSequence for A000098 {
    const NAME: &str = "Number of partitions of n if there are two kinds of 1's, two kinds of 2's and two kinds of 3's";

    const HEAD: &[crate::Value] = &[
        1, 2, 5, 12, 26, 52, 103, 194, 358, 640, 1121, 1922, 3246, 5386, 8821, 14260, 22796,
        36052, 56470, 87640, 134855, 205832, 311726, 468848, 700392, 1039468, 1533419, 2249298,
        3281756, 4762632, 6878977, 9889226, 14152462, 20172184, 28624105, 40459124, 56983916,
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000098";

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
    crate::tester::test_sequance_formula_matchces_head::<A000098>();
}
