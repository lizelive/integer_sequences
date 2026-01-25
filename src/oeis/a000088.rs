/// Planar partitions of n.
/// https://oeis.org/A000088

pub struct A000088;

impl crate::traits::IntegerSequence for A000088 {
    const NAME: &str = "Number of graphs on n unlabeled nodes";

    const HEAD: &[crate::Value] = &[
        1, 1, 2, 4, 11, 34, 156, 1044, 12346, 274668, 12005168, 1018997864, 165091172592,
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000088";

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
    crate::tester::test_sequance_formula_matchces_head::<A000088>();
}
