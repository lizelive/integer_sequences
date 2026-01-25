/// a(n) = n^3 + 6*n + 9
/// https://oeis.org/A000596

pub struct A000596;

impl crate::traits::IntegerSequence for A000596 {
    const NAME: &str = "a(n) = n^3 + 6*n + 9";

    const HEAD: &[crate::Value] = &[
        9, 16, 29, 54, 97, 164, 261, 394, 569, 792, 1069, 1406, 1809, 2284, 2837, 3474, 4201, 5024, 5949, 6982, 8129, 9396, 10789, 12314, 13977, 15784, 17741, 19854, 22129, 24572
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000596";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        poly_596(n)
    }
}

const fn poly_596(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    n * n * n + 6 * n + 9
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000596>();
}
