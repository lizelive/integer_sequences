/// Number of partitions of n if there are two kinds of 1's and 2's.
/// https://oeis.org/A000103

pub struct A000103;

impl crate::traits::IntegerSequence for A000103 {
    const NAME: &str = "Number of partitions of n if there are two kinds of 1's and 2's";

    const HEAD: &[crate::Value] = &[
        1, 2, 5, 10, 19, 34, 60, 100, 166, 266, 422, 656, 1011, 1534, 2310, 3436, 5074, 7420,
        10784, 15548, 22285, 31730, 44921, 63212, 88494, 123238, 170902, 235850, 324153, 443618,
        605027, 821972, 1113183, 1502658, 2022370, 2713684, 3630228, 4842116, 6440696, 8544692,
        11309984,
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000103";

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
    crate::tester::test_sequance_formula_matchces_head::<A000103>();
}
