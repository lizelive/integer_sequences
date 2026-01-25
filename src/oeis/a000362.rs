/// a(n) = 2*T(n) + 2
/// https://oeis.org/A000362

pub struct A000362;

impl crate::traits::IntegerSequence for A000362 {
    const NAME: &str = "a(n) = 2*T(n) + 2";

    const HEAD: &[crate::Value] = &[
        2, 4, 8, 14, 22, 32, 44, 58, 74, 92, 112, 134, 158, 184, 212, 242, 274, 308, 344, 382, 422, 464, 508, 554, 602
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000362";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        tri_362(n)
    }
}

const fn tri_362(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    2 * n * (n + 1) / 2 + 2
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000362>();
}
