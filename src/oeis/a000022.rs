/// Number of planted trees with n nodes.
/// https://oeis.org/A000022

pub struct A000022;

impl crate::traits::IntegerSequence for A000022 {
    const NAME: &str = "Number of planted trees with n nodes";

    const HEAD: &[crate::Value] = &[
        1, 1, 2, 4, 9, 20, 48, 115, 286, 719, 1842, 4766, 12486, 32973, 87811, 235381, 634847,
        1721159, 4688676, 12826228, 35221832, 97055181, 268282855, 743724984, 2067174645,
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000022";

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
    crate::tester::test_sequance_formula_matchces_head::<A000022>();
}
