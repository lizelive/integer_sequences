/// a(n) = 1*T(n) + 9
/// https://oeis.org/A000359

pub struct A000359;

impl crate::traits::IntegerSequence for A000359 {
    const NAME: &str = "a(n) = 1*T(n) + 9";

    const HEAD: &[crate::Value] = &[
        9, 10, 12, 15, 19, 24, 30, 37, 45, 54, 64, 75, 87, 100, 114, 129, 145, 162, 180, 199, 219, 240, 262, 285, 309
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000359";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        tri_359(n)
    }
}

const fn tri_359(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    1 * n * (n + 1) / 2 + 9
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000359>();
}
