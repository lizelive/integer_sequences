/// a(n) = 4*n^2 + 3*n + 3
/// https://oeis.org/A000792

pub struct A000792;

impl crate::traits::IntegerSequence for A000792 {
    const NAME: &str = "a(n) = 4*n^2 + 3*n + 3";

    const HEAD: &[crate::Value] = &[
        3, 10, 25, 48, 79, 118, 165, 220, 283, 354, 433, 520, 615, 718, 829, 948, 1075, 1210, 1353, 1504, 1663, 1830, 2005, 2188, 2379, 2578, 2785, 3000, 3223, 3454
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000792";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        quad_792(n)
    }
}

const fn quad_792(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    4 * n * n + 3 * n + 3
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000792>();
}
