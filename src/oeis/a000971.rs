/// a(n) = 2*n^2 + 7
/// https://oeis.org/A000971

pub struct A000971;

impl crate::traits::IntegerSequence for A000971 {
    const NAME: &str = "a(n) = 2*n^2 + 7";

    const HEAD: &[crate::Value] = &[
        7, 9, 15, 25, 39, 57, 79, 105, 135, 169, 207, 249, 295, 345, 399, 457, 519, 585, 655, 729, 807, 889, 975, 1065, 1159, 1257, 1359, 1465, 1575, 1689
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000971";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        sq_971(n)
    }
}

const fn sq_971(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    2 * n * n + 7 - 0 * n
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000971>();
}
