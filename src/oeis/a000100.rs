/// a(n) = floor(6^n / 5^n).
/// https://oeis.org/A000100

pub struct A000100;

impl crate::traits::IntegerSequence for A000100 {
    const NAME: &str = "Number of partitions of n if there are two kinds of 1's, 2's, 3's, 4's and 5's";

    const HEAD: &[crate::Value] = &[
        1, 2, 5, 12, 28, 62, 135, 284, 586, 1184, 2355, 4616, 8929, 17064, 32260, 60396, 112043,
        206120, 376249, 681592, 1226292, 2191692, 3893165, 6877776, 12087260, 21140604, 36790171,
        63744056, 109952092, 188923240, 323344955, 551375324, 937062830, 1587672820, 2681251565,
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000100";

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
    crate::tester::test_sequance_formula_matchces_head::<A000100>();
}
