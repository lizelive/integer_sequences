/// a(n) = 3*n^2 + 5*n + 4
/// https://oeis.org/A000774

pub struct A000774;

impl crate::traits::IntegerSequence for A000774 {
    const NAME: &str = "a(n) = 3*n^2 + 5*n + 4";

    const HEAD: &[crate::Value] = &[
        4, 12, 26, 46, 72, 104, 142, 186, 236, 292, 354, 422, 496, 576, 662, 754, 852, 956, 1066, 1182, 1304, 1432, 1566, 1706, 1852, 2004, 2162, 2326, 2496, 2672
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000774";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        quad_774(n)
    }
}

const fn quad_774(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    3 * n * n + 5 * n + 4
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000774>();
}
