/// a(n) = 1*T(n) + 8
/// https://oeis.org/A000358

pub struct A000358;

impl crate::traits::IntegerSequence for A000358 {
    const NAME: &str = "a(n) = 1*T(n) + 8";

    const HEAD: &[crate::Value] = &[
        8, 9, 11, 14, 18, 23, 29, 36, 44, 53, 63, 74, 86, 99, 113, 128, 144, 161, 179, 198, 218, 239, 261, 284, 308
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000358";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        tri_358(n)
    }
}

const fn tri_358(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    1 * n * (n + 1) / 2 + 8
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000358>();
}
