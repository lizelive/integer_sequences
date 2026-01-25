/// a(n) = 3*n^2 + 1*n + 0
/// https://oeis.org/A000750

pub struct A000750;

impl crate::traits::IntegerSequence for A000750 {
    const NAME: &str = "a(n) = 3*n^2 + 1*n + 0";

    const HEAD: &[crate::Value] = &[
        0, 4, 14, 30, 52, 80, 114, 154, 200, 252, 310, 374, 444, 520, 602, 690, 784, 884, 990, 1102, 1220, 1344, 1474, 1610, 1752, 1900, 2054, 2214, 2380, 2552
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000750";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        quad_750(n)
    }
}

const fn quad_750(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    3 * n * n + 1 * n + 0
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000750>();
}
