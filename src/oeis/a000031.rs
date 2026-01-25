/// Number of n-bead necklaces with 2 colors when turning over is not allowed; also number of output sequences from a simple n-stage cycling shift register.
/// https://oeis.org/A000031

pub struct A000031;

impl crate::traits::IntegerSequence for A000031 {
    const NAME: &str = "Number of n-bead necklaces with 2 colors when turning over is not allowed";

    const HEAD: &[crate::Value] = &[
        1, 2, 3, 4, 6, 8, 14, 20, 36, 60, 108, 188, 352, 632, 1182, 2192, 4116, 7712, 14602, 27596,
        52488, 99880, 190746, 364724, 699252, 1342184, 2581428, 4971068, 9587580, 18512792,
        35792568, 69273668,
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000031";

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
    crate::tester::test_sequance_formula_matchces_head::<A000031>();
}
