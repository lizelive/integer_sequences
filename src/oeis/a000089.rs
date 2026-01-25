/// Number of graphs with n nodes.
/// https://oeis.org/A000089

pub struct A000089;

impl crate::traits::IntegerSequence for A000089 {
    const NAME: &str = "Number of solutions to x^2 == 1 (mod n)";

    const HEAD: &[crate::Value] = &[
        1, 1, 2, 2, 2, 2, 2, 4, 2, 2, 2, 4, 2, 2, 4, 4, 2, 2, 2, 4, 4, 2, 2, 8, 2, 2, 2, 4, 2, 4,
        2, 4, 4, 2, 4, 4, 2, 2, 4, 8, 2, 4, 2, 4, 4, 2, 2, 8, 2, 2, 4, 4, 2, 2, 4, 8, 4, 2, 2, 8,
        2, 2, 4, 4, 4, 4, 2, 4, 4, 4, 2, 8, 2, 2, 4, 4, 4, 4, 2, 8, 2, 2, 2, 8, 4, 2, 4, 8, 2, 4,
    ];

    const OFFSET: crate::Index = 1;

    const SOURCE: &str = "https://oeis.org/A000089";

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
    crate::tester::test_sequance_formula_matchces_head::<A000089>();
}
