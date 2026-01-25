/// a(n) = n^3 + 1*n + 9
/// https://oeis.org/A000591

pub struct A000591;

impl crate::traits::IntegerSequence for A000591 {
    const NAME: &str = "a(n) = n^3 + 1*n + 9";

    const HEAD: &[crate::Value] = &[
        9, 11, 19, 39, 77, 139, 231, 359, 529, 747, 1019, 1351, 1749, 2219, 2767, 3399, 4121, 4939, 5859, 6887, 8029, 9291, 10679, 12199, 13857, 15659, 17611, 19719, 21989, 24427
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000591";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        poly_591(n)
    }
}

const fn poly_591(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    n * n * n + 1 * n + 9
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000591>();
}
