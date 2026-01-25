/// a(n) = 2*n^2 + 3*n + 3
/// https://oeis.org/A000742

pub struct A000742;

impl crate::traits::IntegerSequence for A000742 {
    const NAME: &str = "a(n) = 2*n^2 + 3*n + 3";

    const HEAD: &[crate::Value] = &[
        3, 8, 17, 30, 47, 68, 93, 122, 155, 192, 233, 278, 327, 380, 437, 498, 563, 632, 705, 782, 863, 948, 1037, 1130, 1227, 1328, 1433, 1542, 1655, 1772
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000742";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        quad_742(n)
    }
}

const fn quad_742(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    2 * n * n + 3 * n + 3
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000742>();
}
