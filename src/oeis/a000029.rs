/// Number of necklaces with n beads of 2 colors, where turning over is allowed.
/// https://oeis.org/A000029

pub struct A000029;

impl crate::traits::IntegerSequence for A000029 {
    const NAME: &str = "Number of necklaces with n beads of 2 colors, allowing turning over";

    const HEAD: &[crate::Value] = &[
        1, 2, 3, 4, 6, 8, 13, 18, 30, 46, 78, 126, 224, 380, 687, 1224, 2250, 4112, 7685, 14310,
        27012, 50964, 96909, 184410, 352698, 675188, 1296858, 2493726, 4806078, 9272780, 17920860,
        34669602,
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000029";

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
    crate::tester::test_sequance_formula_matchces_head::<A000029>();
}
