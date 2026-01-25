/// Number of regular simple graphs on n nodes.
/// https://oeis.org/A000021

pub struct A000021;

impl crate::traits::IntegerSequence for A000021 {
    const NAME: &str = "Number of connected regular graphs on n nodes";

    const HEAD: &[crate::Value] = &[
        1, 1, 1, 2, 2, 5, 4, 17, 22, 167, 539, 18979, 389436, 50314796, 2942198440,
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000021";

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
    crate::tester::test_sequance_formula_matchces_head::<A000021>();
}
