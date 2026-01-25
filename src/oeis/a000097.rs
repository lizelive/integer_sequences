/// a(n) = floor(3^n / 2^n).
/// https://oeis.org/A000097

pub struct A000097;

impl crate::traits::IntegerSequence for A000097 {
    const NAME: &str = "Number of partitions of n if there are two kinds of 1's and two kinds of 2's";

    const HEAD: &[crate::Value] = &[
        1, 2, 5, 10, 20, 36, 65, 110, 185, 300, 481, 756, 1175, 1800, 2730, 4092, 6084, 8954,
        13085, 18966, 27320, 39084, 55601, 78632, 110625, 154880, 215855, 299550, 413785, 569140,
        779636, 1064008, 1447155, 1961490, 2650265, 3569810, 4793041, 6417890, 8572265, 11418820,
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000097";

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
    crate::tester::test_sequance_formula_matchces_head::<A000097>();
}
