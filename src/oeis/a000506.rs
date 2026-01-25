/// a(n) = n^3 + 6*n + 0
/// https://oeis.org/A000506

pub struct A000506;

impl crate::traits::IntegerSequence for A000506 {
    const NAME: &str = "a(n) = n^3 + 6*n + 0";

    const HEAD: &[crate::Value] = &[
        0, 7, 20, 45, 88, 155, 252, 385, 560, 783, 1060, 1397, 1800, 2275, 2828, 3465, 4192, 5015, 5940, 6973, 8120, 9387, 10780, 12305, 13968, 15775, 17732, 19845, 22120, 24563
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000506";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        poly_506(n)
    }
}

const fn poly_506(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    n * n * n + 6 * n + 0
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000506>();
}
