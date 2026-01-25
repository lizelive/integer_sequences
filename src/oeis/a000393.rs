/// a(n) = 5*T(n) + 3
/// https://oeis.org/A000393

pub struct A000393;

impl crate::traits::IntegerSequence for A000393 {
    const NAME: &str = "a(n) = 5*T(n) + 3";

    const HEAD: &[crate::Value] = &[
        3, 8, 18, 33, 53, 78, 108, 143, 183, 228, 278, 333, 393, 458, 528, 603, 683, 768, 858, 953, 1053, 1158, 1268, 1383, 1503
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000393";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        tri_393(n)
    }
}

const fn tri_393(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    5 * n * (n + 1) / 2 + 3
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000393>();
}
