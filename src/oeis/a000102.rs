/// Number of partitions of n if there are two kinds of 1's.
/// https://oeis.org/A000102

pub struct A000102;

impl crate::traits::IntegerSequence for A000102 {
    const NAME: &str = "Number of partitions of n if there are two kinds of 1's";

    const HEAD: &[crate::Value] = &[
        1, 2, 4, 6, 10, 14, 22, 30, 44, 60, 84, 112, 154, 202, 272, 352, 466, 596, 778, 986, 1272,
        1598, 2042, 2548, 3230, 4004, 5034, 6202, 7744, 9492, 11780, 14372, 17738, 21536, 26432,
        31948, 39014, 46982, 57136, 68552, 83072, 99296, 119900, 142866, 171948, 204254, 245042,
        290228, 347148, 410168,
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000102";

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
    crate::tester::test_sequance_formula_matchces_head::<A000102>();
}
