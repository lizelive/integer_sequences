/// a(n) = n^3 + 9*n + 1
/// https://oeis.org/A000519

pub struct A000519;

impl crate::traits::IntegerSequence for A000519 {
    const NAME: &str = "a(n) = n^3 + 9*n + 1";

    const HEAD: &[crate::Value] = &[
        1, 11, 27, 55, 101, 171, 271, 407, 585, 811, 1091, 1431, 1837, 2315, 2871, 3511, 4241, 5067, 5995, 7031, 8181, 9451, 10847, 12375, 14041, 15851, 17811, 19927, 22205, 24651
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000519";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        poly_519(n)
    }
}

const fn poly_519(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    n * n * n + 9 * n + 1
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000519>();
}
