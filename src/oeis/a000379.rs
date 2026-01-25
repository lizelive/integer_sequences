/// a(n) = 3*T(n) + 9
/// https://oeis.org/A000379

pub struct A000379;

impl crate::traits::IntegerSequence for A000379 {
    const NAME: &str = "a(n) = 3*T(n) + 9";

    const HEAD: &[crate::Value] = &[
        9, 12, 18, 27, 39, 54, 72, 93, 117, 144, 174, 207, 243, 282, 324, 369, 417, 468, 522, 579, 639, 702, 768, 837, 909
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000379";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        tri_379(n)
    }
}

const fn tri_379(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    3 * n * (n + 1) / 2 + 9
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000379>();
}
