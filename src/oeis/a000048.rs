/// Number of n-bead binary necklaces with beads of 2 colors where the weights differ by at most 1, turning over is not allowed.
/// https://oeis.org/A000048

pub struct A000048;

impl crate::traits::IntegerSequence for A000048 {
    const NAME: &str = "Number of n-bead binary necklaces with beads of 2 colors";

    const HEAD: &[crate::Value] = &[
        1, 1, 1, 2, 3, 6, 9, 18, 30, 56, 99, 186, 335, 630, 1161, 2182, 4080, 7710, 14532, 27594,
        52377, 99858, 190557, 364722, 698870, 1342176, 2580795, 4971008, 9586395, 18512790,
        35790267, 69273666,
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000048";

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
    crate::tester::test_sequance_formula_matchces_head::<A000048>();
}
