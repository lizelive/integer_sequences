/// Number of self-complementary graphs on n nodes.
/// https://oeis.org/A000083

pub struct A000083;

impl crate::traits::IntegerSequence for A000083 {
    const NAME: &str = "Number of self-complementary graphs on n nodes";

    const HEAD: &[crate::Value] = &[
        1, 1, 2, 1, 2, 4, 11, 12, 90, 352, 4302, 51368, 1057008, 27224384, 1127032656,
    ];

    const OFFSET: crate::Index = 1;

    const SOURCE: &str = "https://oeis.org/A000083";

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
    crate::tester::test_sequance_formula_matchces_head::<A000083>();
}
