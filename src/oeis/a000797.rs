/// a(n) = 4*n^2 + 3*n + 4
/// https://oeis.org/A000797

pub struct A000797;

impl crate::traits::IntegerSequence for A000797 {
    const NAME: &str = "a(n) = 4*n^2 + 3*n + 4";

    const HEAD: &[crate::Value] = &[
        4, 11, 26, 49, 80, 119, 166, 221, 284, 355, 434, 521, 616, 719, 830, 949, 1076, 1211, 1354, 1505, 1664, 1831, 2006, 2189, 2380, 2579, 2786, 3001, 3224, 3455
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000797";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        quad_797(n)
    }
}

const fn quad_797(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    4 * n * n + 3 * n + 4
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000797>();
}
