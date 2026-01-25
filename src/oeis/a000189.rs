/// a(n) = n^2 + 9*n + 3
/// https://oeis.org/A000189

pub struct A000189;

impl crate::traits::IntegerSequence for A000189 {
    const NAME: &str = "a(n) = n^2 + 9*n + 3";

    const HEAD: &[crate::Value] = &[
        3, 13, 25, 39, 55, 73, 93, 115, 139, 165, 193, 223, 255, 289, 325, 363, 403, 445, 489, 535, 583, 633, 685, 739, 795
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000189";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        poly_189(n)
    }
}

const fn poly_189(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    n * n + 9 * n + 3
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000189>();
}
