/// a(n) = 2*T(n)
/// https://oeis.org/A000360

pub struct A000360;

impl crate::traits::IntegerSequence for A000360 {
    const NAME: &str = "a(n) = 2*T(n)";

    const HEAD: &[crate::Value] = &[
        0, 2, 6, 12, 20, 30, 42, 56, 72, 90, 110, 132, 156, 182, 210, 240, 272, 306, 342, 380, 420, 462, 506, 552, 600
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000360";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        tri_360(n)
    }
}

const fn tri_360(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    2 * n * (n + 1) / 2 + 0
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000360>();
}
