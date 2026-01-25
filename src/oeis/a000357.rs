/// a(n) = 1*T(n) + 7
/// https://oeis.org/A000357

pub struct A000357;

impl crate::traits::IntegerSequence for A000357 {
    const NAME: &str = "a(n) = 1*T(n) + 7";

    const HEAD: &[crate::Value] = &[
        7, 8, 10, 13, 17, 22, 28, 35, 43, 52, 62, 73, 85, 98, 112, 127, 143, 160, 178, 197, 217, 238, 260, 283, 307
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000357";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        tri_357(n)
    }
}

const fn tri_357(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    1 * n * (n + 1) / 2 + 7
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000357>();
}
