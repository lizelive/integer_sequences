/// Number of n-bead necklaces with beads of 2 colors when turning over is allowed; also number of outputs of a Boolean function of n variables.
/// https://oeis.org/A000011

pub struct A000011;

impl crate::traits::IntegerSequence for A000011 {
    const NAME: &str = "Number of n-bead necklaces (turning over is allowed) where complements are equivalent";

    const HEAD: &[crate::Value] = &[
        1, 1, 2, 2, 4, 4, 8, 9, 18, 23, 44, 63, 122, 190, 362, 612, 1162, 2056, 3914, 7155, 13648,
        25482, 48734, 92205, 176906, 337594, 649532, 1246863, 2405236, 4636390, 8964800, 17334801,
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000011";

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
    crate::tester::test_sequance_formula_matchces_head::<A000011>();
}
