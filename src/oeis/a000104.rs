/// Number of partitions of n if there are two kinds of 1's, 2's, and 3's.
/// https://oeis.org/A000104

pub struct A000104;

impl crate::traits::IntegerSequence for A000104 {
    const NAME: &str = "Number of partitions of n if there are two kinds of 1's, 2's, and 3's";

    const HEAD: &[crate::Value] = &[
        1, 2, 5, 12, 26, 52, 102, 190, 348, 618, 1078, 1840, 3094, 5118, 8358, 13466, 21456,
        33808, 52750, 81510, 124796, 189390, 285156, 426024, 632064, 931328, 1363590, 1983016,
        2867906, 4124046, 5902968, 8403928, 11911998, 16805932, 23608670, 33029692, 46038116,
        63935380, 88471252, 121984810,
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000104";

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
    crate::tester::test_sequance_formula_matchces_head::<A000104>();
}
