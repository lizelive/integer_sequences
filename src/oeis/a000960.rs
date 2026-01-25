/// a(n) = 1*n^2 + 6
/// https://oeis.org/A000960

pub struct A000960;

impl crate::traits::IntegerSequence for A000960 {
    const NAME: &str = "a(n) = 1*n^2 + 6";

    const HEAD: &[crate::Value] = &[
        6, 7, 10, 15, 22, 31, 42, 55, 70, 87, 106, 127, 150, 175, 202, 231, 262, 295, 330, 367, 406, 447, 490, 535, 582, 631, 682, 735, 790, 847
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000960";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        sq_960(n)
    }
}

const fn sq_960(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    1 * n * n + 6 - 0 * n
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000960>();
}
