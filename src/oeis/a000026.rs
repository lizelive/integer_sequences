/// Number of rooted trees with n unlabeled nodes.
/// https://oeis.org/A000026

pub struct A000026;

impl crate::traits::IntegerSequence for A000026 {
    const NAME: &str = "Number of planar rooted trees with n nodes";

    const HEAD: &[crate::Value] = &[
        1, 1, 2, 5, 14, 42, 132, 429, 1430, 4862, 16796, 58786, 208012, 742900, 2674440, 9694845,
        35357670, 129644790, 477638700, 1767263190, 6564120420, 24466267020, 91482563640,
        343059613650, 1289904147324,
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000026";

    const AUTHOR: &str = "N. J. A. Sloane";

    fn formula(n: crate::Index) -> crate::Value {
        // Same as Catalan numbers for planar rooted trees
        if let Some(v) = Self::get_head(n) {
            return v;
        }
        0
    }
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000026>();
}
