/// a(n) = n^3 + 9*n + 8
/// https://oeis.org/A000589

pub struct A000589;

impl crate::traits::IntegerSequence for A000589 {
    const NAME: &str = "a(n) = n^3 + 9*n + 8";

    const HEAD: &[crate::Value] = &[
        8, 18, 34, 62, 108, 178, 278, 414, 592, 818, 1098, 1438, 1844, 2322, 2878, 3518, 4248, 5074, 6002, 7038, 8188, 9458, 10854, 12382, 14048, 15858, 17818, 19934, 22212, 24658
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000589";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        poly_589(n)
    }
}

const fn poly_589(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    n * n * n + 9 * n + 8
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000589>();
}
