/// a(n) = 7*n^2 + 3
/// https://oeis.org/A000936

pub struct A000936;

impl crate::traits::IntegerSequence for A000936 {
    const NAME: &str = "a(n) = 7*n^2 + 3";

    const HEAD: &[crate::Value] = &[
        3, 10, 31, 66, 115, 178, 255, 346, 451, 570, 703, 850, 1011, 1186, 1375, 1578, 1795, 2026, 2271, 2530, 2803, 3090, 3391, 3706, 4035, 4378, 4735, 5106, 5491, 5890
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000936";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        sq_936(n)
    }
}

const fn sq_936(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    7 * n * n + 3 - 0 * n
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000936>();
}
