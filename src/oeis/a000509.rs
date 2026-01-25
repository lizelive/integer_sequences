/// a(n) = n^3 + 9*n + 0
/// https://oeis.org/A000509

pub struct A000509;

impl crate::traits::IntegerSequence for A000509 {
    const NAME: &str = "a(n) = n^3 + 9*n + 0";

    const HEAD: &[crate::Value] = &[
        0, 10, 26, 54, 100, 170, 270, 406, 584, 810, 1090, 1430, 1836, 2314, 2870, 3510, 4240, 5066, 5994, 7030, 8180, 9450, 10846, 12374, 14040, 15850, 17810, 19926, 22204, 24650
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000509";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        poly_509(n)
    }
}

const fn poly_509(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    n * n * n + 9 * n + 0
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000509>();
}
