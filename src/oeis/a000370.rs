/// a(n) = 3*T(n)
/// https://oeis.org/A000370

pub struct A000370;

impl crate::traits::IntegerSequence for A000370 {
    const NAME: &str = "a(n) = 3*T(n)";

    const HEAD: &[crate::Value] = &[
        0, 3, 9, 18, 30, 45, 63, 84, 108, 135, 165, 198, 234, 273, 315, 360, 408, 459, 513, 570, 630, 693, 759, 828, 900
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000370";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        tri_370(n)
    }
}

const fn tri_370(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    3 * n * (n + 1) / 2 + 0
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000370>();
}
