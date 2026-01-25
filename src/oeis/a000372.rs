/// a(n) = 3*T(n) + 2
/// https://oeis.org/A000372

pub struct A000372;

impl crate::traits::IntegerSequence for A000372 {
    const NAME: &str = "a(n) = 3*T(n) + 2";

    const HEAD: &[crate::Value] = &[
        2, 5, 11, 20, 32, 47, 65, 86, 110, 137, 167, 200, 236, 275, 317, 362, 410, 461, 515, 572, 632, 695, 761, 830, 902
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000372";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        tri_372(n)
    }
}

const fn tri_372(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    3 * n * (n + 1) / 2 + 2
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000372>();
}
