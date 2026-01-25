/// a(n) = 4*n^2 + 1*n + 4
/// https://oeis.org/A000795

pub struct A000795;

impl crate::traits::IntegerSequence for A000795 {
    const NAME: &str = "a(n) = 4*n^2 + 1*n + 4";

    const HEAD: &[crate::Value] = &[
        4, 9, 22, 43, 72, 109, 154, 207, 268, 337, 414, 499, 592, 693, 802, 919, 1044, 1177, 1318, 1467, 1624, 1789, 1962, 2143, 2332, 2529, 2734, 2947, 3168, 3397
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000795";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        quad_795(n)
    }
}

const fn quad_795(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    4 * n * n + 1 * n + 4
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000795>();
}
