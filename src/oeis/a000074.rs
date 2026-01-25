/// Number of odd permutations with exactly 2 fixed points.
/// https://oeis.org/A000074

pub struct A000074;

impl crate::traits::IntegerSequence for A000074 {
    const NAME: &str = "Number of permutations with a given number of fixed points";

    const HEAD: &[crate::Value] = &[
        1, 0, 1, 2, 6, 14, 36, 90, 232, 600, 1572, 4144, 11016, 29456, 79264, 214368, 582528,
        1589776, 4353408, 11959648, 32949648, 91013552, 252028832, 699369824, 1945163584,
        5420001056, 15133174592, 42322476224, 118539108480, 332462960768, 933789069312,
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000074";

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
    crate::tester::test_sequance_formula_matchces_head::<A000074>();
}
