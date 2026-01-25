/// a(n) = n^3 + 0*n^2 + 3*n + 0
/// https://oeis.org/A000215

pub struct A000215;

impl crate::traits::IntegerSequence for A000215 {
    const NAME: &str = "a(n) = n^3 + 0*n^2 + 3*n + 0";

    const HEAD: &[crate::Value] = &[
        0, 4, 14, 36, 76, 140, 234, 364, 536, 756, 1030, 1364, 1764, 2236, 2786, 3420, 4144, 4964, 5886, 6916, 8060, 9324, 10714, 12236, 13896
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000215";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        poly_215(n)
    }
}

const fn poly_215(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    n * n * n + 0 * n * n + 3 * n + 0
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000215>();
}
