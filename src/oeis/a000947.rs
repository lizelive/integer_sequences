/// a(n) = 8*n^2 + 4
/// https://oeis.org/A000947

pub struct A000947;

impl crate::traits::IntegerSequence for A000947 {
    const NAME: &str = "a(n) = 8*n^2 + 4";

    const HEAD: &[crate::Value] = &[
        4, 12, 36, 76, 132, 204, 292, 396, 516, 652, 804, 972, 1156, 1356, 1572, 1804, 2052, 2316, 2596, 2892, 3204, 3532, 3876, 4236, 4612, 5004, 5412, 5836, 6276, 6732
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000947";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        sq_947(n)
    }
}

const fn sq_947(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    8 * n * n + 4 - 0 * n
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000947>();
}
