/// a(n) = 3*n^3 + 3*n^2 + 1*n
/// https://oeis.org/A001020

pub struct A001020;

impl crate::traits::IntegerSequence for A001020 {
    const NAME: &str = "a(n) = 3*n^3 + 3*n^2 + 1*n";

    const HEAD: &[crate::Value] = &[
        0, 7, 38, 111, 244, 455, 762, 1183, 1736, 2439, 3310, 4367, 5628, 7111, 8834, 10815, 13072, 15623, 18486, 21679, 25220, 29127, 33418, 38111, 43224, 48775, 54782, 61263, 68236, 75719
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A001020";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        cubic_1020(n)
    }
}

const fn cubic_1020(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    3 * n * n * n + 3 * n * n + 1 * n
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A001020>();
}
