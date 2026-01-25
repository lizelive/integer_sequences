/// a(n) = n^2 + 8*n + 0
/// https://oeis.org/A000158

pub struct A000158;

impl crate::traits::IntegerSequence for A000158 {
    const NAME: &str = "a(n) = n^2 + 8*n + 0";

    const HEAD: &[crate::Value] = &[
        0, 9, 20, 33, 48, 65, 84, 105, 128, 153, 180, 209, 240, 273, 308, 345, 384, 425, 468, 513, 560, 609, 660, 713, 768
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000158";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        poly_158(n)
    }
}

const fn poly_158(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    n * n + 8 * n + 0
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000158>();
}
