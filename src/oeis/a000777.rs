/// a(n) = 4*n^2 + 3*n + 0
/// https://oeis.org/A000777

pub struct A000777;

impl crate::traits::IntegerSequence for A000777 {
    const NAME: &str = "a(n) = 4*n^2 + 3*n + 0";

    const HEAD: &[crate::Value] = &[
        0, 7, 22, 45, 76, 115, 162, 217, 280, 351, 430, 517, 612, 715, 826, 945, 1072, 1207, 1350, 1501, 1660, 1827, 2002, 2185, 2376, 2575, 2782, 2997, 3220, 3451
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000777";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        quad_777(n)
    }
}

const fn quad_777(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    4 * n * n + 3 * n + 0
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000777>();
}
