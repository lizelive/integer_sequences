/// a(n) = n^3 + 3*n + 0
/// https://oeis.org/A000503

pub struct A000503;

impl crate::traits::IntegerSequence for A000503 {
    const NAME: &str = "a(n) = n^3 + 3*n + 0";

    const HEAD: &[crate::Value] = &[
        0, 4, 14, 36, 76, 140, 234, 364, 536, 756, 1030, 1364, 1764, 2236, 2786, 3420, 4144, 4964, 5886, 6916, 8060, 9324, 10714, 12236, 13896, 15700, 17654, 19764, 22036, 24476
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000503";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        poly_503(n)
    }
}

const fn poly_503(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    n * n * n + 3 * n + 0
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000503>();
}
