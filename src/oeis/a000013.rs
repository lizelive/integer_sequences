/// Number of n-bead binary necklaces with beads of 2 colors where the colors differ by at most 1.
/// https://oeis.org/A000013

pub struct A000013;

impl crate::traits::IntegerSequence for A000013 {
    const NAME: &str = "Number of n-bead binary necklaces with beads of 2 colors where the colors differ by at most 1";

    const HEAD: &[crate::Value] = &[
        1, 2, 2, 2, 4, 4, 8, 10, 20, 30, 56, 94, 180, 316, 596, 1096, 2068, 3856, 7316, 13798,
        26272, 49940, 95420, 182362, 349716, 671092, 1290872, 2485534, 4793492, 9256396, 17896832,
        34636834,
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000013";

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
    crate::tester::test_sequance_formula_matchces_head::<A000013>();
}
