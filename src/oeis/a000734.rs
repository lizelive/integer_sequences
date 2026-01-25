/// a(n) = 2*n^2 + 5*n + 1
/// https://oeis.org/A000734

pub struct A000734;

impl crate::traits::IntegerSequence for A000734 {
    const NAME: &str = "a(n) = 2*n^2 + 5*n + 1";

    const HEAD: &[crate::Value] = &[
        1, 8, 19, 34, 53, 76, 103, 134, 169, 208, 251, 298, 349, 404, 463, 526, 593, 664, 739, 818, 901, 988, 1079, 1174, 1273, 1376, 1483, 1594, 1709, 1828
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000734";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        quad_734(n)
    }
}

const fn quad_734(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    2 * n * n + 5 * n + 1
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000734>();
}
