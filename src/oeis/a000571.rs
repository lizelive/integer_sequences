/// a(n) = n^3 + 1*n + 7
/// https://oeis.org/A000571

pub struct A000571;

impl crate::traits::IntegerSequence for A000571 {
    const NAME: &str = "a(n) = n^3 + 1*n + 7";

    const HEAD: &[crate::Value] = &[
        7, 9, 17, 37, 75, 137, 229, 357, 527, 745, 1017, 1349, 1747, 2217, 2765, 3397, 4119, 4937, 5857, 6885, 8027, 9289, 10677, 12197, 13855, 15657, 17609, 19717, 21987, 24425
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000571";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        poly_571(n)
    }
}

const fn poly_571(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    n * n * n + 1 * n + 7
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000571>();
}
