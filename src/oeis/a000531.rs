/// a(n) = n^3 + 1*n + 3
/// https://oeis.org/A000531

pub struct A000531;

impl crate::traits::IntegerSequence for A000531 {
    const NAME: &str = "a(n) = n^3 + 1*n + 3";

    const HEAD: &[crate::Value] = &[
        3, 5, 13, 33, 71, 133, 225, 353, 523, 741, 1013, 1345, 1743, 2213, 2761, 3393, 4115, 4933, 5853, 6881, 8023, 9285, 10673, 12193, 13851, 15653, 17605, 19713, 21983, 24421
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000531";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        poly_531(n)
    }
}

const fn poly_531(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    n * n * n + 1 * n + 3
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000531>();
}
