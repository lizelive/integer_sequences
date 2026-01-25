/// a(n) = 2*T(n) + 9
/// https://oeis.org/A000369

pub struct A000369;

impl crate::traits::IntegerSequence for A000369 {
    const NAME: &str = "a(n) = 2*T(n) + 9";

    const HEAD: &[crate::Value] = &[
        9, 11, 15, 21, 29, 39, 51, 65, 81, 99, 119, 141, 165, 191, 219, 249, 281, 315, 351, 389, 429, 471, 515, 561, 609
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000369";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        tri_369(n)
    }
}

const fn tri_369(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    2 * n * (n + 1) / 2 + 9
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000369>();
}
