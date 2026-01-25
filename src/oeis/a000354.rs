/// a(n) = 1*T(n) + 4
/// https://oeis.org/A000354

pub struct A000354;

impl crate::traits::IntegerSequence for A000354 {
    const NAME: &str = "a(n) = 1*T(n) + 4";

    const HEAD: &[crate::Value] = &[
        4, 5, 7, 10, 14, 19, 25, 32, 40, 49, 59, 70, 82, 95, 109, 124, 140, 157, 175, 194, 214, 235, 257, 280, 304
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000354";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        tri_354(n)
    }
}

const fn tri_354(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    1 * n * (n + 1) / 2 + 4
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000354>();
}
