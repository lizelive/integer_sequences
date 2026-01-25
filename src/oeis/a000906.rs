/// a(n) = 7*n^2 + 0
/// https://oeis.org/A000906

pub struct A000906;

impl crate::traits::IntegerSequence for A000906 {
    const NAME: &str = "a(n) = 7*n^2 + 0";

    const HEAD: &[crate::Value] = &[
        0, 7, 28, 63, 112, 175, 252, 343, 448, 567, 700, 847, 1008, 1183, 1372, 1575, 1792, 2023, 2268, 2527, 2800, 3087, 3388, 3703, 4032, 4375, 4732, 5103, 5488, 5887
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000906";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        sq_906(n)
    }
}

const fn sq_906(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    7 * n * n + 0 - 0 * n
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000906>();
}
