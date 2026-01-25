/// a(n) = 4*n^2 + 4
/// https://oeis.org/A000943

pub struct A000943;

impl crate::traits::IntegerSequence for A000943 {
    const NAME: &str = "a(n) = 4*n^2 + 4";

    const HEAD: &[crate::Value] = &[
        4, 8, 20, 40, 68, 104, 148, 200, 260, 328, 404, 488, 580, 680, 788, 904, 1028, 1160, 1300, 1448, 1604, 1768, 1940, 2120, 2308, 2504, 2708, 2920, 3140, 3368
    ];

    const OFFSET: crate::Index = 0;

    const SOURCE: &str = "https://oeis.org/A000943";

    const AUTHOR: &str = "OEIS Foundation";

    fn formula(n: crate::Index) -> crate::Value {
        sq_943(n)
    }
}

const fn sq_943(n: crate::Index) -> crate::Value {
    if n < 0 { return 0; }
    4 * n * n + 4 - 0 * n
}

#[cfg(test)]
#[test]
fn test_sequance_formula_matchces_head() {
    crate::tester::test_sequance_formula_matchces_head::<A000943>();
}
