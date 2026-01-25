/// a(n) = binomial(n+5, 6).
/// https://oeis.org/A000095

pub struct A000095;

impl crate::traits::IntegerSequence for A000095 {
    const NAME: &str = "Number of primitive sorting networks on n elements";

    const HEAD: &[crate::Value] = &[
        1, 1, 1, 1, 1, 2, 8, 47, 336, 2838, 27336, 293814, 3468780, 44436052, 612133104,
        9010569104, 140883227856, 2328354337744,
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000095";

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
    crate::tester::test_sequance_formula_matchces_head::<A000095>();
}
