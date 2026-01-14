/// Number of groups of order n.
/// https://oeis.org/A000001 
pub const fn num_groups(n: crate::Index) -> crate::Value {
    return 0;
}

pub struct A000001;

impl crate::traits::IntegerSequence for A000001 {
    const NAME: &str = "Number of groups of order n.";

    const HEAD: &[crate::Value] = &[
        0, 1, 1, 1, 2, 1, 2, 1, 5, 2, 2, 1, 5, 1, 2, 1, 14, 1, 5, 1, 5, 2, 2, 1, 15, 2, 2, 5, 4, 1,
        4, 1, 51, 1, 2, 1, 14, 1, 2, 2, 14, 1, 6, 1, 4, 2, 2, 1, 52, 2, 5, 1, 5, 1, 15, 2, 13, 2,
        2, 1, 13, 1, 2, 4, 267, 1, 4, 1, 5, 1, 4, 1, 50, 1, 2, 3, 4, 1, 6, 1, 52, 15, 2, 1, 15, 1,
        2, 1, 12, 1, 10, 1, 4, 2,
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000001";

    const AUTHOR: &str = "N. J. A. Sloane";

    fn formula(n: crate::Index) -> crate::Value {
        num_groups(n)
    }
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head(){
    crate::tester::test_sequance_formula_matchces_head::<A000001>();
}