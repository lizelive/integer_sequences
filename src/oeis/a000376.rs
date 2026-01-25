/// a(n) = 3*T(n) + 6
/// https://oeis.org/A000376

pub struct A000376;

impl crate::traits::IntegerSequence for A000376 {
    const NAME: &str = "a(n) = 3*T(n) + 6";

    const HEAD: &[crate::Value] = &[
        6, 9, 15, 24, 36, 51, 69, 90, 114, 141, 171, 204, 240, 279, 321, 366, 414, 465, 519, 576, 636, 699, 765, 834, 906
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000376";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        tri_376(n)
    }
}

const fn tri_376(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    3 * n * (n + 1) / 2 + 6
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000376>();
}
