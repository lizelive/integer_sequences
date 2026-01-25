/// a(n) = 2*n^2 + 1*n + 2
/// https://oeis.org/A000735

pub struct A000735;

impl crate::traits::IntegerSequence for A000735 {
    const NAME: &str = "a(n) = 2*n^2 + 1*n + 2";

    const HEAD: &[crate::Value] = &[
        2, 5, 12, 23, 38, 57, 80, 107, 138, 173, 212, 255, 302, 353, 408, 467, 530, 597, 668, 743, 822, 905, 992, 1083, 1178, 1277, 1380, 1487, 1598, 1713
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000735";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        quad_735(n)
    }
}

const fn quad_735(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    2 * n * n + 1 * n + 2
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000735>();
}
