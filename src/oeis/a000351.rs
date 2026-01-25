/// a(n) = 1*T(n) + 1
/// https://oeis.org/A000351

pub struct A000351;

impl crate::traits::IntegerSequence for A000351 {
    const NAME: &str = "a(n) = 1*T(n) + 1";

    const HEAD: &[crate::Value] = &[
        1, 2, 4, 7, 11, 16, 22, 29, 37, 46, 56, 67, 79, 92, 106, 121, 137, 154, 172, 191, 211, 232, 254, 277, 301
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000351";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        tri_351(n)
    }
}

const fn tri_351(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    1 * n * (n + 1) / 2 + 1
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000351>();
}
