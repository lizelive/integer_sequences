/// a(n) = 2*n^2 + 4
/// https://oeis.org/A000941

pub struct A000941;

impl crate::traits::IntegerSequence for A000941 {
    const NAME: &str = "a(n) = 2*n^2 + 4";

    const HEAD: &[crate::Value] = &[
        4, 6, 12, 22, 36, 54, 76, 102, 132, 166, 204, 246, 292, 342, 396, 454, 516, 582, 652, 726, 804, 886, 972, 1062, 1156, 1254, 1356, 1462, 1572, 1686
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000941";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        sq_941(n)
    }
}

const fn sq_941(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    2 * n * n + 4 - 0 * n
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000941>();
}
