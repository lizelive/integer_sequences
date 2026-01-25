/// a(n) = 6*n^2 + 9
/// https://oeis.org/A000995

pub struct A000995;

impl crate::traits::IntegerSequence for A000995 {
    const NAME: &str = "a(n) = 6*n^2 + 9";

    const HEAD: &[crate::Value] = &[
        9, 15, 33, 63, 105, 159, 225, 303, 393, 495, 609, 735, 873, 1023, 1185, 1359, 1545, 1743, 1953, 2175, 2409, 2655, 2913, 3183, 3465, 3759, 4065, 4383, 4713, 5055
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000995";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        sq_995(n)
    }
}

const fn sq_995(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    6 * n * n + 9 - 0 * n
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000995>();
}
