/// Number of primitive periodic sequences of period n on a 2 letter alphabet.
/// https://oeis.org/A000016

pub struct A000016;

impl crate::traits::IntegerSequence for A000016 {
    const NAME: &str = "Number of distinct primitive n-th roots of unity";

    const HEAD: &[crate::Value] = &[
        1, 1, 1, 2, 3, 6, 9, 18, 30, 56, 99, 186, 335, 630, 1161, 2182, 4080, 7710, 14532, 27594,
        52377, 99858, 190557, 364722, 698870, 1342176, 2580795, 4971008, 9586395, 18512790,
        35790267,
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000016";

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
    crate::tester::test_sequance_formula_matchces_head::<A000016>();
}
