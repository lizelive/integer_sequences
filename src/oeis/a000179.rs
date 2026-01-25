/// a(n) = n^2 + 9*n + 2
/// https://oeis.org/A000179

pub struct A000179;

impl crate::traits::IntegerSequence for A000179 {
    const NAME: &str = "a(n) = n^2 + 9*n + 2";

    const HEAD: &[crate::Value] = &[
        2, 12, 24, 38, 54, 72, 92, 114, 138, 164, 192, 222, 254, 288, 324, 362, 402, 444, 488, 534, 582, 632, 684, 738, 794
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000179";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        poly_179(n)
    }
}

const fn poly_179(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    n * n + 9 * n + 2
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000179>();
}
