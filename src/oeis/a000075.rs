/// Number of rooted trees on n nodes.
/// https://oeis.org/A000075

pub struct A000075;

impl crate::traits::IntegerSequence for A000075 {
    const NAME: &str = "Number of rooted trees on n nodes";

    const HEAD: &[crate::Value] = &[
        0, 1, 1, 2, 4, 9, 20, 48, 115, 286, 719, 1842, 4766, 12486, 32973, 87811, 235381, 634847,
        1721159, 4688676, 12826228, 35221832, 97055181, 268282855, 743724984, 2067174645,
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000075";

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
    crate::tester::test_sequance_formula_matchces_head::<A000075>();
}
