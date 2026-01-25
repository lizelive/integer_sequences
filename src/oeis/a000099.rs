/// a(n) = floor(5^n / 4^n).
/// https://oeis.org/A000099

pub struct A000099;

impl crate::traits::IntegerSequence for A000099 {
    const NAME: &str = "Number of partitions of n if there are two kinds of 1's, 2's, 3's and 4's";

    const HEAD: &[crate::Value] = &[
        1, 2, 5, 12, 28, 60, 127, 258, 514, 998, 1901, 3554, 6539, 11834, 21119, 37180, 64670,
        111150, 188995, 317776, 529170, 872914, 1427226, 2314042, 3722393, 5946428, 9434091,
        14865658, 23278084, 36235084, 56101379, 86379366, 132330110, 201743010, 306093825,
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000099";

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
    crate::tester::test_sequance_formula_matchces_head::<A000099>();
}
