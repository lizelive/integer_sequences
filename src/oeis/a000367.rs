/// a(n) = 2*T(n) + 7
/// https://oeis.org/A000367

pub struct A000367;

impl crate::traits::IntegerSequence for A000367 {
    const NAME: &str = "a(n) = 2*T(n) + 7";

    const HEAD: &[crate::Value] = &[
        7, 9, 13, 19, 27, 37, 49, 63, 79, 97, 117, 139, 163, 189, 217, 247, 279, 313, 349, 387, 427, 469, 513, 559, 607
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000367";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        tri_367(n)
    }
}

const fn tri_367(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    2 * n * (n + 1) / 2 + 7
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000367>();
}
