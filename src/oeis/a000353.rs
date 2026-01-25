/// a(n) = 1*T(n) + 3
/// https://oeis.org/A000353

pub struct A000353;

impl crate::traits::IntegerSequence for A000353 {
    const NAME: &str = "a(n) = 1*T(n) + 3";

    const HEAD: &[crate::Value] = &[
        3, 4, 6, 9, 13, 18, 24, 31, 39, 48, 58, 69, 81, 94, 108, 123, 139, 156, 174, 193, 213, 234, 256, 279, 303
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000353";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        tri_353(n)
    }
}

const fn tri_353(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    1 * n * (n + 1) / 2 + 3
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000353>();
}
