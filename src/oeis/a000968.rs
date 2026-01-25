/// a(n) = 9*n^2 + 6
/// https://oeis.org/A000968

pub struct A000968;

impl crate::traits::IntegerSequence for A000968 {
    const NAME: &str = "a(n) = 9*n^2 + 6";

    const HEAD: &[crate::Value] = &[
        6, 15, 42, 87, 150, 231, 330, 447, 582, 735, 906, 1095, 1302, 1527, 1770, 2031, 2310, 2607, 2922, 3255, 3606, 3975, 4362, 4767, 5190, 5631, 6090, 6567, 7062, 7575
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000968";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        sq_968(n)
    }
}

const fn sq_968(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    9 * n * n + 6 - 0 * n
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000968>();
}
