/// a(n) = 3*n^2 + 4*n + 3
/// https://oeis.org/A000768

pub struct A000768;

impl crate::traits::IntegerSequence for A000768 {
    const NAME: &str = "a(n) = 3*n^2 + 4*n + 3";

    const HEAD: &[crate::Value] = &[
        3, 10, 23, 42, 67, 98, 135, 178, 227, 282, 343, 410, 483, 562, 647, 738, 835, 938, 1047, 1162, 1283, 1410, 1543, 1682, 1827, 1978, 2135, 2298, 2467, 2642
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000768";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        quad_768(n)
    }
}

const fn quad_768(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    3 * n * n + 4 * n + 3
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000768>();
}
